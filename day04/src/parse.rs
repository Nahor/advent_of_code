use winnow::{
    ascii::line_ending,
    combinator::{opt, repeat, terminated, trace},
    prelude::*,
    token::one_of,
};

use crate::error::AdventError;

pub fn parse(content: &str) -> Result<Vec<Vec<char>>, AdventError> {
    Ok(trace(
        "parser",
        repeat(1.., terminated(parse_line, opt(line_ending))),
    )
    .parse(content)?)
}

fn parse_line(input: &mut &str) -> PResult<Vec<char>> {
    trace("parse_line", repeat(1.., one_of(['X', 'M', 'A', 'S']))).parse_next(input)
}
