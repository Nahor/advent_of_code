use std::collections::BTreeMap;

use nom::{
    character::complete::*,
    combinator::{all_consuming, map},
    multi::{many0, many1, separated_list1},
    sequence::{delimited, preceded, terminated},
    Finish, IResult, Parser,
};

pub mod aocerror;
pub mod progress;
pub use aocerror::*;

type Span<'a> = nom_locate::LocatedSpan<&'a str>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RockType {
    Round,
    Square,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

pub fn parse(input: &str) -> Result<BTreeMap<Coord, RockType>, AocError> {
    let (_, data) = document(input).finish().map_err(|err| {
        //let span_substr = &err.input[..err.len];

        AocError::ParseError {
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
        }
    })?;

    Ok(data)
}

fn document(input: &str) -> IResult<Span, BTreeMap<Coord, RockType>, AocParseError> {
    all_consuming(delimited(multispace0, data_parser, multispace0)).parse(Span::new(input))
}

fn data_parser(input: Span) -> IResult<Span, BTreeMap<Coord, RockType>, AocParseError> {
    map(
        separated_list1(
            newline,
            terminated(
                many1(preceded(many0(char('.')), parse_rock)),
                many0(char('.')),
            ),
        ),
        |vec| vec.into_iter().flatten().collect(),
    )
    .parse(input)
}

fn parse_rock(input: Span) -> IResult<Span, (Coord, RockType), AocParseError> {
    let (input, position) = nom_locate::position(input)?;
    let (input, rock) = map(one_of("#O"), |c| match c {
        '#' => RockType::Square,
        'O' => RockType::Round,
        _ => panic!("not match for {c}"),
    })
    .parse(input)?;
    Ok((
        input,
        (
            Coord {
                x: position.get_utf8_column() - 1,
                y: (position.location_line() - 1) as usize,
            },
            rock,
        ),
    ))
}
