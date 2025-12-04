use miette::Result;

use crate::{Cell, Grid, parse::parse};

pub fn process(grid: &Grid) -> (Grid, usize) {
    let mut grid2 = grid.clone();

    let result: usize = (0..grid.lines())
        .flat_map(|y| {
            (0..grid.columns()).filter_map(move |x| {
                if !matches!(grid.get(x, y), Some(Cell::Roll)) {
                    return None;
                }

                #[rustfmt::skip]
                let kernel = [
                    (x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
                    (x - 1, y),                 (x + 1, y),
                    (x - 1, y + 1), (x, y + 1), (x + 1, y + 1),
                ];
                let r = kernel
                    .into_iter()
                    .map(|(x, y)| {
                        grid.get(x, y).map_or(0, |cell| match cell {
                            crate::Cell::Empty => 0,
                            crate::Cell::Roll => 1,
                        })
                    })
                    .sum::<u64>();
                Some(((x, y), r))
            })
        })
        .filter(|&(_, roll_count)| roll_count < 4)
        .inspect(|&((x, y), _)| {
            grid2.set(x, y, Cell::Empty);
        })
        .count();

    (grid2, result)
}

pub fn run(content: &[u8]) -> Result<usize> {
    let mut grid = parse(content)?;
    let mut total = 0;
    while let (grid2, count) = process(&grid)
        && count > 0
    {
        grid = grid2;
        total += count;
    }
    Ok(total)
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
