use miette::Result;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::{Cell, parse::parse};

#[rustfmt::skip]
pub fn kernel(x: isize, y: isize) -> [(isize, isize); 8] {
    [
        (x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
        (x - 1, y),                 (x + 1, y),
        (x - 1, y + 1), (x, y + 1), (x + 1, y + 1),
    ]
}

pub fn run(content: &[u8]) -> Result<usize> {
    let grid = parse(content)?;

    // position => neighbors
    let mut map = FxHashMap::default();
    // neighbors => position (we only care for position with less than 4 neighbors)
    let mut neighbors = std::array::from_fn::<_, 4, _>(|_| FxHashSet::default());

    (0..grid.lines()).for_each(|y| {
        (0..grid.columns()).for_each(|x| {
            if !matches!(grid.get(x, y), Some(Cell::Roll)) {
                return;
            }

            let count = kernel(x, y)
                .into_iter()
                .filter_map(|(x2, y2)| {
                    grid.get(x2, y2).and_then(|cell| match cell {
                        crate::Cell::Empty => None,
                        crate::Cell::Roll => Some(()),
                    })
                })
                .count();
            map.insert((x, y), count);
            if count < 4 {
                neighbors[count].insert((x, y));
            }
        });
    });

    let start_count = map.len();
    loop {
        // Get position of a removable roll
        let Some((x, y)) = neighbors
            .iter_mut()
            .filter_map(|neighbors| {
                let position = neighbors.iter().next().copied();
                if let Some(elt) = position {
                    neighbors.remove(&elt);
                }
                position
            })
            .next()
        else {
            break;
        };

        // Remove the roll
        map.remove(&(x, y));

        // Update the neighbors
        kernel(x, y).into_iter().for_each(|(x, y)| {
            map.entry((x, y)).and_modify(|count| {
                if *count < 4 {
                    neighbors[*count].remove(&(x, y));
                }
                *count -= 1;
                if *count < 4 {
                    neighbors[*count].insert((x, y));
                }
            });
        });
    }
    let end_count = map.len();

    Ok(start_count - end_count)
}

#[cfg(test)]
mod test {
    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 43);
    }
}
