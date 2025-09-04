#![allow(clippy::mutable_key_type)]
use std::{
    cell::{self, RefCell},
    collections::{hash_map::Entry, HashMap, HashSet, VecDeque},
    fmt::Debug,
    hash::Hash,
    rc::Rc,
};

pub mod aocerror;
pub mod progress;
pub use aocerror::*;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

#[derive(Clone, Default)]
pub struct Brick {
    pub id: usize,

    pub v1: Coord,
    pub v2: Coord,

    pub supported_by: BrickList,
    pub supports: BrickList,
}
impl Brick {
    pub fn new(id: usize, v1: Coord, v2: Coord) -> Self {
        Self {
            id,
            v1,
            v2,
            ..Default::default()
        }
    }
    pub fn at_height(&self, z: isize) -> bool {
        let (z1, z2) = self.minmax_z();
        (z1 <= z) && (z2 >= z)
    }
    pub fn get_name(&self) -> String {
        let mut i = self.id;
        let mut chars = Vec::new();
        const MAP: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        while i != 0 {
            chars.push(MAP[i % 26]);
            i /= 26;
        }
        if chars.is_empty() {
            chars.push(MAP[0]);
        }
        chars.reverse();
        unsafe { String::from_utf8_unchecked(chars) }
    }
    pub fn minmax_z(&self) -> (isize, isize) {
        self.v1.z.minmax(self.v2.z)
    }
    pub fn min_z(&self) -> isize {
        self.minmax_z().0
    }
    pub fn max_z(&self) -> isize {
        self.minmax_z().1
    }
}
impl Debug for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parents = self
            .supported_by
            .iter()
            .map(|brick| brick.borrow().get_name())
            .collect::<Vec<_>>();
        let children = self
            .supports
            .iter()
            .map(|brick| brick.borrow().get_name())
            .collect::<Vec<_>>();
        f.debug_struct("Brick")
            .field("id", &self.get_name())
            .field("v1", &self.v1)
            .field("v2", &self.v2)
            .field("supported_by", &parents)
            .field("supports", &children)
            .finish()
    }
}

#[derive(Clone, Default)]
pub struct BrickNode(Rc<RefCell<Brick>>);
impl BrickNode {
    pub fn new(id: usize, v1: Coord, v2: Coord) -> Self {
        Self(Rc::new(RefCell::new(Brick::new(id, v1, v2))))
    }
    pub fn borrow(&'_ self) -> cell::Ref<'_, Brick> {
        self.0.borrow()
    }
    pub fn borrow_mut(&'_ self) -> cell::RefMut<'_, Brick> {
        self.0.as_ref().borrow_mut()
    }
}
impl Debug for BrickNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("BrickNode").field(&self.0.borrow()).finish()
    }
}
impl Hash for BrickNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.as_ptr().hash(state);
    }
}
impl PartialEq for BrickNode {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_ptr() == other.0.as_ptr()
    }
}
impl Eq for BrickNode {}
pub type BrickList = HashSet<BrickNode>;

pub fn parse(input: &str) -> Result<BrickList, AocError<'_>> {
    let mut id = 0;
    let grid = input
        .lines()
        .enumerate()
        .map(|(lineno, line)| {
            let source = AocSourceChunk::new(line, lineno);
            let (v1_str, v2_str) =
                line.split_once('~')
                    .ok_or_else(|| AocError::InvalidLineError {
                        desc: "expected `~`".to_string(),
                        src: source,
                        span: (0, line.len()).into(),
                        inner: None,
                    })?;
            let v1 = parse_coord(v1_str, source, 0)?;
            let v2 = parse_coord(v2_str, source, v1_str.len() + 1)?;
            let brick_id = id;
            id += 1;
            Ok(BrickNode::new(brick_id, v1, v2))
        })
        .collect::<Result<BrickList, _>>()?;

    Ok(grid)
}

