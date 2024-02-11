use crate::aocerror::{span_from_substr, AocError, AocErrorKind, AocParseError};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, newline, space0, space1},
    combinator::{all_consuming, map, map_res},
    error::context,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, tuple},
    Finish, IResult, Parser,
};

#[derive(Debug)]
pub struct Data {
    pub time: u64,
    pub distance: u64,
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

fn time(input: &str) -> IResult<&str, u64, AocParseError<&str>> {
    line("Time", input)
}
fn distance(input: &str) -> IResult<&str, u64, AocParseError<&str>> {
    line("Distance", input)
}

fn line<'a>(
    tag_str: &'static str,
    input: &'a str,
) -> IResult<&'a str, u64, AocParseError<&'a str>> {
    preceded(
        context(tag_str, tuple((tag(tag_str), tag(":"), space0))),
        map_res(
            separated_list1(space1, context("expected u64", digit1)),
            |vec| {
                let iter = vec.iter().map(|&s| s);
                let collect: String = iter.collect();
                collect.parse::<u64>()
            },
        ),
    )
    .parse(input)
}
