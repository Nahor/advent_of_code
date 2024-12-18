use common::error::AdventError;
use winnow::{
    ascii::{dec_uint, line_ending, newline},
    combinator::{delimited, opt, preceded, separated, separated_pair, terminated, trace},
    prelude::*,
};

use crate::machine::Machine;

pub fn parse(content: &[u8]) -> Result<Machine, AdventError> {
    Ok(trace(
        "parser",
        terminated(
            separated_pair(parse_registers, newline, parse_program)
                .map(|((a, b, c), prog)| Machine::new(prog, a, b, c)),
            opt(line_ending),
        ),
    )
    .parse(content)?)
}

fn parse_registers(input: &mut &[u8]) -> PResult<(usize, usize, usize)> {
    trace(
        "parse_registers",
        (
            delimited("Register A: ", dec_uint::<_, usize, _>, line_ending),
            delimited("Register B: ", dec_uint::<_, usize, _>, line_ending),
            delimited("Register C: ", dec_uint::<_, usize, _>, line_ending),
        ),
    )
    .parse_next(input)
}

fn parse_program(input: &mut &[u8]) -> PResult<Vec<usize>> {
    trace(
        "parse_program",
        preceded("Program: ", separated(2.., dec_uint::<_, usize, _>, ',')),
    )
    .parse_next(input)
}
