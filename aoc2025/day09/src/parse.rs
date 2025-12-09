use common::error::AdventError;
use glam::{DVec2, I64Vec2};
use winnow::{
    ascii::{dec_int, line_ending},
    combinator::{alt, eof, repeat_till, separated_pair, terminated, trace},
    prelude::*,
};

pub fn parse(content: &[u8]) -> Result<Vec<DVec2>, AdventError> {
    Ok(trace(
        "parser",
        repeat_till(1.., terminated(parse_line, alt((line_ending, eof))), eof).map(|(v, _)| v),
    )
    .parse(content)?)
}

fn parse_line(input: &mut &[u8]) -> ModalResult<DVec2> {
    trace(
        "parse_line",
        separated_pair(dec_int, b',', dec_int)
            .map(|(v1, v2): (i64, i64)| DVec2::new(v1 as f64, v2 as f64)), // technically, this consumes everything until eof, not just the line
    )
    .parse_next(input)
}

pub mod i64 {
    use super::*;

    pub fn parse(content: &[u8]) -> Result<Vec<I64Vec2>, AdventError> {
        Ok(trace(
            "parser",
            repeat_till(1.., terminated(parse_line, alt((line_ending, eof))), eof).map(|(v, _)| v),
        )
        .parse(content)?)
    }

    fn parse_line(input: &mut &[u8]) -> ModalResult<I64Vec2> {
        trace(
            "parse_line",
            separated_pair(dec_int, b',', dec_int).map(|(v1, v2): (i64, i64)| I64Vec2::new(v1, v2)), // technically, this consumes everything until eof, not just the line
        )
        .parse_next(input)
    }
}
