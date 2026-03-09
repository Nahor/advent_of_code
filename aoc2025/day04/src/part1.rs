use miette::Result;

use crate::{Cell, parse::parse};

pub fn run(content: &[u8]) -> Result<usize> {
    let grid = parse(content)?;
    let grid = &grid;

    let result: usize = (0..grid.lines())
        .flat_map(|y| {
            (0..grid.columns()).map(move |x| {
                if !matches!(grid.get(x, y), Some(Cell::Roll)) {
                    return u64::MAX;
                }

                #[rustfmt::skip]
                let kernel = [
                    (x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
                    (x - 1, y),                 (x + 1, y),
                    (x - 1, y + 1), (x, y + 1), (x + 1, y + 1),
                ];
                kernel
                    .into_iter()
                    .map(|(x, y)| {
                        grid.get(x, y).map_or(0, |cell| match cell {
                            crate::Cell::Empty => 0,
                            crate::Cell::Roll => 1,
                        })
                    })
                    .sum::<u64>()
            })
        })
        .filter(|&roll_count| roll_count < 4)
        .count();

    Ok(result)
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

        assert_eq!(run(input).unwrap(), 13);
    }
}
