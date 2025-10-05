use std::{collections::BTreeMap, fmt::Display};

use nom::{
    Finish, IResult, Parser,
    character::complete::*,
    combinator::{all_consuming, map},
    multi::{many0, many1, separated_list1},
    sequence::{delimited, preceded, terminated},
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
impl Display for RockType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RockType::Round => write!(f, "O"),
            RockType::Square => write!(f, "#"),
        }
    }
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

fn document(input: &'_ str) -> IResult<Span<'_>, BTreeMap<Coord, RockType>, AocParseError> {
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

pub fn process_one_direction(
    rock_map: BTreeMap<Coord, RockType>,
    size: usize,
) -> BTreeMap<Coord, RockType> {
    rock_map
        .into_iter()
        .scan(
            Coord { x: 0, y: 0 },
            |next_coord, (mut rock_coord, rock_type)| match rock_type {
                RockType::Round => {
                    if next_coord.x == rock_coord.x {
                        rock_coord = *next_coord;
                    } else {
                        // Change of column
                        rock_coord = Coord {
                            x: rock_coord.x,
                            y: 0,
                        };
                    }
                    *next_coord = Coord {
                        x: rock_coord.x,
                        y: rock_coord.y + 1,
                    };
                    Some((rock_coord, rock_type))
                }
                RockType::Square => {
                    *next_coord = Coord {
                        x: rock_coord.x,
                        y: rock_coord.y + 1,
                    };
                    Some((rock_coord, rock_type))
                }
            },
        )
        .map(|(coord, rock_type)| {
            // Rotate the map 90 degree clockwise
            (
                Coord {
                    x: size - coord.y - 1,
                    y: coord.x,
                },
                rock_type,
            )
        })
        .collect::<BTreeMap<Coord, RockType>>()
}

// Computes the load in the direction of the last tilt (which, because we
// rotated once clockwise, means the "right")
pub fn compute_load_after_tilt(rock_map: &BTreeMap<Coord, RockType>) -> usize {
    rock_map
        .iter()
        .filter(|(_, rock_type)| **rock_type == RockType::Round)
        .map(|(coord, _)| coord.x + 1)
        .sum()
}

// Computes the load in the direction of the last tilt (which, because we
// rotated once clockwise, means the "right")
pub fn compute_load_before_tilt(rock_map: &BTreeMap<Coord, RockType>, size: usize) -> usize {
    rock_map
        .iter()
        .filter(|(_, rock_type)| **rock_type == RockType::Round)
        .map(|(coord, _)| size - coord.y)
        .sum()
}

pub fn compute_loads(
    rock_map: &BTreeMap<Coord, RockType>,
    size: usize,
) -> (usize, usize, usize, usize) {
    let up = rock_map
        .iter()
        .filter(|(_, rock_type)| **rock_type == RockType::Round)
        .map(|(coord, _)| size - coord.y)
        .sum();

    let left = rock_map
        .iter()
        .filter(|(_, rock_type)| **rock_type == RockType::Round)
        .map(|(coord, _)| size - coord.x)
        .sum();

    let down = rock_map
        .iter()
        .filter(|(_, rock_type)| **rock_type == RockType::Round)
        .map(|(coord, _)| coord.y + 1)
        .sum();

    let right = rock_map
        .iter()
        .filter(|(_, rock_type)| **rock_type == RockType::Round)
        .map(|(coord, _)| coord.x + 1)
        .sum();

    (up, left, down, right)
}
