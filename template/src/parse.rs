use winnow::{
    ascii::line_ending,
    combinator::{opt, rest, separated, terminated, trace},
    prelude::*,
};

use crate::error::AdventError;

pub fn parse(content: &str) -> Result<Vec<()>, AdventError> {
    Ok(trace(
        "parser",
        terminated(separated(1.., parse_line, line_ending), opt(line_ending)),
    )
    .parse(content)?)
}

fn parse_line(input: &mut &str) -> PResult<()> {
    trace("parse_line", rest.void()).parse_next(input)
}
