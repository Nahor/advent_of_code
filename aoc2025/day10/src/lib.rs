pub mod parse;
pub mod part1;
pub mod part1_bitvec;
pub mod part2;
pub mod part2_good_lp;
pub mod part2_z3;

pub mod int {
    #[derive(Debug)]
    pub struct Machine {
        pub lights: u32,
        pub buttons: Vec<u32>,
        pub joltage: Vec<u32>,
    }
}

pub mod bitvec {
    use bitvec::prelude::*;

    pub type MachineStorage = BitArray<u32, LocalBits>;

    #[derive(Debug)]
    pub struct Machine {
        pub lights: MachineStorage,
        pub buttons: Vec<MachineStorage>,
        pub joltage: Vec<u32>,
    }
}
