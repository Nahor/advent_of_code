// This is incomplete. I've only added rules until I got the 8 wires (4 pairs)
// and passed puzzle
use std::collections::HashSet;

use itertools::{Either, Itertools};
use miette::Result;

use crate::{
    machine::{Gate, Operator, Wire},
    parse::parse,
};

struct HalfAdderStage1 {
    pub _n: u16, // First-stage are necessarily Xn/Yn, order doesn't matter
    pub s: Wire,
    pub c: Wire,
}

struct HalfAdderStage2 {
    pub a: Wire,
    pub b: Wire,
    pub s: Wire, // Should be Zn, but it might have been swapped
    pub c: Wire,
}

pub fn run(content: &[u8]) -> Result<String> {
    let machine = parse(content)?;

    // let op_max = machine.ops.len();
    // let mut rng = thread_rng();

    // This is an adder. Typically, an adder, is a series of full-adders.
    // A full-adder is two half-adders.
    //
    // A half-adder has 2 inputs (A/B)  and 2 outputs (S/C). On one side,
    // the two inputs are combined with a XOR to create an output S. On
    // the other, the two inputs are combined with an AND to create an output C.
    //
    // A full-adder has 3 inputs (A, B, C_in) and 2 outputs. The first
    // half-adder takes 2 of the input and produces S_tmp and C_tmp1. The second
    // half-adder takes the third input and S_tmp and produces S and C_tmp2.
    // Then the two C_tmp are combined into a single output C_out.
    //
    // In a full-adder, there are multiple ways to consume the A/B/C_in inputs.
    // The first combines A/B to produce S_tmp/C_tmp1, then S_tmp and C_in are
    // combined into S/C_tmp2.
    // The other ways combine A, or B, with C, which is then combined with B,
    // or A.
    //
    // I believe the first is the most common because then all the As/Bs can be
    // combined in parallel, then the Cs can be combined in series, for an
    // overall cost of O(n).
    // The others requires all the half-adders to be in series, for an overall
    // cost of O(2n)
    //                             C_tmp1
    //                           ┌─────────────────►┌──┐
    //     C_in ─────────────────│─┬─►┌───┐         │OR├► C_out
    //            ┌─►┌───┐       │ │  │AND├────────►└──┘
    //        A ──┤  │AND├───────┘┌│─►└───┘ C_tmp2
    //           ┌│─►└───┘        │└─►┌───┐
    //        B ─┤└─►┌───┐ S_tmp  │   │XOR├─────────────► S
    //           │   │XOR├────────┴──►└───┘
    //           └──►└───┘
    //          │    Half       │      Half      │
    //          │    Adder      │      Adder     │
    //
    // For multi-bits additions, X0/Y0 are combined with a half-adder, producing
    // Z0/C0, Z0 being the output bit and C0 being the carry.
    // Then X1/Y1/C0 are send to a full-adder to produce Z1/C1, then X2/Y2/C1
    // produce Z2/C2,... until Xn/Yy/C(n-1) produce Zn/Cn, where Cn becomes Z(n+1).
    //
    // Ignoring the special X0/Y0/Z0/C0:
    // - The output of an OR is always the carry
    // - ANDs and XORs in the same half-adder always share the same inputs
    //   => we can find the half-adders
    // - All the first-stage half-adders are connected to X/Y, which cannot be
    //   swapped
    //   => we can know all the first-stage half-adders
    //   => all the remaining half-adders must be second-stage
    // => All second-stage half-adders must have their Z/XOR output *should* be
    //   connected to a Z wire.
    // => All second-stage half-adders must have their C/AND output *should* be
    //   connected to a OR gate
    // => All OR gates output should connect to a second-stage half-adder
    // => A Xn/Yn first-stage half-adder should connect to a second-stage
    //    half-adders connected to a Zn wire.

    let mut swapped_output = HashSet::new();
    let z0_gate = machine
        .gates
        .iter()
        .find(|gate| {
            gate.operator == Operator::Xor && gate.has_inputs(Wire::get_x(0), Wire::get_y(0))
        })
        .unwrap();
    if z0_gate.out != Wire::get_z(0) {
        swapped_output.insert(z0_gate.out);
    }
    // println!(
    //     "After bad Z0: {:?}",
    //     swapped_output.iter().sorted().join(",")
    // );

    let (stage1, stage2) = get_half_adders(&machine.gates);

    // Find any stage2 where S is not a Zn
    swapped_output.extend(stage2.iter().filter_map(|adder| {
        if adder.s.is_output().is_some() {
            None
        } else {
            Some(adder.s)
        }
    }));
    // println!(
    //     "After bad adder->Z: {:?}",
    //     swapped_output.iter().sorted().join(",")
    // );

    // Reverse: find Z that is not a stage 2 S
    swapped_output.extend(
        (1..(machine.i_bits + 1)) // start at 1 because Z0 is special
            .map(Wire::get_z)
            .filter(|wire| stage2.iter().all(|adder| adder.s != *wire))
            .filter(|wire| {
                // Last Z is actually a carry so connected to a OR, not an adder
                *wire != Wire::get_z(machine.i_bits)
            }),
    );
    // println!(
    //     "After bad Z->adder: {:?}",
    //     swapped_output.iter().sorted().join(",")
    // );

    // Find the OR gates whose output is not going to a stage 2 adder
    swapped_output.extend(
        machine
            .gates
            .iter()
            .filter_map(|gate| (gate.operator == Operator::Or).then_some(gate.out))
            .filter(|gate_out| {
                stage2
                    .iter()
                    .all(|adder| ![adder.a, adder.b].contains(gate_out))
            })
            .filter(|gate_out| {
                // The last carry goes into the MSB
                *gate_out != Wire::get_z(machine.i_bits)
            }),
    );
    // println!(
    //     "After bad Or->adder.in: {:?}",
    //     swapped_output.iter().sorted().join(",")
    // );

    // Find any stage2 where C is not a OR gate
    swapped_output.extend(stage2.iter().map(|adder| adder.c).filter(|c_tmp2| {
        machine
            .gates
            .iter()
            .filter_map(|gate| (gate.operator == Operator::Or).then_some([gate.in1, gate.in2]))
            .all(|or_ins| !or_ins.contains(c_tmp2))
    }));
    // println!(
    //     "After bad stage2->Or: {:?}",
    //     swapped_output.iter().sorted().join(",")
    // );

    // Find any stage1 where C is not a OR gate
    swapped_output.extend(stage1.iter().map(|adder| adder.c).filter(|c_tmp2| {
        machine
            .gates
            .iter()
            .filter_map(|gate| (gate.operator == Operator::Or).then_some([gate.in1, gate.in2]))
            .all(|or_ins| !or_ins.contains(c_tmp2))
    }));
    // println!(
    //     "After bad stage1-(C)->Or: {:?}",
    //     swapped_output.iter().sorted().join(",")
    // );

    // Find any stage1 where S is not a stage 2e
    swapped_output.extend(stage1.iter().map(|adder| adder.s).filter(|s_tmp| {
        stage2
            .iter()
            .all(|adder| ![adder.a, adder.b].contains(s_tmp))
    }));
    // println!(
    //     "After bad stage1-(S)->stage2: {:?}",
    //     swapped_output.iter().sorted().join(",")
    // );

    // [...MORE RULES NEEDED...]

    Ok(swapped_output.iter().sorted().join(","))
}

