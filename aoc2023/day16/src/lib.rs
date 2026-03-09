use enumflags2::{BitFlag, BitFlags, bitflags};

pub mod aocerror;
pub mod progress;
pub use aocerror::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Mirror {
    #[default]
    None,
    Degree0, // horizontal
    Degree45,
    Degree90,
    Degree135,
}

#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Cell {
    pub mirror: Mirror,
    pub energy: BitFlags<Direction>, // side where the energy is (regardless if it's coming in or out)
}

pub type Grid = Vec<Vec<Cell>>;

pub fn parse(input: &str) -> Result<Grid, AocError> {
    let grid = input
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .map(|(lineno, line)| {
            line.chars()
                .enumerate()
                .map(|(charno, c)| match c {
                    '.' => Ok(Cell::default()),
                    '-' => Ok(Cell {
                        mirror: Mirror::Degree0,
                        energy: Direction::empty(),
                    }),
                    '/' => Ok(Cell {
                        mirror: Mirror::Degree45,
                        energy: Direction::empty(),
                    }),
                    '|' => Ok(Cell {
                        mirror: Mirror::Degree90,
                        energy: Direction::empty(),
                    }),
                    '\\' => Ok(Cell {
                        mirror: Mirror::Degree135,
                        energy: Direction::empty(),
                    }),
                    _ => Err(AocError::InvalidLineError {
                        desc: format!("invalid char {c}").to_owned(),
                        src: AocSourceChunk::new(line.to_owned(), lineno),
                        span: (charno, 1).into(),
                        inner: None,
                    }),
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(grid)
}
