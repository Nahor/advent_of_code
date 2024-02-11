use std::fmt::Display;

use nom::{
    branch::alt,
    character::complete::{self, *},
    combinator::{all_consuming, cut, map, value},
    error::context,
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    Finish, IResult, Parser,
};

pub mod aocerror;
pub mod progress;
pub use aocerror::*;

type Span<'a> = nom_locate::LocatedSpan<&'a str>;

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add(usize), // focal
    Remove,
}
impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Add(focal) => write!(f, "={}", focal),
            Operation::Remove => write!(f, "-"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Step {
    pub label: String,
    pub op: Operation,

    pub hash: usize,
}
impl Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.label, self.op)
    }
}

pub fn parse(input: &str) -> Result<Vec<Step>, AocError> {
    let (_, data) = document(input)
        .finish()
        .map_err(|err| AocError::ParseError {
            input: AocSourceChunk::new(err.input, err.line),
            span: (err.col - 1, err.len).into(),
            label: err.label,
            help: err.help,
            kind: if let Some(kind) = err.kind {
                kind
            } else if let Some(ctx) = err.context {
                AocErrorKind::Context(ctx)
            } else {
                AocErrorKind::Other
            },
        })?;

    Ok(data)
}

fn document(input: &str) -> IResult<Span, Vec<Step>, AocParseError> {
    all_consuming(delimited(multispace0, data_parser, multispace0)).parse(Span::new(input))
}

fn data_parser(input: Span) -> IResult<Span, Vec<Step>, AocParseError> {
    separated_list1(char(','), step_parser).parse(input)
}

fn step_parser(input: Span) -> IResult<Span, Step, AocParseError> {
    context(
        "step",
        map(tuple((label, operation)), |(label, op)| {
            let hash = aoc_hash(label.as_str());
            Step { label, op, hash }
        }),
    )
    .parse(input)
}

fn label(input: Span) -> IResult<Span, String, AocParseError> {
    context(
        "label",
        map(
            alphanumeric1, /*many1(none_of("=-,\n"))*/
            |vec: Span| {
                //let v: Vec<char> = vec;
                let label = vec.chars().collect::<String>();
                //println!("Got label {label}");
                //let label = String::new();
                label
            },
        ),
    )
    .parse(input)
}

fn operation(input: Span) -> IResult<Span, Operation, AocParseError> {
    cut(context(
        "operation",
        //preceded(peek(one_of("-=")), alt((op_add, op_remove))),
        alt((op_add, op_remove)),
    ))
    .parse(input)
}

fn op_add(input: Span) -> IResult<Span, Operation, AocParseError> {
    context(
        "op_add",
        map(preceded(char('='), cut(complete::u32)), |focal| {
            //println!("Got Add op with focal {focal}");
            Operation::Add(focal as usize)
        }),
    )
    .parse(input)
}

fn op_remove(input: Span) -> IResult<Span, Operation, AocParseError> {
    context("op_remove", value(Operation::Remove, char('-'))).parse(input)
}

pub fn aoc_hash(span: &str) -> usize {
    span.chars()
        .into_iter()
        .fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}