fn get_half_adders(gates: &[Gate]) -> (Vec<HalfAdderStage1>, Vec<HalfAdderStage2>) {
    let (first_stage, second_stage): (Vec<_>, Vec<_>) = gates
        .iter()
        .filter(|g| g.operator == Operator::Xor)
        .filter(|g| ![g.in1, g.in2].contains(&Wire::get_x(0))) // Filter the X0/Y0 half-adder
        .map(|g1| {
            gates
                .iter()
                .find_map(|g2| {
                    // AND and XOR in the same half-adder always share the same
                    // inputs, and no other half-adder share them, so we only
                    // need to check one input to know if those gates are in
                    // the same half-adder. However, the order might be different
                    (g2.operator == Operator::And && [g1.in1, g1.in2].contains(&g2.in1))
                        .then_some((g1, g2))
                })
                .unwrap_or_else(|| panic!("should have found a AND gate matching {g1:?}"))
        })
        .partition_map(|(g1, g2)| {
            if let Some(n) = g1.in1.is_input() {
                Either::Left(HalfAdderStage1 {
                    _n: n,
                    s: g1.out,
                    c: g2.out,
                })
            } else {
                Either::Right(HalfAdderStage2 {
                    a: g1.in1,
                    b: g1.in2,
                    s: g1.out,
                    c: g2.out,
                })
            }
        });

    (first_stage, second_stage)
}
