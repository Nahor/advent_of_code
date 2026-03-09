use common::error::AdventError;
use winnow::{
    ascii::line_ending,
    combinator::{alt, eof, repeat_till, terminated, trace},
    prelude::*,
    token::rest,
};

pub fn parse(content: &[u8]) -> Result<Vec<()>, AdventError> {
    Ok(trace(
        "parser",
        repeat_till(1.., terminated(parse_line, alt((line_ending, eof))), eof).map(|(v, _)| v),
    )
    .parse(content)?)
}

fn parse_line(input: &mut &[u8]) -> ModalResult<()> {
    trace(
        "parse_line",
        rest.void(), // technically, this consumes everything until eof, not just the line
    )
    .parse_next(input)
}
