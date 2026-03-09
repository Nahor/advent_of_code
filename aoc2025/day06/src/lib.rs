pub mod parse;
pub mod part1;
pub mod part2;
pub mod part2_left_right;

#[derive(Debug, Clone, Copy)]
pub enum Ops {
    Add,
    Mul,
}
