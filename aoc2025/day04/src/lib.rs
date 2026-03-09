use std::fmt::{Display, Write};

use common::error::AdventError;
use winnow::stream::Accumulate;

pub mod parse;
pub mod part1;
pub mod part1_rayon;
pub mod part2;
pub mod part2_floodfill;

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Empty,
    Roll,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => f.write_char('.'),
            Cell::Roll => f.write_char('@'),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Grid {
    grid: Vec<Vec<Cell>>,
    lines: isize,
    columns: isize,
}

impl Grid {
    pub fn lines(&self) -> isize {
        self.lines
    }

    pub fn columns(&self) -> isize {
        self.columns
    }

    pub fn get(&self, x: isize, y: isize) -> Option<Cell> {
        if (0..self.columns).contains(&x) && (0..self.lines).contains(&y) {
            Some(self.grid[y as usize][x as usize])
        } else {
            None
        }
    }

    pub fn set(&mut self, x: isize, y: isize, cell: Cell) {
        if (0..self.columns).contains(&x) && (0..self.lines).contains(&y) {
            self.grid[y as usize][x as usize] = cell;
        }
    }

    pub fn add(&mut self, line: Vec<Cell>) -> Result<(), AdventError> {
        let len = line.len() as isize;
        if self.lines == 0 {
            self.lines = 1;
            self.columns = len;
            self.grid.push(line);
            Ok(())
        } else if len == self.columns {
            self.grid.push(line);
            self.lines += 1;
            Ok(())
        } else {
            Err("invalid number of columns".into())
        }
    }
}

impl Accumulate<Vec<Cell>> for Grid {
    // Required methods
    fn initial(capacity: Option<usize>) -> Self {
        let mut grid = Grid::default();
        if let Some(capacity) = capacity {
            grid.grid.reserve(capacity);
        }
        grid
    }
    fn accumulate(&mut self, acc: Vec<Cell>) {
        self.add(acc).expect("invalid number of columns");
    }
}
