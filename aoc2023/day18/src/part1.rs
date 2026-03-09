use std::collections::HashMap;

use nom::{
    Finish, IResult, Parser,
    branch::alt,
    bytes::complete::take_while_m_n,
    character::complete::{self, *},
    combinator::{all_consuming, map, map_res, value},
    error::context,
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
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
impl Direction {
    pub fn reverse(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cell {
    pub out_dir: Direction, // direction when going out of the cell
    pub in_dir: Direction,  // Direction when going into the cell
    pub len: i32,
    pub color: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}
impl Coord {
    pub fn move_to(&self, dir: Direction, len: i32) -> Coord {
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

pub type Grid = HashMap<Coord, Cell>;

pub fn parse(input: &str) -> Result<Grid, AocError> {
    let (_, mut grid) = document(input)
        .finish()
        .map_err(|err| AocError::ParseError {
            _input: AocSourceChunk::new(err.input, err.line),
            _span: (err.col - 1, err.len).into(),
            _label: err.label,
            _help: err.help,
            _kind: if let Some(kind) = err.kind {
                kind
            } else if let Some(ctx) = err.context {
                AocErrorKind::Context(ctx)
            } else {
                AocErrorKind::Other
            },
        })?;

    // Fill in the borders
    let mut cur_coord = Coord::default();
    loop {
        let cell = *grid.get(&cur_coord).unwrap();
        let next_coord = cur_coord.move_to(cell.out_dir, cell.len);
        // println!("Coord from: {cur_coord:?} -> {next_coord:?}  (cell: {cell:?})");
        for i in 1..cell.len {
            grid.insert(
                cur_coord.move_to(cell.out_dir, i),
                Cell {
                    out_dir: cell.out_dir,
                    in_dir: cell.out_dir,
                    len: cell.len - i,
                    color: cell.color,
                },
            );
        }
        let next_cell = grid.get_mut(&next_coord).unwrap();
        next_cell.in_dir = cell.out_dir;

        if next_coord == Coord::default() {
            break;
        }
        cur_coord = next_coord;
    }

    Ok(grid)
}

fn document(input: &'_ str) -> IResult<Span<'_>, Grid, AocParseError> {
    all_consuming(delimited(multispace0, data_parser, multispace0)).parse(Span::new(input))
}

fn data_parser(input: Span) -> IResult<Span, Grid, AocParseError> {
    let (input, grid) = map(separated_list1(newline, parse_cell), |vec| {
        vec.into_iter()
            .scan(Coord::default(), |coord, cell| {
                let output = Some((*coord, cell));
                *coord = coord.move_to(cell.out_dir, cell.len);

                output
            })
            .collect()
    })
    .parse(input)?;

    Ok((input, grid))
}

fn parse_cell(input: Span) -> IResult<Span, Cell, AocParseError> {
    let (input, out_dir) = context(
        "direction",
        alt((
            value(Direction::Up, char('U')),
            value(Direction::Down, char('D')),
            value(Direction::Left, char('L')),
            value(Direction::Right, char('R')),
        )),
    )
    .parse(input)?;
    let (input, len) = context("length", preceded(space1, complete::i32)).parse(input)?;
    let (input, color) = context("color", preceded(space1, parse_color)).parse(input)?;

    Ok((
        input,
        Cell {
            out_dir,
            in_dir: Direction::Up,
            len,
            color,
        },
    ))
}

fn parse_color(input: Span) -> IResult<Span, Color, AocParseError> {
    delimited(
        char('('),
        preceded(
            char('#'),
            map(
                tuple((hex_u8_exact, hex_u8_exact, hex_u8_exact)),
                |(r, g, b)| Color { r, g, b },
            ),
        ),
        char(')'),
    )
    .parse(input)
}

fn hex_u8_exact(input: Span) -> IResult<Span, u8, AocParseError> {
    let v = map_res(take_while_m_n(2, 2, is_hex_digit), |s: Span| {
        u8::from_str_radix(&s, 16)
    })
    .parse(input)?;

    Ok(v)
}

fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}
