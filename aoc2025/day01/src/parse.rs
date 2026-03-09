use common::error::AdventError;
use winnow::{
    ascii::{dec_int, line_ending},
    combinator::{alt, eof, opt, preceded, repeat_till, terminated, trace},
    prelude::*,
};

pub fn parse(content: &[u8]) -> Result<Vec<i32>, AdventError> {
    Ok(trace(
        "parser",
        repeat_till(0.., terminated(parse_line, opt(line_ending)), eof).map(|(v, _)| v),
    )
    .parse(content)?)
}

fn parse_line(input: &mut &[u8]) -> ModalResult<i32> {
    trace(
        "parse_line",
        alt((
            preceded('L', dec_int).map(|i: i32| -i),
            preceded('R', dec_int),
        )),
    )
    .parse_next(input)
}
