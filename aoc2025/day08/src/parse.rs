use common::error::AdventError;
use glam::I64Vec3;
use winnow::{
    ascii::{dec_int, line_ending},
    combinator::{alt, eof, repeat_till, separated, terminated, trace},
    prelude::*,
};

pub fn parse(content: &[u8]) -> Result<Vec<I64Vec3>, AdventError> {
    Ok(trace(
        "parser",
        repeat_till(1.., terminated(parse_line, alt((line_ending, eof))), eof).map(|(v, _)| v),
    )
    .parse(content)?)
}

fn parse_line(input: &mut &[u8]) -> ModalResult<I64Vec3> {
    trace(
        "parse_line",
        separated(3..=3, dec_int::<_, i64, _>, b',').map(|v: Vec<_>| I64Vec3::from_slice(&v)), // technically, this consumes everything until eof, not just the line
    )
    .parse_next(input)
}
