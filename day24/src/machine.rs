use std::fmt::Debug;

use std::fmt::Display;
use std::mem::swap;
use std::num::NonZeroU16;

use rustc_hash::FxHashMap;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Wire(NonZeroU16); // No wire starts with a digit, so `000` is not used

impl Wire {
    pub const X_START: Self = Self::new(((b'x' - b'a') as u16 + 10) * (36 * 36));
    pub const Y_START: Self = Self::new(((b'y' - b'a') as u16 + 10) * (36 * 36));
    pub const Z_START: Self = Self::new(((b'z' - b'a') as u16 + 10) * (36 * 36));
    pub const IO_MASK: u16 = 36 * 36 - 1;

    pub const fn new(n: u16) -> Self {
        Self(NonZeroU16::new(n).unwrap())
    }

    pub fn get_x(n: u16) -> Self {
        Self::get_io(Self::X_START, n)
    }

    pub fn get_y(n: u16) -> Self {
        Self::get_io(Self::Y_START, n)
    }

    pub fn get_z(n: u16) -> Self {
        Self::get_io(Self::Z_START, n)
    }

    pub fn get_io(base: Wire, n: u16) -> Self {
        assert!((0..100).contains(&n));
        Self::new(base.0.get() + (n / 10) * 36 + (n % 10))
    }

    pub fn is_input(&self) -> Option<u16> {
        (Self::X_START.0..Self::Z_START.0)
            .contains(&self.0)
            .then_some(self.0.get() & Self::IO_MASK)
    }

    pub fn is_output(&self) -> Option<u16> {
        (Self::Z_START.0..)
            .contains(&self.0)
            .then_some(self.0.get() & Self::IO_MASK)
    }
}

impl Display for Wire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        static DIGITS: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";
        let w = self.0.get();
        let c1 = DIGITS[((w / (36 * 36)) % 36) as usize];
        let c2 = DIGITS[((w / 36) % 36) as usize];
        let c3 = DIGITS[(w % 36) as usize];
        let s = [c1, c2, c3];
        write!(f, "{}", unsafe { std::str::from_utf8_unchecked(&s) })
    }
}

impl Debug for Wire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operator {
    And,
    Or,
    Xor,
    Not,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Gate {
    pub operator: Operator,
    pub in1: Wire,
    pub in2: Wire,
    pub out: Wire,
    pub last_val: Option<bool>,
}

impl Gate {
    pub fn has_inputs(&self, w1: Wire, w2: Wire) -> bool {
        [(self.in1, self.in2), (self.in2, self.in1)].contains(&(w1, w2))
    }
}

#[derive(Debug, Clone)]
pub struct Machine {
    pub init: FxHashMap<Wire, bool>,
    pub inputs: FxHashMap<Wire, bool>,
    pub outputs: FxHashMap<Wire, bool>,
    pub gates: Vec<Gate>,

    // cache of useful values
    pub free_wires: Vec<Wire>,
    pub i_bits: u16,
}

impl Machine {
    pub fn new(init: FxHashMap<Wire, bool>, gates: Vec<Gate>) -> Self {
        // Validate the input
        let x_bits = init
            .keys()
            .filter(|wire| (Wire::X_START..Wire::Y_START).contains(wire))
            .count() as u16;
        let y_bits = init
            .keys()
            .filter(|wire| (Wire::Y_START..Wire::Z_START).contains(wire))
            .count() as u16;
        let z_bits = gates
            .iter()
            .map(|gate| gate.out)
            .filter(|wire| *wire >= Wire::Z_START)
            .count() as u16;
        assert_eq!(
            x_bits, y_bits,
            "x and y should have the same number of bits (x:{x_bits},y:{y_bits}"
        );
        assert_eq!(
            x_bits + 1,
            z_bits,
            "z should have one more bit: {z_bits} != {x_bits} + 1"
        );

        // // Find some unused wires that we can use later when modifying the machine
        // // Start at 1xx to avoid a 0xx
        // let free_wires = (1..Wire::X_START.0.get())
        //     .map(Wire::new)
        //     .filter(|&wire| {
        //         gates
        //             .iter()
        //             .all(|gate| gate.in1 != wire && gate.in2 != wire)
        //     })
        //     .take(8)
        //     .collect::<Vec<_>>();
        // if free_wires.len() < 8 {
        //     panic!("expected at least 8 free wires");
        // }

        // Create the machine
        Self {
            init,
            inputs: Default::default(),
            outputs: Default::default(),
            gates,
            free_wires: vec![],
            i_bits: x_bits,
        }
    }

    pub fn add(&mut self, x: u64, y: u64) -> (u64, u64, u64) {
        // Set init
        self.init.clear();
        let masked_x = self.extend_init(Wire::X_START, x, self.i_bits);
        let masked_y = self.extend_init(Wire::Y_START, y, self.i_bits);

        // Clear other data
        self.inputs.clear();
        self.outputs.clear();
        self.gates.iter_mut().for_each(|g| {
            g.last_val = None;
        });

        while self.tick() {}

        (masked_x, masked_y, self.get_output())
    }

    pub fn execute(&mut self) -> u64 {
        self.inputs.clear();
        self.outputs.clear();
        while self.tick() {}
        self.get_output()
    }

    fn tick(&mut self) -> bool {
        // The output of the previous tick become the input of the next tick,
        // unless it's the first run, in which case the input is the init value
        if self.outputs.is_empty() {
            self.inputs.clone_from(&self.init);
        } else {
            swap(&mut self.inputs, &mut self.outputs);
        }
        self.outputs.clone_from(&self.init);

        let mut changed = false;
        self.gates.iter_mut().for_each(|gate| {
            let Gate {
                operator: op_type,
                in1,
                in2,
                out,
                ref mut last_val,
            } = gate;
            let Some(&in1) = self.inputs.get(in1) else {
                return;
            };
            let Some(&in2) = self.inputs.get(in2) else {
                return;
            };
            let v = match op_type {
                Operator::And => in1 & in2,
                Operator::Or => in1 | in2,
                Operator::Xor => in1 ^ in2,
                Operator::Not => !in1,
            };
            if self.outputs.insert(*out, v).is_some() {
                panic!("output {out} already set");
            }
            if Some(v) != *last_val {
                changed = true;
                *last_val = Some(v);
            }
        });

        changed
    }

    fn extend_init(&mut self, base_wire: Wire, value: u64, max: u16) -> u64 {
        (0..max).for_each(|n| {
            let wire = Wire::new(base_wire.0.get() + (n / 10) * 36 + (n % 10));
            let wire_val = (value >> max & 1) == 1;
            self.init.insert(wire, wire_val);
        });
        value & ((1 << max) - 1)
    }

    fn get_output(&self) -> u64 {
        (0..(self.i_bits + 1)).rev().fold(0, |acc, bit| {
            acc << 1
                | if *self
                    .outputs
                    .get(&Wire::get_z(bit))
                    .unwrap_or_else(|| panic!("expected to find wire z{bit}: {}", Wire::get_z(bit)))
                {
                    1
                } else {
                    0
                }
        })
        // self.outputs
        //     .iter()
        //     .map(|(wire, value)| (*wire, *value))
        //     .filter(|(wire, _)| *wire >= Wire::Z_START)
        //     .sorted_by(|(wire1, _), (wire2, _)| wire1.cmp(wire2).reverse())
        //     .fold(0, |acc, (_, v)| acc << 1 | if v { 1 } else { 0 })
    }
}
