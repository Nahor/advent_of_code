use common::error::AdventError;
use winnow::{
    ascii::{digit1, line_ending},
    combinator::{eof, opt, separated, separated_pair, terminated, trace},
    prelude::*,
};

pub fn parse(content: &str) -> Result<Vec<(&str, &str)>, AdventError> {
    Ok(trace(
        "parser",
        (terminated(parse_line, opt(line_ending)), eof).map(|(v, _)| v),
    )
    .parse(content)?)
}

fn parse_line<'a>(input: &mut &'a str) -> ModalResult<Vec<(&'a str, &'a str)>> {
    trace(
        "parse_line",
        separated(.., parse_range, ','), // technically, this consumes everything until eof, not just the line
    )
    .parse_next(input)
}

fn parse_range<'a>(input: &mut &'a str) -> ModalResult<(&'a str, &'a str)> {
    trace(
        "parse_range",
        separated_pair(digit1, '-', digit1), // technically, this consumes everything until eof, not just the line
    )
    .parse_next(input)
}
