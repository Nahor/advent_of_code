use std::iter::successors;

use common::error::AdventError;
use rustc_hash::{FxHashMap, FxHashSet};

pub fn parse(content: &[u8]) -> Result<FxHashMap<(isize, isize), usize>, AdventError> {
    let mut start = (0, 0);
    let mut end = (0, 0);

    let path = content
        .split(|b| *b == b'\n')
        .enumerate()
        .flat_map(|(y, l)| {
            let v = l
                .iter()
                .enumerate()
                .filter_map(|(x, c)| match c {
                    b'#' => None,
                    b'.' => Some((x as isize, y as isize)),
                    b'S' => {
                        start = (x as isize, y as isize);
                        Some(start)
                    }
                    b'E' => {
                        end = (x as isize, y as isize);
                        Some(end)
                    }
                    _ => panic!("unexpected char {}", *c as char),
                })
                .collect::<Vec<_>>();
            v
        })
        .collect::<FxHashSet<_>>();

    let path = successors(Some((start, start)), |(prev, cur)| {
        if *cur == end {
            return None;
        }
        let next = [(0, -1), (1, 0), (0, 1), (-1, 0)]
            .iter()
            .map(|dir| (cur.0 + dir.0, cur.1 + dir.1))
            .find(|pos| (pos != prev) && path.contains(pos))
            .unwrap_or_else(|| panic!("expected a next position after {cur:?}"));
        Some((*cur, next))
    })
    .enumerate()
    .map(|(i, (_, pos))| (pos, i))
    .collect::<FxHashMap<_, _>>();

    Ok(path)
}

pub fn parse_ordered_vec(content: &[u8]) -> Result<Vec<(isize, isize)>, AdventError> {
    let mut start = (0, 0);
    let mut end = (0, 0);

    let path = content
        .split(|b| *b == b'\n')
        .enumerate()
        .flat_map(|(y, l)| {
            let v = l
                .iter()
                .enumerate()
                .filter_map(|(x, c)| match c {
                    b'#' => None,
                    b'.' => Some((x as isize, y as isize)),
                    b'S' => {
                        start = (x as isize, y as isize);
                        Some(start)
                    }
                    b'E' => {
                        end = (x as isize, y as isize);
                        Some(end)
                    }
                    _ => panic!("unexpected char {}", *c as char),
                })
                .collect::<Vec<_>>();
            v
        })
        .collect::<FxHashSet<_>>();

    let path = successors(Some((start, start)), |(prev, cur)| {
        if *cur == end {
            return None;
        }
        let next = [(0, -1), (1, 0), (0, 1), (-1, 0)]
            .iter()
            .map(|dir| (cur.0 + dir.0, cur.1 + dir.1))
            .find(|pos| (pos != prev) && path.contains(pos))
            .unwrap_or_else(|| panic!("expected a next position after {cur:?}"));
        Some((*cur, next))
    })
    .map(|(_, cur)| cur)
    .collect::<Vec<_>>();

    Ok(path)
}
