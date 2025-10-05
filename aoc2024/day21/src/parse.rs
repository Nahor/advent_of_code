use common::error::AdventError;
use winnow::{
    ascii::line_ending,
    combinator::{eof, opt, repeat, repeat_till, terminated, trace},
    prelude::*,
    token::one_of,
};

pub fn parse(content: &[u8]) -> Result<Vec<Vec<u8>>, AdventError> {
    Ok(trace(
        "parser",
        repeat_till(1.., terminated(parse_line, opt(line_ending)), eof).map(|(v, _)| v),
    )
    .parse(content)?)
}

fn parse_line(input: &mut &[u8]) -> PResult<Vec<u8>> {
    trace(
        "parse_line",
        repeat(1.., one_of((b'0'..=b'9', b'A'))), // technically, this consumes everything until eof, not just the line
    )
    .parse_next(input)
}
