use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Add, Sub},
};

pub mod aocerror;
pub mod progress;
pub use aocerror::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cell {
    Rock,
    Garden(Option<u64>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}
impl Coord {
    pub fn new(x: isize, y: isize) -> Coord {
        Coord { x, y }
    }
    pub fn min(self, other: Coord) -> Coord {
        Coord {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }
    pub fn max(self, other: Coord) -> Coord {
        Coord {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }
}
impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Sub<Coord> for Coord {
    type Output = Coord;

    fn sub(self, rhs: Coord) -> Self::Output {
        Coord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

pub type Grid = HashMap<Coord, Cell>;

//////////////
//////////////
//////////////
//////////////
//////////////

pub fn parse(input: &str) -> Result<(Coord, Grid), AocError> {
    let mut start = Coord::default();
    let grid = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            //let start = Rc::clone(&start);
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => Ok((Coord::new(x as isize, y as isize), Cell::Garden(None))),
                    '#' => Ok((Coord::new(x as isize, y as isize), Cell::Rock)),
                    'S' => {
                        start = Coord::new(x as isize, y as isize);
                        Ok((start, Cell::Garden(None)))
                    }
                    _ => Err(AocError::InvalidLineError {
                        desc: format!("invalid character '{c}'").to_owned(),
                        src: AocSourceChunk::new(line.to_owned(), y),
                        span: (x, 1).into(),
                        inner: None,
                    }),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Result<Grid, _>>()?;

    Ok((start, grid))
    //Ok((Coord::default(), Grid::new()))
}
