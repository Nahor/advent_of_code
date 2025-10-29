use day21::*;

fn main() -> miette::Result<()> {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../input/2023/",
        env!("CARGO_PKG_NAME"),
        "/input.txt"
    ));
    let output = process(input, 64)?;
    dbg!(output);
    Ok(())
}

fn process(input: &str, steps: u64) -> Result<usize, AocError> {
    let (start, mut grid) = parse(input)?;

    let mut pending = vec![start];
    for i in 0..=steps {
        pending = pending
            .into_iter()
            .filter_map(|coord| match grid.get_mut(&coord) {
                Some(cell) => match cell {
                    Cell::Rock => None,
                    Cell::Garden(steps) => match steps {
                        Some(_) => None,
                        None => {
                            *steps = Some(i);
                            Some(vec![
                                coord + Coord { x: -1, y: 0 },
                                coord + Coord { x: 1, y: 0 },
                                coord + Coord { x: 0, y: -1 },
                                coord + Coord { x: 0, y: 1 },
                            ])
                        }
                    },
                },
                None => None,
            })
            .flatten()
            .collect();

        // println!("Grid after step {i}:");
        // print_grid(&grid);
    }

    let output = grid
        .iter()
        .filter(|(_, cell)| match cell {
            Cell::Garden(Some(count)) => (count % 2) == (steps % 2),
            _ => false,
        })
        .count();

    Ok(output)
}

#[allow(unused)]
fn print_grid(grid: &Grid) {
    'y: for y in 0.. {
        let mut has_y = false;
        'x: for x in 0.. {
            match grid.get(&Coord { x, y }) {
                Some(Cell::Rock) => {
                    has_y = true;
                    print!("#");
                }
                Some(Cell::Garden(Some(count))) if count % 2 == 0 => {
                    has_y = true;
                    print!("O")
                }
                Some(Cell::Garden(Some(count))) => {
                    has_y = true;
                    print!("o")
                }
                Some(Cell::Garden(None)) => {
                    has_y = true;
                    print!(".")
                }
                None if has_y => break 'x,
                None => break 'y,
            };
        }
        println!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() -> miette::Result<()> {
        let input = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";
        assert_eq!(process(input, 6).unwrap(), 16);

        Ok(())
    }
}
