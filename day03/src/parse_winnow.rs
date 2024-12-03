use winnow::{
    ascii::dec_uint,
    combinator::{alt, delimited, repeat, repeat_till, rest, separated_pair, terminated, trace},
    prelude::*,
    token::any,
};

use crate::{error::AdventError, op::Op};

pub fn parse_part1(content: &str) -> Result<Vec<(u64, u64)>, AdventError> {
    Ok(trace(
        "parser",
        terminated(
            repeat(
                1..,
                repeat_till::<_, _, (), _, _, _, _>(.., any, parse_mul).map(|(_, mul)| mul),
            ),
            rest,
        ),
    )
    .parse(content)?)
}

pub fn parse_part2(content: &str) -> Result<Vec<Op>, AdventError> {
    Ok(trace(
        "parser",
        terminated(
            repeat(
                1..,
                repeat_till::<_, _, (), _, _, _, _>(.., any, parse_op).map(|(_, op)| op),
            ),
            rest,
        ),
    )
    .parse(content)?)
}

fn parse_op(input: &mut &str) -> PResult<Op> {
    trace(
        "parse_op",
        alt((
            parse_mul.map(|(a, b)| Op::Mul(a, b)),
            "don't()".map(|_| Op::Dont), // Using `"don't()".value(Op::Dont)` is slower
            "do()".map(|_| Op::Do),
        )),
    )
    .parse_next(input)
}

fn parse_mul(input: &mut &str) -> PResult<(u64, u64)> {
    trace(
        "parse_mul",
        delimited("mul(", separated_pair(dec_uint, ',', dec_uint), ")"),
    )
    .parse_next(input)
}

// Same as `parse_part2` but drop everything between `don't()` and `do()`
// only returning `mul()` that will need to be processed, instead of returning
// everything then ignoring some of them later.
pub fn parse_part2_skip(content: &str) -> Result<Vec<(u64, u64)>, AdventError> {
    Ok(trace(
        "parser",
        terminated(
            repeat(
                1..,
                repeat_till::<_, _, (), _, _, _, _>(.., any, parse_op_skip).map(|(_, op)| op),
            )
            .fold(Vec::new, |mut acc: Vec<_>, op| {
                if let Some(op) = op {
                    acc.push(op);
                }
                acc
            }),
            rest,
        ),
    )
    .parse(content)?)
}

fn parse_op_skip(input: &mut &str) -> PResult<Option<(u64, u64)>> {
    trace(
        "parse_op",
        alt((
            parse_mul.map(|(a, b)| Some((a, b))),
            (
                "don't()",
                repeat_till::<_, _, (), _, _, _, _>(.., any, "do()"),
            )
                .map(|_| None),
        )),
    )
    .parse_next(input)
}

// Same as `parse_part2_skip` but compute the result on the fly.
pub fn parse_part2_compute(content: &str) -> Result<u64, AdventError> {
    Ok(trace(
        "parser",
        terminated(
            repeat(
                1..,
                repeat_till::<_, _, (), _, _, _, _>(.., any, parse_op_pre_mul).map(|(_, op)| op),
            )
            .fold(
                || 0_u64,
                |mut acc, op| {
                    acc += op;

                    acc
                },
            ),
            rest,
        ),
    )
    .parse(content)?)
}

fn parse_op_pre_mul(input: &mut &str) -> PResult<u64> {
    trace(
        "parse_op",
        alt((
            parse_pre_mul,
            (
                "don't()",
                repeat_till::<_, _, (), _, _, _, _>(.., any, "do()"),
            )
                .map(|_| 0),
        )),
    )
    .parse_next(input)
}

fn parse_pre_mul(input: &mut &str) -> PResult<u64> {
    trace(
        "parse_mul",
        delimited(
            "mul(",
            separated_pair(dec_uint, ',', dec_uint).map(|(a, b): (u64, u64)| a * b),
            ")",
        ),
    )
    .parse_next(input)
}
