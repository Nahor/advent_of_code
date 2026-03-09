use common::error::AdventError;
use winnow::{
    BStr,
    ascii::{dec_uint, line_ending, space1},
    combinator::{alt, delimited, eof, repeat, separated, seq, terminated, trace},
    prelude::*,
};

pub mod int {

    use super::*;
    use crate::int::Machine;

    pub fn parse(content: &BStr) -> Result<Vec<Machine>, AdventError> {
        Ok(trace(
            "parser",
            terminated(
                repeat(1.., terminated(parse_line, alt((line_ending, eof)))),
                eof,
            ),
        )
        .parse(content)?)
    }

    fn parse_line(input: &mut &BStr) -> ModalResult<Machine> {
        trace(
            "parse_line",
            seq! {Machine{
                lights: parse_lights,
                _: space1,
                buttons: parse_buttons,
                _: space1,
                joltage: parse_joltage
            }},
        )
        .parse_next(input)
    }

    fn parse_lights(input: &mut &BStr) -> ModalResult<u32> {
        trace(
            "parse_lights",
            delimited(
                b'[',
                repeat(1.., alt((b'.'.value(false), b'#'.value(true)))).map(|v: Vec<_>| {
                    v.iter()
                        .enumerate()
                        .filter_map(|(bit, b)| b.then_some(bit))
                        .fold(0_u32, |acc, bit| acc | (1_u32 << bit))
                }),
                b']',
            ),
        )
        .parse_next(input)
    }

    fn parse_buttons(input: &mut &BStr) -> ModalResult<Vec<u32>> {
        trace("parse_buttons", separated(1.., parse_button, b' ')).parse_next(input)
    }

    fn parse_button(input: &mut &BStr) -> ModalResult<u32> {
        trace(
            "parse_button",
            delimited(
                b'(',
                separated(1.., dec_uint::<_, u32, _>, b',')
                    .map(|v: Vec<_>| v.into_iter().fold(0_u32, |acc, bit| acc | (1_u32 << bit))),
                b')',
            ),
        )
        .parse_next(input)
    }

    fn parse_joltage(input: &mut &BStr) -> ModalResult<Vec<u32>> {
        trace(
            "parse_joltage",
            delimited(b'{', separated(1.., dec_uint::<_, u32, _>, b','), b'}'),
        )
        .parse_next(input)
    }
}

pub mod bitvec {
    use crate::bitvec::Machine;
    use crate::bitvec::MachineStorage;

    use super::*;

    pub fn parse(content: &BStr) -> Result<Vec<Machine>, AdventError> {
        Ok(trace(
            "parser",
            terminated(
                repeat(1.., terminated(parse_line, alt((line_ending, eof)))),
                eof,
            ),
        )
        .parse(content)?)
    }

    fn parse_line(input: &mut &BStr) -> ModalResult<Machine> {
        trace("parse_line", move |input: &mut _| {
            let mut parser = seq! {Machine{
                lights: parse_lights,
                _: space1,
                buttons: parse_buttons,
                _: space1,
                joltage: parse_joltage
            }};
            let machine = parser.parse_next(input)?;
            Ok(machine)
        })
        .parse_next(input)
    }

    fn parse_lights(input: &mut &BStr) -> ModalResult<MachineStorage> {
        trace(
            "parse_lights",
            delimited(
                b'[',
                repeat(1.., alt((b'.'.value(false), b'#'.value(true)))).map(|v: Vec<_>| {
                    v.iter()
                        .enumerate()
                        .filter_map(|(bit, b)| b.then_some(bit))
                        .fold(MachineStorage::default(), |mut acc, bit| {
                            acc.set(bit, true);
                            acc
                        })
                }),
                b']',
            ),
        )
        .parse_next(input)
    }

    fn parse_buttons(input: &mut &BStr) -> ModalResult<Vec<MachineStorage>> {
        trace("parse_buttons", separated(1.., parse_button, b' ')).parse_next(input)
    }

    fn parse_button(input: &mut &BStr) -> ModalResult<MachineStorage> {
        trace(
            "parse_button",
            delimited(
                b'(',
                separated(1.., dec_uint::<_, usize, _>, b',').map(|v: Vec<_>| {
                    v.into_iter()
                        .fold(MachineStorage::default(), |mut acc, bit| {
                            acc.set(bit, true);
                            acc
                        })
                }),
                b')',
            ),
        )
        .parse_next(input)
    }

    fn parse_joltage(input: &mut &BStr) -> ModalResult<Vec<u32>> {
        trace(
            "parse_joltage",
            delimited(b'{', separated(1.., dec_uint::<_, u32, _>, b','), b'}'),
        )
        .parse_next(input)
    }
}
