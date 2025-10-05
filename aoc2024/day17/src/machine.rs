use std::fmt::Debug;

use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

#[derive(Clone)]
pub struct Machine {
    pub(crate) instructions: Vec<usize>,
    pub(crate) reg_a: usize,
    pub(crate) reg_b: usize,
    pub(crate) reg_c: usize,
    pub(crate) reg_pc: usize,
    pub(crate) out: Vec<usize>,
}

impl Machine {
    pub fn new(instructions: Vec<usize>, reg_a: usize, reg_b: usize, reg_c: usize) -> Self {
        Self {
            instructions,
            reg_a,
            reg_b,
            reg_c,
            reg_pc: 0,
            out: Vec::new(),
        }
    }

    pub fn program(&self) -> &Vec<usize> {
        &self.instructions
    }

    pub fn output(&self) -> String {
        self.out
            .iter()
            .map(|v| format!("{v}").to_string())
            .collect::<Vec<_>>()
            .join(",")
    }

    pub fn exec(&mut self) {
        while self.exec_single() {}
    }

    pub fn fix_exec(&mut self) -> Option<usize> {
        (0..usize::MAX)
            .into_par_iter()
            .with_min_len(1_000_000)
            .by_exponential_blocks()
            .find_first(|&v| {
                let mut machine = self.clone();
                // Reset the registers and output
                machine.reg_a = v;
                // println!("Testing {self:?}");

                machine.exec_till_self()
            })
    }

    pub fn exec_till_self(&mut self) -> bool {
        let mut last_len = 0;
        while self.exec_single() {
            let out_len = self.out.len();
            if out_len > last_len {
                // new output, verify we are still valid
                if out_len > self.instructions.len() {
                    // Too much output
                    // println!("too much output");
                    return false;
                }
                if self.out[out_len - 1] != self.instructions[out_len - 1] {
                    // No match
                    // println!(
                    //     "wrong output @{}: {} (expected {})",
                    //     out_len - 1,
                    //     self.out[out_len - 1],
                    //     self.instructions[out_len - 1]
                    // );
                    return false;
                }
            }
            last_len = out_len;
        }
        self.out == self.instructions
    }

    pub fn exec_single(&mut self) -> bool {
        let Some(&[instruction, operand]) = self.next_instruction() else {
            return false;
        };
        match instruction {
            0 => self.exec_adv(operand),
            1 => self.exec_bxl(operand),
            2 => self.exec_bst(operand),
            3 => self.exec_jnz(operand),
            4 => self.exec_bxc(operand),
            5 => self.exec_out(operand),
            6 => self.exec_bdv(operand),
            7 => self.exec_cdv(operand),
            _ => panic!("invalid instruction {instruction}"),
        };
        true
    }

    fn next_instruction(&self) -> Option<&[usize; 2]> {
        if self.reg_pc + 2 > self.instructions.len() {
            // println!("invalid PC {}", self.reg_pc);
            None
        } else {
            self.instructions[self.reg_pc..self.reg_pc + 2]
                .try_into()
                .ok()
        }
    }

    fn exec_adv(&mut self, operand: usize) {
        // "... / 2^n" is the same a ">>n"
        self.reg_a >>= self.combo_operand(operand);
        self.reg_pc += 2;
    }

    fn exec_bxl(&mut self, operand: usize) {
        self.reg_b ^= operand;
        self.reg_pc += 2;
    }

    fn exec_bst(&mut self, operand: usize) {
        self.reg_b = self.combo_operand(operand) % 8;
        self.reg_pc += 2;
    }

    fn exec_jnz(&mut self, operand: usize) {
        if self.reg_a != 0 {
            self.reg_pc = operand
        } else {
            self.reg_pc += 2;
        }
    }

    fn exec_bxc(&mut self, _operand: usize) {
        self.reg_b ^= self.reg_c;
        self.reg_pc += 2;
    }

    fn exec_out(&mut self, operand: usize) {
        self.out.push(self.combo_operand(operand) % 8);
        self.reg_pc += 2;
    }

    fn exec_bdv(&mut self, operand: usize) {
        // "... / 2^n" is the same a ">>n"
        self.reg_b = self.reg_a >> self.combo_operand(operand);
        self.reg_pc += 2;
    }

    fn exec_cdv(&mut self, operand: usize) {
        // "... / 2^n" is the same a ">>n"
        self.reg_c = self.reg_a >> self.combo_operand(operand);
        self.reg_pc += 2;
    }

    fn combo_operand(&self, operand: usize) -> usize {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("invalid operand {operand}"),
        }
    }
}

impl Debug for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Machine [ a:{}, b:{}, c:{}, pc:{}, inst: ",
            self.reg_a, self.reg_b, self.reg_c, self.reg_pc
        )?;
        if !self.instructions.is_empty() {
            let mut i = self.instructions.iter();
            write!(f, "{}", i.next().unwrap())?;
            i.try_for_each(|i| write!(f, ",{i}"))?;
        }
        write!(f, " ]")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex_instruction_1() {
        let mut machine = Machine::new(vec![2, 6], 0, 0, 9);
        machine.exec();
        assert_eq!(machine.reg_b, 1);
    }

    #[test]
    fn ex_instruction_2() {
        let mut machine = Machine::new(vec![5, 0, 5, 1, 5, 4], 10, 0, 0);
        machine.exec();
        assert_eq!(machine.output(), "0,1,2");
    }

    #[test]
    fn ex_instruction_3() {
        let mut machine = Machine::new(vec![0, 1, 5, 4, 3, 0], 2024, 0, 0);
        machine.exec();
        assert_eq!(machine.output(), "4,2,5,6,7,7,7,7,3,1,0");
        assert_eq!(machine.reg_a, 0);
    }

    #[test]
    fn ex_instruction_4() {
        let mut machine = Machine::new(vec![1, 7], 0, 29, 0);
        machine.exec();
        assert_eq!(machine.reg_b, 26);
    }

    #[test]
    fn ex_instruction_5() {
        let mut machine = Machine::new(vec![4, 0], 0, 2024, 43690);
        machine.exec();
        assert_eq!(machine.reg_b, 44354);
    }

    #[test]
    fn partial() {
        let mut machine = Machine::new(
            vec![2, 4, 1, 1, 7, 5, 4, 6, 0, 3, 1, 4, 5, 5],
            28066687,
            0,
            0,
        );
        machine.exec();
        assert_eq!(dbg!(machine).reg_b % 8, 7);
    }
}
