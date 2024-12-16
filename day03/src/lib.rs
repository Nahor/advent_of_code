use std::{
    cell::RefCell,
    cmp::Ordering,
    collections::{BTreeMap, HashSet},
    hash::Hash,
    rc::Rc,
};

use aocerror::{AocError, AocSourceChunk};
use regex::Regex;

pub mod aocerror;

#[derive(Debug, Clone)]
pub enum CellData {
    Empty,
    Symbol(char),
    Number(Rc<Part>),
}

pub type SymbolList = RefCell<HashSet<char>>;
pub type PartList = Vec<Rc<Part>>;
pub type GridList = BTreeMap<Coord, CellData>;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}
impl Coord {
    pub fn add(&self, x: isize, y: isize) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }
}
impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Coord {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.y.cmp(&other.y) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.x.cmp(&other.x)
    }
}

// We have duplicate numbers but they must be considered separate parts anyway
#[derive(Debug, Default)]
pub struct Part {
    pub number: u32, // Not unique
    pub symbols: SymbolList,
}

pub fn parse(input: &str) -> Result<(PartList, GridList), AocError> {
    let mut parts = PartList::new();
    let mut grid = GridList::new();

    // Parse part numbers and location
    input.lines().enumerate().try_for_each(|(lineno, line)| {
        parse_line(&mut parts, &mut grid, lineno, line).map_err(|err| AocError::InputError {
            src: AocSourceChunk::new(line.to_owned(), lineno),
            bad_bit: (0, line.len()).into(),
            inner: Some(Box::new(err)),
        })
    })?;

    // Propagate symbols to parts
    for cell in grid.iter() {
        if let (coord, CellData::Symbol(symbol)) = cell {
            add_symbol(&grid, coord.add(-1, -1), *symbol);
            add_symbol(&grid, coord.add(0, -1), *symbol);
            add_symbol(&grid, coord.add(1, -1), *symbol);
            add_symbol(&grid, coord.add(-1, 0), *symbol);
            add_symbol(&grid, coord.add(1, 0), *symbol);
            add_symbol(&grid, coord.add(-1, 1), *symbol);
            add_symbol(&grid, coord.add(0, 1), *symbol);
            add_symbol(&grid, coord.add(1, 1), *symbol);
        }
    }

    Ok((parts, grid))
}

fn parse_line(
    parts: &mut PartList,
    grid: &mut GridList,
    lineno: usize,
    line: &str,
) -> Result<(), AocError> {
    let re = Regex::new(r"(?:\d+)|.").unwrap();
    re.captures_iter(line).try_for_each(|captures| {
        let re_match = captures.get(0).expect("regex with no match");
        let match_str = re_match.as_str();
        let cell = match match_str.chars().next().unwrap() {
            '.' => CellData::Empty,
            _ if match_str.starts_with(|c: char| c.is_ascii_digit()) => {
                let part_number = match_str
                    .parse()
                    .map_err(|err| AocError::InvalidPartNumber {
                        num_str: re_match.as_str().to_owned(),
                        inner: Some(Box::new(err)),
                    })?;
                let part = Part {
                    number: part_number,
                    ..Default::default()
                };
                let rc_part = Rc::new(part);
                parts.push(Rc::clone(&rc_part));
                CellData::Number(rc_part)
            }
            c => CellData::Symbol(c),
        };
        if let CellData::Empty = cell {
        } else {
            for x in re_match.start()..re_match.end() {
                grid.insert(
                    Coord {
                        x: x as isize,
                        y: lineno as isize,
                    },
                    cell.clone(),
                );
            }
        }
        Ok(())
    })?;
    Ok(())
}

fn add_symbol(grid: &GridList, coord: Coord, symbol: char) {
    if let Some(CellData::Number(part)) = grid.get(&coord) {
        part.symbols.borrow_mut().insert(symbol);
    };
}
