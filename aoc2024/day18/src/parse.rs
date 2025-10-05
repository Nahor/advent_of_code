use std::ops::Add;

use common::{error::AdventError, position::Direction};
use winnow::{
    ascii::{dec_int, line_ending},
    combinator::{opt, repeat, separated_pair, terminated, trace},
    prelude::*,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}
impl Add<Direction> for Coord {
    type Output = Coord;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Up => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

pub fn parse(content: &[u8]) -> Result<Vec<Coord>, AdventError> {
    Ok(trace(
        "parser",
        repeat(1.., terminated(parse_position, opt(line_ending))),
    )
    .parse(content)?)
}

fn parse_position(input: &mut &[u8]) -> PResult<Coord> {
    trace(
        "parse_position",
        separated_pair(dec_int, ',', dec_int).map(|(x, y)| Coord { x, y }), // technically, this consumes everything until eof, not just the line
    )
    .parse_next(input)
}
