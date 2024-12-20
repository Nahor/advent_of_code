use common::error::AdventError;
use winnow::{
    ascii::line_ending,
    combinator::{separated, seq, terminated, trace},
    prelude::*,
    token::take_while,
};

pub struct Data<'a> {
    pub patterns: Vec<&'a [u8]>,
    pub designs: Vec<&'a [u8]>,
}

pub fn parse(content: &[u8]) -> Result<Data, AdventError> {
    Ok(trace(
        "parser",
        seq! {Data{
            patterns: parse_patterns,
            _: line_ending,
            designs: parse_designs
        }}, //(parse_patterns, line_ending, parse_designs),
    )
    .parse(content)?)
}

fn parse_patterns<'a>(input: &mut &'a [u8]) -> PResult<Vec<&'a [u8]>> {
    trace(
        "parse_patterns",
        terminated(separated(1.., parse_colors, ", "), line_ending),
    )
    .parse_next(input)
}

fn parse_designs<'a>(input: &mut &'a [u8]) -> PResult<Vec<&'a [u8]>> {
    trace(
        "parse_designs",
        terminated(separated(1.., parse_colors, line_ending), line_ending),
    )
    .parse_next(input)
}

fn parse_colors<'a>(input: &mut &'a [u8]) -> PResult<&'a [u8]> {
    trace(
        "parse_colors",
        //repeat::<_, _, (), _, _>(1.., one_of([b'w', b'u', b'b', b'r', b'g'])).take(),
        take_while(1.., b"wubrg"),
        //one_of([b'w', b'u', b'b', b'r', b'g']).take(),
    )
    .parse_next(input)
}
