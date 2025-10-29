use common::position::Position;

#[derive(Debug, Default, Clone, Copy)]
pub struct Button {
    pub tokens: usize,
    pub claw_move: Position,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Machine {
    pub btn_a: Button,
    pub btn_b: Button,
    pub prize: Position,
}
