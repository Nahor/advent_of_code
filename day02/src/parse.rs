use winnow::{
    ascii::{dec_int, line_ending, space1},
    combinator::{opt, repeat, separated, terminated, trace},
    prelude::*,
};

use crate::error::AdventError;

pub fn parse(content: &str) -> Result<Vec<Vec<i64>>, AdventError> {
    Ok(trace(
        "parser",
        repeat(1.., terminated(parse_line, opt(line_ending))),
    )
    .parse(content)?)
}

fn parse_line(input: &mut &str) -> PResult<Vec<i64>> {
    trace("parse_line", separated(1.., dec_int::<_, i64, _>, space1)).parse_next(input)
}
