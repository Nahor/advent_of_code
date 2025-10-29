#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Op {
    Mul(u64, u64),
    Dont,
    Do,
}
