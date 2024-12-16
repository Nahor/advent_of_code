use std::iter::successors;

use aocerror::AocError;

pub mod aocerror;

pub struct Data {
    pub first: u64,
    pub second: u64,
}

#[derive(Debug)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

pub fn parse(input: &str, age: usize) -> Result<Vec<Coord>, AocError> {
    let mut g_list = input
        .lines()
        .enumerate()
        .filter_map(|(y, line)| {
            let g = line
                .as_bytes()
                .iter()
                .copied()
                .enumerate()
                .filter_map(|(x, b)| {
                    if b == b'#' {
                        Some(Coord { x, y })
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            if g.is_empty() {
                None
            } else {
                Some(g)
            }
        })
        .flatten()
        .collect::<Vec<_>>();

    let width = g_list.iter().map(|g| g.x).max().unwrap();
    let height = g_list.iter().map(|g| g.y).max().unwrap();
    let shift_x = successors(Some((0, 0)), |&(shift, x)| {
        if x >= width {
            None
        } else if g_list.iter().all(|g| g.x != x) {
            Some((shift + age, x + 1))
        } else {
            Some((shift, x + 1))
        }
    })
    .map(|(shift, _)| shift)
    .collect::<Vec<_>>();
    let shift_y = successors(Some((0, 0)), |&(shift, y)| {
        if y >= height {
            None
        } else if g_list.iter().all(|g| g.y != y) {
            Some((shift + age, y + 1))
        } else {
            Some((shift, y + 1))
        }
    })
    .map(|(shift, _)| shift)
    .collect::<Vec<_>>();
    //dbg!(shift_x);
    //dbg!(shift_y);

    g_list.iter_mut().for_each(|g| {
        g.x += shift_x[g.x];
        g.y += shift_y[g.y];
    });

    Ok(g_list)
}

pub fn distance_sum(g_list: Vec<Coord>) -> isize {
    g_list
        .iter()
        .enumerate()
        .flat_map(|(num, coord1)| {
            g_list[num + 1..].iter().map(|coord2| {
                (coord2.x as isize - coord1.x as isize).abs()
                    + (coord2.y as isize - coord1.y as isize).abs()
            })
        })
        .sum()
}
