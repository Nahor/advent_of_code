use common::error::AdventError;
use winnow::{
    ascii::{dec_uint, line_ending},
    combinator::{repeat, terminated, trace},
    prelude::*,
};

pub fn parse(content: &[u8]) -> Result<Vec<u32>, AdventError> {
    Ok(trace("parser", repeat(1.., parse_line)).parse(content)?)
}

fn parse_line(input: &mut &[u8]) -> PResult<u32> {
    trace("parse_line", terminated(dec_uint, line_ending)).parse_next(input)
}
