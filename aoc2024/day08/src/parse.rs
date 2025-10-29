use std::{collections::HashMap, ops::Range};

use common::error::AdventError;

pub type Coord = (isize, isize);
pub type Bounds = (Range<isize>, Range<isize>);
pub type Frequency = u8;
pub type Antennas = HashMap<Frequency, Vec<Coord>>;
pub type Data = (Antennas, Bounds);

pub fn parse(content: &[u8]) -> Result<Data, AdventError> {
    let lines: Vec<_> = content
        .split(|b| *b == b'\n')
        .take_while(|l| !l.is_empty())
        .collect();

    let bounds = (0..(lines[0].len() as isize), 0..(lines.len() as isize));

    let antennas: Antennas = lines
        .into_iter()
        .enumerate()
        .flat_map(move |(y, l)| {
            l.iter().enumerate().map(move |(x, b)| {
                if *b == b'.' {
                    None
                } else {
                    Some(((x as isize, y as isize), *b))
                }
            })
        })
        .flatten()
        .fold(Antennas::new(), |mut antennas, (coord, frequency)| {
            antennas.entry(frequency).or_default().push(coord);
            antennas
        });

    Ok((antennas, bounds))
}
