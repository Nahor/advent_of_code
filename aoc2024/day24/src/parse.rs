use common::error::AdventError;
use rustc_hash::FxHashMap;
use winnow::{
    ascii::{line_ending, space1},
    combinator::{alt, repeat, separated_pair, terminated, trace},
    prelude::*,
    token::one_of,
};

use crate::machine::{Gate, Machine, Operator, Wire};

pub fn parse(content: &[u8]) -> Result<Machine, AdventError> {
    Ok(trace(
        "parser",
        separated_pair(parse_inputs, line_ending, parse_gates)
            .map(|(inputs, ops)| Machine::new(inputs, ops)),
    )
    .parse(content)?)
}

fn parse_inputs(input: &mut &[u8]) -> PResult<FxHashMap<Wire, bool>> {
    trace(
        "parse_inputs",
        repeat(
            1..,
            terminated(
                separated_pair(parse_wire_name, b": ", parse_bool),
                line_ending,
            ),
        ),
    )
    .parse_next(input)
}

fn parse_gates(input: &mut &[u8]) -> PResult<Vec<Gate>> {
    trace(
        "parse_gates",
        repeat(1.., terminated(parse_gate, line_ending)),
    )
    .parse_next(input)
}

fn parse_gate(input: &mut &[u8]) -> PResult<Gate> {
    trace(
        "parse_gate",
        (
            parse_wire_name,
            space1,
            alt((
                b"AND".value(Operator::And),
                b"OR".value(Operator::Or),
                b"XOR".value(Operator::Xor),
            )),
            space1,
            parse_wire_name,
            (space1, b"->", space1),
            parse_wire_name,
        )
            .map(|(in1, _, operator, _, in2, _, out)| Gate {
                operator,
                in1,
                in2,
                out,
                last_val: None,
            }),
    )
    .parse_next(input)
}

fn parse_wire_name(input: &mut &[u8]) -> PResult<Wire> {
    trace(
        "parse_wire_name",
        (parse_wire_digit, parse_wire_digit, parse_wire_digit)
            .map(|(d3, d2, d1)| Wire::new(d3 * 36 * 36 + d2 * 36 + d1)),
    )
    .parse_next(input)
}

fn parse_wire_digit(input: &mut &[u8]) -> PResult<u16> {
    trace(
        "parse_wire_digit",
        alt((
            one_of(b'a'..=b'z').map(|b| (b - b'a') as u16 + 10),
            one_of(b'0'..=b'9').map(|b| (b - b'0') as u16),
        )),
    )
    .parse_next(input)
}

fn parse_bool(input: &mut &[u8]) -> PResult<bool> {
    trace("parse_bool", alt(("1".value(true), "0".value(false)))).parse_next(input)
}
