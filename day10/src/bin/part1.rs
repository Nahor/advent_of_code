use std::{collections::HashMap, fmt::Display, iter::successors, ops::Add};

use day10::aocerror::AocError;
use miette;

fn main() -> miette::Result<()> {
    let input = include_bytes!("input.txt");
    let output = process(input)?;
    dbg!(output);
    Ok(())
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}
impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x + 1, self.y + 1) // +1 to match line numbering in an editor rather than array indices
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl From<Direction> for Coord {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => Coord { x: 0, y: -1 },
            Direction::East => Coord { x: 1, y: 0 },
            Direction::South => Coord { x: 0, y: 1 },
            Direction::West => Coord { x: -1, y: 0 },
        }
    }
}

struct Grid {
    data: Vec<u8>,
    width: isize,
    height: isize,
}
impl Grid {
    fn get(&self, coord: Coord) -> Option<u8> {
        match self.linear_coord(coord) {
            Some(coord) => self.data.get(coord).copied(),
            None => None,
        }
    }

    fn linear_coord(&self, coord: Coord) -> Option<usize> {
        if coord.x < 0 || coord.y < 0 {
            None
        } else if coord.x >= self.width || coord.y >= self.height {
            None
        } else {
            Some((coord.y * (self.width + 1) + coord.x) as usize)
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Out(Direction, Direction);

fn process(input: &[u8]) -> Result<usize, AocError> {
    let width = input.split(|b| *b == b'\n').next().unwrap().len() as isize;
    let height = input.len() as isize / (width + 1);
    let grid = Grid {
        data: input.into(),
        width,
        height,
    };

    let mut next_codes = HashMap::new();
    next_codes.insert(b'|', Out(Direction::North, Direction::South));
    next_codes.insert(b'-', Out(Direction::East, Direction::West));
    next_codes.insert(b'L', Out(Direction::North, Direction::East));
    next_codes.insert(b'J', Out(Direction::North, Direction::West));
    next_codes.insert(b'7', Out(Direction::West, Direction::South));
    next_codes.insert(b'F', Out(Direction::East, Direction::South));

    let start = (0..height)
        .into_iter()
        .find_map(|y| {
            (0..width).into_iter().find_map(|x| {
                if grid.get(Coord { x, y }).is_some_and(|b| b == b'S') {
                    Some(Coord { x, y })
                } else {
                    None
                }
            })
        })
        .ok_or_else(|| AocError::InvalidDocumentError {
            desc: "no starting position".to_owned(),
        })?;

    // Per spec, only two pipes connect to the starting position, i.e. only
    // the pipes belonging to the loop.

    // Don't need the fourth since it's supposed to loop to one of the other 3
    let mut valid_dir = vec![
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ]
    .into_iter()
    .filter(|&dir| match dir {
        Direction::North => grid
            .get(start + dir.into())
            .is_some_and(|b| [b'|', b'7', b'F'].contains(&b)),
        Direction::East => grid
            .get(start + dir.into())
            .is_some_and(|b| [b'-', b'7', b'J'].contains(&b)),
        Direction::South => grid
            .get(start + dir.into())
            .is_some_and(|b| [b'|', b'J', b'L'].contains(&b)),
        Direction::West => grid
            .get(start + dir.into())
            .is_some_and(|b| [b'-', b'L', b'F'].contains(&b)),
    })
    .collect::<Vec<_>>();
    valid_dir.sort_unstable();
    println!("dir: {valid_dir:?}");

    let count = successors(
        Some((start, valid_dir.first().copied().unwrap())),
        |&(pos, dir)| {
            let new_pos = pos + dir.into();
            // println!("    {}", pos);
            if new_pos == start {
                return None;
            }
            let b = grid.get(new_pos).unwrap();
            let dir = match (b, dir) {
                (b'|', Direction::North) => Direction::North,
                (b'|', Direction::South) => Direction::South,
                (b'-', Direction::West) => Direction::West,
                (b'-', Direction::East) => Direction::East,
                (b'L', Direction::South) => Direction::East,
                (b'L', Direction::West) => Direction::North,
                (b'J', Direction::East) => Direction::North,
                (b'J', Direction::South) => Direction::West,
                (b'F', Direction::North) => Direction::East,
                (b'F', Direction::West) => Direction::South,
                (b'7', Direction::North) => Direction::West,
                (b'7', Direction::East) => Direction::South,
                _ => panic!(
                    "Invalid combination ({},{:?}) for {}->{}",
                    b as char, dir, pos, new_pos
                ),
            };
            Some((new_pos, dir))
        },
    )
    .count();
    println!("Count: {count}");

    Ok(count / 2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() -> miette::Result<()> {
        let input = b"\
-L|F7
7S-7|
L|7||
-L-J|
L|-JF
";
        assert_eq!(process(input).unwrap(), 4);

        Ok(())
    }

    #[test]
    fn test2() -> miette::Result<()> {
        let input = b"\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";
        assert_eq!(process(input).unwrap(), 8);

        Ok(())
    }
}
