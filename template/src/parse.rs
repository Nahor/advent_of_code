use common::error::AdventError;
use winnow::{
    ascii::line_ending,
    combinator::{opt, repeat, rest, terminated, trace},
    prelude::*,
};

pub fn parse(content: &str) -> Result<Vec<()>, AdventError> {
    Ok(trace(
        "parser",
        repeat(1.., terminated(parse_line, opt(line_ending))),
    )
    .parse(content)?)
}

fn parse_line(input: &mut &str) -> PResult<()> {
    trace("parse_line", rest.void()).parse_next(input)
}
