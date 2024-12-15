use std::{
    fmt::Debug,
    ops::{Add, AddAssign},
    slice::{Iter, IterMut},
};

#[derive(Default, Clone, PartialEq, Eq)]
pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T>
where
    T: Copy,
{
    pub fn new(data: Vec<T>, width: usize, height: usize) -> Self {
        assert_eq!(data.len(), width * height);
        Self {
            data,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn iter(&self) -> Iter<T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.data.iter_mut()
    }

    pub fn get(&self, pos: Position) -> Option<T> {
        self.data.get(pos.to_index(self.width)).copied()
    }

    pub fn get_mut(&mut self, pos: Position) -> Option<&mut T> {
        self.data.get_mut(pos.to_index(self.width))
    }

    pub fn get_idx(&self, idx: usize) -> Option<T> {
        self.data.get(idx).copied()
    }

    pub fn get_idx_mut(&mut self, idx: usize) -> Option<&mut T> {
        self.data.get_mut(idx)
    }

    pub fn swap(&mut self, pos1: Position, pos2: Position) {
        self.data
            .swap(pos1.to_index(self.width), pos2.to_index(self.width));
    }

    /// # Safety
    ///
    /// `idx` must be in [0..`size()`] range
    pub unsafe fn get_idx_unchecked(&self, idx: usize) -> T {
        *self.data.get_unchecked(idx)
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<T> Debug for Grid<T>
where
    T: Copy + Into<&'static str>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        let mut idx = 0;
        for _y in 0..self.height {
            write!(f, "  ")?;
            for _x in 0..self.width {
                let s: &str = unsafe { self.get_idx_unchecked(idx) }.into();
                write!(f, "{s}")?;
                idx += 1;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub fn to_index(&self, width: usize) -> usize {
        if self.x >= width {
            usize::MAX
        } else {
            self.y * width + self.x
        }
    }
    pub fn from_index(index: usize, width: usize) -> Self {
        let y = index / width;
        let x = index - y * width;
        Self { x, y }
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Position")
            .field("x", &(self.x.wrapping_add(1)))
            .field("y", &(self.y.wrapping_add(1)))
            .finish()
    }
}

impl Add<Direction> for Position {
    type Output = Self;

    fn add(self, dir: Direction) -> Self::Output {
        match dir {
            Direction::Up => Self {
                x: self.x,
                y: self.y.wrapping_sub(1),
            },
            Direction::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Self {
                x: self.x.wrapping_sub(1),
                y: self.y,
            },
        }
    }
}
impl AddAssign<Direction> for Position {
    fn add_assign(&mut self, rhs: Direction) {
        *self = *self + rhs;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl Direction {
    pub fn list() -> &'static [Direction] {
        const LIST: [Direction; 4] = [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];
        &LIST
    }
    pub fn right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn reverse(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }
}
