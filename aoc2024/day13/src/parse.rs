use common::{error::AdventError, position::Position};
use winnow::{
    ascii::{dec_uint, line_ending},
    combinator::{empty, eof, opt, repeat_till, seq, terminated, trace},
    prelude::*,
};

use crate::machine::{Button, Machine};

pub fn parse(content: &[u8]) -> Result<Vec<Machine>, AdventError> {
    Ok(trace(
        "parser",
        repeat_till(1.., terminated(parse_machine, opt(line_ending)), eof).map(|(v, _)| v),
    )
    .parse(content)?)
}

fn parse_machine(input: &mut &[u8]) -> PResult<Machine> {
    trace(
        "parse_machine",
        (parse_button_a, parse_button_b, parse_prize).map(|(btn_a, btn_b, prize)| Machine {
            btn_a,
            btn_b,
            prize,
        }),
    )
    .parse_next(input)
}

fn parse_button_a(input: &mut &[u8]) -> PResult<Button> {
    trace(
        "parse_button_a",
        terminated(
            seq! {
                Button{
                    tokens: empty.value(3),
                    _: "Button A: ",
                    claw_move: parse_claw_move,
                }
            },
            line_ending,
        ),
    )
    .parse_next(input)
}

fn parse_button_b(input: &mut &[u8]) -> PResult<Button> {
    trace(
        "parse_button_B",
        terminated(
            seq! {
                Button{
                    tokens: empty.value(1),
                    _: "Button B: ".void(),
                    claw_move: parse_claw_move,
                }
            },
            line_ending,
        ),
    )
    .parse_next(input)
}

fn parse_claw_move(input: &mut &[u8]) -> PResult<Position> {
    trace(
        "parse_claw_move",
        seq! {
            Position {
                _: "X+".void(),
                x: dec_uint,
                _: ", Y+",
                y: dec_uint,
            }
        },
    )
    .parse_next(input)
}

fn parse_prize(input: &mut &[u8]) -> PResult<Position> {
    trace(
        "parse_prize",
        seq! {
            Position{
                _: "Prize: X=",
                x: dec_uint,
                _: ", Y=",
                y: dec_uint,
                _: line_ending,
            }
        },
    )
    .parse_next(input)
}
