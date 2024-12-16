use std::collections::HashMap;

use nom::{
    bytes::complete::{take_until, take_while_m_n},
    character::complete::*,
    combinator::{all_consuming, map, map_res},
    error::context,
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    Finish, IResult, Parser,
};

pub use crate::aocerror::*;

type Span<'a> = nom_locate::LocatedSpan<&'a str>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Coord {
    pub x: i64,
    pub y: i64,
}
impl Coord {
    pub fn move_to(&self, dir: Direction, len: i64) -> Coord {
        let mut coord = *self;
        match dir {
            Direction::Up => coord.y -= len,
            Direction::Down => coord.y += len,
            Direction::Left => coord.x -= len,
            Direction::Right => coord.x += len,
        };
        coord
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub dir: Direction,
    pub len: i64,
}

pub type Grid = HashMap<Coord, Cell>;

pub fn parse(input: &str) -> Result<Grid, AocError> {
    let (_, grid) = document(input)
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

    Ok(grid)
}

fn document(input: &str) -> IResult<Span, Grid, AocParseError> {
    all_consuming(delimited(multispace0, data_parser, multispace0)).parse(Span::new(input))
}

fn data_parser(input: Span) -> IResult<Span, Grid, AocParseError> {
    let (input, grid) = map(separated_list1(newline, parse_cell), |vec| {
        vec.into_iter()
            .scan(Coord::default(), |coord, cell| {
                let output = Some((*coord, cell));
                *coord = coord.move_to(cell.dir, cell.len);

                println!("Adding {output:?}  (next: {:?}", *coord);
                output
            })
            .collect()
    })
    .parse(input)?;

    Ok((input, grid))
}

fn parse_cell(input: Span) -> IResult<Span, Cell, AocParseError> {
    context("color", preceded(take_until("("), parse_color)).parse(input)
}

fn parse_color(input: Span) -> IResult<Span, Cell, AocParseError> {
    delimited(
        char('('),
        preceded(
            char('#'),
            map(tuple((hex_5_exact, dir_digit)), |(len, dir)| Cell {
                dir,
                len,
            }),
        ),
        char(')'),
    )
    .parse(input)
}

fn hex_5_exact(input: Span) -> IResult<Span, i64, AocParseError> {
    map_res(take_while_m_n(5, 5, is_hex_digit), |s: Span| {
        i64::from_str_radix(&s, 16)
    })
    .parse(input)
}

fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

fn dir_digit(input: Span) -> IResult<Span, Direction, AocParseError> {
    map(one_of("0123"), |c| match c {
        '0' => Direction::Right,
        '1' => Direction::Down,
        '2' => Direction::Left,
        '3' => Direction::Up,
        _ => panic!("can't be here"),
    })
    .parse(input)
}