pub fn build_graph(bricks: &BrickList) -> BrickList {
    // !! This graph is based on visibility, not support !!
    // At this stage, brick will be seen as supporting another if there are
    // no other brick in between at that coordinate, i.e. it ignore any gap
    // between bricks. This is because at this point, the brick are not
    // necessarily in contact yet, until we make them fall.
    let max_z = bricks
        .iter()
        .map(|brick| brick.borrow().max_z())
        .max()
        .unwrap();
    println!("Max height: {max_z}");

    let mut ground = BrickList::new();
    let mut coverage = HashMap::<Coord, BrickNode>::new();
    for z in 1..(max_z + 1) {
        bricks
            .iter()
            .filter(|brick| brick.borrow().at_height(z))
            .for_each(|brick_cell| {
                let mut brick = brick_cell.borrow_mut();
                // println!("Processing: {} for height {z}", brick.get_name());
                let (x_min, x_max) = brick.v1.x.minmax(brick.v2.x);
                let (y_min, y_max) = brick.v1.y.minmax(brick.v2.y);
                // println!("  base range: ({},{})x({},{})", x_min, x_max, y_min, y_max);
                for y in y_min..=y_max {
                    for x in x_min..=x_max {
                        // println!("   processing ({},{})", x, y);
                        let coord = Coord { x, y, z: 0 };
                        let mut entry = coverage.entry(coord);
                        match entry {
                            Entry::Occupied(ref mut entry) => {
                                // Create a link between the base brick and
                                // the current brick
                                if brick_cell != entry.get() {
                                    // println!(
                                    //     "Linking {} to {}",
                                    //     brick.get_name(),
                                    //     entry.get().borrow().get_name()
                                    // );
                                    entry.get().borrow_mut().supports.insert(brick_cell.clone());
                                    brick.supported_by.insert(entry.get().clone());
                                    *entry.get_mut() = brick_cell.clone();
                                }
                            }
                            Entry::Vacant(_) => {
                                // println!("   ({x},{y}) is vacant");
                                // No base brick => supported by the ground
                                ground.insert(brick_cell.clone());
                                entry.or_insert(brick_cell.clone());
                            }
                        };
                    }
                }
            });
    }

    // Cause all the brick to fall
    fall(&ground);

    // Now remove the link we there is no actual contact between bricks
    clean_support(bricks);

    ground
}

pub fn fall(graph: &BrickList) {
    // Move all the brick to the ground
    let mut pending = graph.iter().cloned().collect::<VecDeque<_>>();
    while let Some(brick) = pending.pop_front() {
        let mut brick = brick.borrow_mut();
        let height = (brick.v1.z - brick.v2.z).abs();
        brick.v1.z = 0;
        brick.v2.z = height;

        // Add its children to the list of pending bricks, unless they are
        // already moved
        pending.extend(
            brick
                .supports
                .iter()
                .filter(|brick| brick.borrow().v1.z > 0)
                .cloned(),
        );
    }

    // Move the brick back up based on what bricks are directly below
    let mut pending = graph
        .iter()
        .map(|brick| (brick.clone(), 1))
        .collect::<VecDeque<_>>();
    while let Some((brick, z)) = pending.pop_front() {
        let mut brick = brick.borrow_mut();
        let (min_z, max_z) = brick.minmax_z();
        if min_z >= z {
            // Already at the right height
            continue;
        }
        brick.v1.z = z;
        brick.v2.z = z + (max_z - min_z);

        let child_z = brick.v2.z + 1;
        pending.extend(brick.supports.iter().map(|brick| (brick.clone(), child_z)));
    }
}

fn clean_support(bricks: &BrickList) {
    bricks.iter().for_each(|brick_cell| {
        let mut brick = brick_cell.borrow_mut();
        let need_z = brick.max_z() + 1;
        let parent_z = brick.min_z() - 1;
        brick.supports = brick
            .supports
            .iter()
            .filter(|brick| brick.borrow().min_z() == need_z)
            .cloned()
            .collect();
        brick.supported_by = brick
            .supported_by
            .iter()
            .filter(|brick| brick.borrow().max_z() == parent_z)
            .cloned()
            .collect();
    });
}

pub fn parse_coord<'a>(
    input: &'a str,
    source: AocSourceChunk<'a>,
    colno: usize,
) -> Result<Coord, AocError<'a>> {
    let axis = input.splitn(4, ',').collect::<Vec<_>>();
    if axis.len() != 3 {
        return Err(AocError::InvalidLineError {
            desc: format!("invalid number of fields: got {}, expected 3", axis.len()),
            src: source,
            span: (colno, input.len()).into(),
            inner: None,
        });
    }
    let x = axis[0].parse().map_err(|err| AocError::InvalidNumber {
        src: source,
        span: (colno, axis[0].len()).into(),
        inner: Some(Box::new(err)),
    })?;
    let y = axis[1].parse().map_err(|err| AocError::InvalidNumber {
        src: source,
        span: (colno + axis[0].len() + 1, axis[1].len()).into(),
        inner: Some(Box::new(err)),
    })?;
    let z = axis[2].parse().map_err(|err| AocError::InvalidNumber {
        src: source,
        span: (colno + axis[0].len() + 1 + axis[1].len() + 1, axis[2].len()).into(),
        inner: Some(Box::new(err)),
    })?;

    Ok(Coord { x, y, z })
}
