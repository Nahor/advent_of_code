use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
    hash::Hash,
    ops::{Add, Range},
};
use std::{fs::File, io::Write};

pub mod aocerror;
pub mod progress;
pub use aocerror::*;
use num::{One, ToPrimitive};
use petgraph::{algo::all_simple_paths, dot::Dot, Graph};

pub trait MinMax<T> {
    type Output;
    fn minmax(&self, other: T) -> Self::Output;
}
impl MinMax<isize> for isize {
    type Output = (Self, Self);
    fn minmax(&self, other: isize) -> Self::Output {
        if *self <= other {
            (*self, other)
        } else {
            (other, *self)
        }
    }
}
impl MinMax<isize> for (isize, isize) {
    type Output = Self;
    fn minmax(&self, other: isize) -> Self::Output {
        if other < self.0 {
            (other, self.1)
        } else if self.1 < other {
            (self.0, other)
        } else {
            *self
        }
    }
}
impl<T> MinMax<T> for Range<T>
where
    T: Ord + Add<T, Output = T> + One + Clone,
{
    type Output = Self;
    fn minmax(&self, other: T) -> Self::Output {
        if other < self.start {
            other..self.end.clone()
        } else if self.end <= other {
            (self.start.clone())..(other + T::one())
        } else {
            self.clone()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}
impl Coord {
    pub fn new<T>(x: T, y: T) -> Self
    where
        T: ToPrimitive,
    {
        Self {
            x: x.to_isize().unwrap(),
            y: y.to_isize().unwrap(),
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
impl Add<Direction> for Coord {
    type Output = Coord;

    fn add(self, rhs: Direction) -> Self::Output {
        self + Coord::from(rhs)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    pub fn reverse(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
    pub fn next(self) -> &'static [Direction] {
        match self {
            Direction::North => [Direction::North, Direction::East, Direction::West].as_slice(),
            Direction::East => [Direction::North, Direction::East, Direction::South].as_slice(),
            Direction::South => [Direction::East, Direction::South, Direction::West].as_slice(),
            Direction::West => [Direction::North, Direction::South, Direction::West].as_slice(),
        }
    }
}
impl From<Direction> for Coord {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => Coord::new(0, -1),
            Direction::East => Coord::new(1, 0),
            Direction::South => Coord::new(0, 1),
            Direction::West => Coord::new(-1, 0),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Cell {
    Flat,
    Slope(Direction),
}

pub struct Maze {
    pub range_x: Range<isize>,
    pub range_y: Range<isize>,
    pub start: Coord,
    pub end: Coord,
    pub cells: HashMap<Coord, Cell>,
}

pub fn parse(input: &str) -> Result<Maze, AocError<'_>> {
    let mut start = None;
    let mut end = Coord::default();
    let mut range_x = 0..0;
    let mut range_y = 0..0;

    let cells = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            range_y = range_y.minmax(y as isize);
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    if c != '#' {
                        if start.is_none() {
                            // The start is single non-wall cell on the first row (aka
                            // the first non-wall of the whole maze), per specs
                            start = Some(Coord::new(x, y));
                        }
                        // The end is the single non-wall on the last row (aka the last
                        // non-wall of the whole maze), per specs
                        end = Coord::new(x, y);
                    }

                    range_x = range_x.minmax(x as isize);

                    match c {
                        '#' => None,
                        '.' => Some(Ok((Coord::new(x, y), Cell::Flat))),
                        '^' => Some(Ok((Coord::new(x, y), Cell::Slope(Direction::North)))),
                        '>' => Some(Ok((Coord::new(x, y), Cell::Slope(Direction::East)))),
                        'v' => Some(Ok((Coord::new(x, y), Cell::Slope(Direction::South)))),
                        '<' => Some(Ok((Coord::new(x, y), Cell::Slope(Direction::West)))),
                        _ => Some(Err(AocError::GenericParseError {
                            desc: format!("unexpected character `{c}`").to_string(),
                            src: AocSourceChunk::new(line, y),
                            span: (x, 1).into(),
                            inner: None,
                        })),
                    }
                })
                // Ideally, we wouldn't collect but return the iterator, but
                // that means multiple instances of the `filter_map` closure,
                // which means multiple borrowing of the captured variables
                .collect::<Vec<_>>()
        })
        .collect::<Result<HashMap<_, _>, _>>()?;

    Ok(Maze {
        range_x,
        range_y,
        start: start.unwrap(),
        end,
        cells,
    })
}

#[derive(Debug, Clone, Default)]
pub struct SimpleCell {
    pub edges: HashMap<Coord, usize>,
}

pub fn condense(maze: &Maze, with_slope: bool) -> HashMap<Coord, SimpleCell> {
    let mut condensed = HashMap::new();
    condensed.insert(maze.start, SimpleCell::default());
    condensed.insert(maze.end, SimpleCell::default());

    let mut pending = VecDeque::new();
    pending.push_back((
        maze.start,
        maze.start + Direction::South,
        1,
        Direction::South,
    ));
    while let Some((start, mut end, mut cost, mut dir)) = pending.pop_front() {
        loop {
            if condensed.contains_key(&end) {
                // Reached a known intersection or the end

                // Add this branch to our starting point
                let cell = condensed.get_mut(&start).unwrap();
                cell.edges.insert(end, cost);

                // Don't add the reverse path to the intersection since that
                // intersection must already have created feelers towards our
                // starting point (or it's the end where there is no backward
                // path anyway)

                // Then break to process another path
                break;
            }

            let next_dirs = match maze.cells.get(&end) {
                Some(Cell::Slope(slope_dir)) if with_slope && dir == slope_dir.reverse() => break, // Can't go up a slope
                Some(Cell::Slope(slope_dir)) if with_slope => vec![*slope_dir],
                Some(_) => dir.next().to_vec(),
                None => unreachable!("`end` is supposed to be valid"),
            };
            let dest = next_dirs
                .into_iter()
                .filter_map(|dir| {
                    let coord = end + dir;
                    maze.cells.get(&coord).map(|_| (coord, dir))
                })
                .collect::<Vec<_>>();
            if dest.is_empty() {
                // dead end => drop this path
                break;
            } else if dest.len() == 1 {
                // only one path to follow, keep following the path
                (end, dir) = dest[0];
                cost += 1;
            } else {
                // this is a new intersection, i.e. end of the path

                // Add this path to our starting point
                let cell = condensed.get_mut(&start).unwrap();
                cell.edges.insert(end, cost);

                // Create a new intersection
                condensed.insert(end, SimpleCell::default());
                // Add a branch going back up the path in case one of the other
                // branches can be taken in reverse as well (since the
                // intersection will already exist, a reverse path will not
                // send new feelers)
                pending.push_back((end, end + dir.reverse(), 1, dir.reverse()));

                dest.into_iter().for_each(|(coord, dir)| {
                    pending.push_back((end, coord, 1, dir));
                });

                break;
            }
        }
    }

    condensed
}

pub fn petegraph(maze: &Maze, condensed: &HashMap<Coord, SimpleCell>) -> usize {
    let mut graph = Graph::new();
    let indices = condensed
        .iter()
        .map(|(coord, _)| (*coord, graph.add_node(*coord)))
        .collect::<HashMap<_, _>>();
    condensed.iter().for_each(|(coord, cell)| {
        cell.edges.iter().for_each(|(next_coord, cost)| {
            let a_idx = *indices.get(coord).unwrap();
            let b_idx = *indices.get(next_coord).unwrap();
            if !graph.contains_edge(a_idx, b_idx) {
                graph.add_edge(a_idx, b_idx, cost);
            }
        });
    });

    let start_idx = *indices.get(&maze.start).unwrap();
    let end_idx = *indices.get(&maze.end).unwrap();

    let mut file = File::create("./graph.dot").unwrap();
    let _ = write!(file, "{:?}", Dot::new(&graph));

    let (max, solution) = all_simple_paths::<Vec<_>, _>(&graph, start_idx, end_idx, 0, None)
        //.map(|v| v.len())
        .fold((0, vec![]), |(max, solution), path| {
            let cost = path
                .windows(2)
                .map(|indices| {
                    let start = graph.node_weight(indices[0]).unwrap();
                    let end = graph.node_weight(indices[1]).unwrap();
                    *condensed.get(start).unwrap().edges.get(end).unwrap()
                })
                .sum::<usize>();
            if cost > max {
                (cost, path)
            } else {
                (max, solution)
            }
        });
    let solution = solution
        .into_iter()
        .map(|idx| graph.node_weight(idx).unwrap())
        .collect::<Vec<_>>();
    println!("Solution: {solution:?}");
    max
}
