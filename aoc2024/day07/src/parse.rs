use common::error::AdventError;
use winnow::{
    ascii::{dec_int, line_ending},
    combinator::{opt, repeat, separated, separated_pair, terminated, trace},
    prelude::*,
};

use crate::document::Equation;

pub fn parse(content: &[u8]) -> Result<Vec<Equation>, AdventError> {
    Ok(trace(
        "parser",
        repeat(1.., terminated(parse_equation, opt(line_ending))),
    )
    .parse(content)?)
}

fn parse_equation(input: &mut &[u8]) -> PResult<Equation> {
    trace(
        "parse_equation",
        separated_pair(dec_int, ": ", parse_terms)
            .map(|(result, terms)| Equation { result, terms }),
    )
    .parse_next(input)
}

fn parse_terms(input: &mut &[u8]) -> PResult<Vec<i64>> {
    trace("parse_terms", separated(2.., dec_int::<_, i64, _>, ' ')).parse_next(input)
}
