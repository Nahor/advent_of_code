use common::error::AdventError;
use rustc_hash::FxHashMap;
use winnow::{
    ascii::{alpha1, line_ending, space0, space1},
    combinator::{alt, eof, repeat_till, separated, separated_pair, terminated, trace},
    prelude::*,
};

pub fn parse(content: &[u8]) -> Result<FxHashMap<&[u8], Vec<&[u8]>>, AdventError> {
    Ok(trace(
        "parser",
        repeat_till(1.., terminated(parse_line, alt((line_ending, eof))), eof).map(|(v, _)| v),
    )
    .parse(content)?)
}

fn parse_line<'a>(input: &mut &'a [u8]) -> ModalResult<(&'a [u8], Vec<&'a [u8]>)> {
    trace(
        "parse_line",
        separated_pair(alpha1, (b':', space0), parse_outputs), // technically, this consumes everything until eof, not just the line
    )
    .parse_next(input)
}

fn parse_outputs<'a>(input: &mut &'a [u8]) -> ModalResult<Vec<&'a [u8]>> {
    trace(
        "parse_outputs",
        separated(1.., alpha1, space1), // technically, this consumes everything until eof, not just the line
    )
    .parse_next(input)
}
