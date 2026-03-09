use crate::aocerror::{AocError, AocErrorKind, AocParseError, span_from_substr};
use nom::{
    Finish, IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, multispace0, newline, space0, space1},
    combinator::{all_consuming, map},
    error::context,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, tuple},
};

#[derive(Debug)]
pub struct Data {
    pub time: Vec<u32>,
    pub distance: Vec<u32>,
}

pub fn parse(input: &str) -> Result<Data, AocError> {
    let (_, data) = data_parser(input).finish().map_err(|err| {
        let span_substr = &err.input[..err.len];

        AocError::ParseError {
            input: input.to_owned(),
            span: span_from_substr(input, span_substr),
            help: err.help,
            label: err.label,
            kind: if let Some(kind) = err.kind {
                kind
            } else if let Some(ctx) = err.context {
                AocErrorKind::Context(ctx)
            } else {
                AocErrorKind::Other
            },
        }
    })?;

    Ok(data)
}

fn data_parser(input: &str) -> IResult<&str, Data, AocParseError<&str>> {
    all_consuming(delimited(
        multispace0,
        map(
            separated_pair(time, newline, distance),
            |(time, distance)| Data { time, distance },
        ),
        multispace0,
    ))
    .parse(input)
}

fn time(input: &str) -> IResult<&str, Vec<u32>, AocParseError<&str>> {
    preceded(
        context("Time tag", tuple((tag("Time:"), space0))),
        separated_list1(space1, context("expected u32", complete::u32)),
    )
    .parse(input)
}
fn distance(input: &str) -> IResult<&str, Vec<u32>, AocParseError<&str>> {
    preceded(
        context("Distance tag", tuple((tag("Distance:"), space0))),
        separated_list1(space1, context("expected u32", complete::u32)),
    )
    .parse(input)
}
