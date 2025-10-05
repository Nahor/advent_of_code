use common::error::AdventError;
use winnow::{
    ascii::line_ending,
    combinator::{eof, opt, repeat_till, rest, terminated, trace},
    prelude::*,
};

pub fn parse(content: &[u8]) -> Result<Vec<()>, AdventError> {
    Ok(trace(
        "parser",
        repeat_till(0.., terminated(parse_line, opt(line_ending)), eof).map(|(v, _)| v),
    )
    .parse(content)?)
}

fn parse_line(input: &mut &[u8]) -> PResult<()> {
    trace(
        "parse_line",
        rest.void(), // technically, this consumes everything until eof, not just the line
    )
    .parse_next(input)
}
