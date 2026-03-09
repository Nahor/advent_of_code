use common::error::AdventError;
use winnow::{
    ascii::line_ending,
    combinator::{alt, eof, repeat, repeat_till, terminated, trace},
    prelude::*,
};

use crate::{Cell, Grid};

pub fn parse(content: &[u8]) -> Result<Grid, AdventError> {
    Ok(trace(
        "parser",
        repeat_till(0.., terminated(parse_line, alt((line_ending, eof))), eof).map(|(v, _)| v),
        //repeat_till(0.., terminated(parse_line, opt(line_ending)), eof).map(|(v, _)| v),
    )
    .parse(content)?)
}

fn parse_line(input: &mut &[u8]) -> ModalResult<Vec<Cell>> {
    trace(
        "parse_line",
        repeat(1.., alt((b'@'.value(Cell::Roll), b'.'.value(Cell::Empty)))), // technically, this consumes everything until eof, not just the line
    )
    .parse_next(input)
}
