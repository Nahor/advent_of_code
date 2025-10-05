use common::error::AdventError;
use winnow::{
    ascii::line_ending,
    combinator::{alt, preceded, repeat, separated, terminated, trace},
    prelude::*,
};

pub enum Schematic {
    Lock([u8; 5]),
    Key([u8; 5]),
}

pub fn parse(content: &[u8]) -> Result<Vec<Schematic>, AdventError> {
    Ok(trace("parser", separated(2.., parse_schematic, line_ending)).parse(content)?)
}

fn parse_schematic(input: &mut &[u8]) -> PResult<Schematic> {
    trace(
        "parse_schematic",
        alt((
            preceded(("#####", line_ending), parse_lock),
            preceded((".....", line_ending), parse_key),
        )),
    )
    .parse_next(input)
}

fn parse_lock(input: &mut &[u8]) -> PResult<Schematic> {
    trace(
        "parse_lock",
        repeat(1.., parse_line)
            .fold(
                || [0_u8; 5],
                |mut acc, line| {
                    assert!(line.len() == 5);
                    (0..5).for_each(|i| acc[i] += line[i]);
                    acc
                },
            )
            .map(Schematic::Lock),
    )
    .parse_next(input)
}

fn parse_key(input: &mut &[u8]) -> PResult<Schematic> {
    trace(
        "parse_key",
        repeat(1.., parse_line)
            .fold(
                || [0_u8; 5],
                |mut acc, line| {
                    assert!(line.len() == 5);
                    (0..5).for_each(|i| acc[i] += line[i]);
                    acc
                },
            )
            .map(Schematic::Key),
    )
    .parse_next(input)
}

fn parse_line(input: &mut &[u8]) -> PResult<Vec<u8>> {
    trace(
        "parse_line",
        terminated(
            repeat(5..6, alt((b'#'.value(1), b'.'.value(0)))),
            line_ending,
        ),
    )
    .parse_next(input)
}
