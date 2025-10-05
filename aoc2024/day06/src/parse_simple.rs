use std::collections::HashSet;

use common::error::AdventError;

use crate::document::IPosition;

pub fn parse(input: &[u8]) -> Result<(HashSet<IPosition>, IPosition), AdventError> {
    let mut start = Default::default();
    Ok((
        input
            .split(|b| *b == b'\n')
            .enumerate()
            .filter(|(_, l)| !l.is_empty())
            .flat_map(|(y, line)| {
                line.iter().enumerate().filter_map(move |(x, b)| {
                    if *b == b'^' || *b == b'#' {
                        Some((
                            IPosition {
                                x: x as isize,
                                y: y as isize,
                            },
                            b,
                        ))
                    } else {
                        None
                    }
                })
            })
            .filter_map(|(pos, b)| {
                if *b == b'^' {
                    start = pos;
                    None
                } else {
                    Some(pos)
                }
            })
            .collect(),
        start,
    ))
}
