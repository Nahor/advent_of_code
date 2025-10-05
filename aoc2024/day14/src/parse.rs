use common::error::AdventError;
use winnow::{
    ascii::{dec_int, line_ending},
    combinator::{opt, separated, separated_pair, seq, terminated, trace},
    prelude::*,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Robot {
    pub pos: (isize, isize),
    pub vel: (isize, isize),
}

pub fn parse(content: &[u8]) -> Result<Vec<Robot>, AdventError> {
    Ok(trace(
        "parser",
        terminated(separated(1.., parse_robot, line_ending), opt(line_ending)),
    )
    .parse(content)?)
}

fn parse_robot(input: &mut &[u8]) -> PResult<Robot> {
    trace(
        "parse_robot",
        seq! {
            Robot{
                _: "p=",
                pos: parse_coord,
                _: " v=",
                vel: parse_coord,
            }
        }, // technically, this consumes everything until eof, not just the line
    )
    .parse_next(input)
}

fn parse_coord(input: &mut &[u8]) -> PResult<(isize, isize)> {
    trace("parse_coord", separated_pair(dec_int, ',', dec_int)).parse_next(input)
}
