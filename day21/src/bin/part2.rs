use std::collections::HashSet;

use day21::{progress::Progress, *};
use miette;

fn main() -> miette::Result<()> {
    let input = include_str!("input.txt");
    let (grid, width) = get_grid(input)?;

    // There is a diamond shape occupying the whole grid (i.e. once centered
    // at distance `width/2`). At that diamond, all the cells are at that
    // `width/2` distance (i.e. the same as if there was no rocks).
    //
    // With the grid repeating, it means that we'll get the same diamond at
    // each grid interval (i.e. `[width/2, width/2+width, width/2+2*width, ...]`)
    // i.e. at `width/2 + x * width`
    //
    // It also means that the increase in number of reachable gardens should
    // increment predictably, and more specifically quadratically (2D shape)
    //
    // With a simple `y = a*x^2 + b*x + c` and 3 data points (x being the
    // diamond number, i.e. 0 for `width/2`, 1 for `width/2+width`, and 2
    // for `width/2+2*width`), and y being the number of reachable gardens, we
    // get:
    //    a = (y1 - 2*y2 + y3) /2
    //    b = - y1 + y2 - a
    //    c = y1
    let y = count_reachable(&grid, width, width / 2);
    println!("y: {y:?}");

    let a = (y[0] - 2 * y[1] + y[2]) / 2;
    let b = -y[0] + y[1] - a;
    let c = y[0];
    println!("c: {a} - {b} - {c}");

    let func = |x| a * x * x + b * x + c;

    assert_eq!(y[0], func(0));
    assert_eq!(y[1], func(1));
    assert_eq!(y[2], func(2));
    assert_eq!(y[3], func(3));

    let x = (26501365 - (width / 2)) / width;
    assert_eq!(x * width + width / 2, 26501365);
    let output = func(x);
    dbg!(output);
    Ok(())
}

fn get_grid(input: &str) -> Result<(Grid, isize), AocError> {
    let (start, mut grid) = parse(input)?;

    let width = input.lines().next().unwrap().len() as isize;

    assert_eq!(start.x, width / 2);
    assert_eq!(start.y, width / 2);

    // Note that the x=0 and y=0 are all gardens
    // => This means that the shortest path to every grid the nearest corner
    // and it's always at distance `x+y` (if x and y are the coordinates of
    // that nearest corner).
    // => It also means that all the grids will be more or less identical
    assert_eq!(
        0,
        grid.iter()
            .filter(|(coord, cell)| {
                ((coord.x == 0) || (coord.y == 0)) && !matches!(cell, Cell::Garden(_))
            })
            .count(),
    );

    // To simplify, "rotate" the grid so that the start is in (0,0) and all
    // the grid points are positive
    grid = grid
        .into_iter()
        .map(|(coord, cell)| {
            let coord = Coord::new(
                (coord.x - start.x).rem_euclid(width),
                (coord.y - start.y).rem_euclid(width),
            );
            (coord, cell)
        })
        .collect();

    //Ok(count_reachable(&grid, width, steps))
    Ok((grid, width))
}

fn count_reachable(grid: &Grid, width: isize, init_steps: isize) -> Vec<isize> {
    let mut pending = HashSet::from([Coord::default()]);
    // println!("== Step 0");
    // print_result(&grid, &pending, width);
    // println!("");

    let total_steps = init_steps + 3 * width;
    let mut count_vec = Vec::new();
    let mut progress = Progress::new(total_steps + 1);
    for i in 1..=total_steps {
        progress.val(i);

        pending = pending
            .into_iter()
            .map(|coord| {
                [
                    coord + Coord::new(-1, 0),
                    coord + Coord::new(1, 0),
                    coord + Coord::new(0, -1),
                    coord + Coord::new(0, 1),
                ]
                .into_iter()
                .filter_map(|coord| {
                    let rem_coord =
                        Coord::new(coord.x.rem_euclid(width), coord.y.rem_euclid(width));
                    match grid.get(&rem_coord) {
                        Some(cell) => match cell {
                            Cell::Rock => None,
                            Cell::Garden(_) => Some(coord),
                        },
                        _ => unreachable!("should always be valid"),
                    }
                })
            })
            .flatten()
            .collect();

        if (i - init_steps) % width == 0 {
            println!("   @{i} ({}) = {}", (i - init_steps) / width, pending.len());
            count_vec.push(pending.len() as isize);
        }

        // println!("== Step {i}");
        // print_result(&grid, &pending, width);
        // println!("");
    }
    progress.finish();

    println!("Counts {:?}", count_vec);

    count_vec
}

#[allow(unused)]
fn print_result(grid: &Grid, pending: &HashSet<Coord>, width: isize) {
    let (x_range, y_range) = pending
        .iter()
        .fold((0..0, 0..0), |(x_range, y_range), coord| {
            (
                (x_range.start.min(coord.x))..(x_range.end.max(coord.x + 1)),
                (y_range.start.min(coord.y))..(x_range.end.max(coord.y + 1)),
            )
        });
    for y in (y_range.start - 4)..(y_range.end + 4) {
        for x in (x_range.start - 4)..(x_range.end + 4) {
            match pending.get(&Coord { x, y }) {
                Some(_) => print!("O"),
                None => {
                    let rem_coord = Coord::new(x.rem_euclid(width), y.rem_euclid(width));
                    match grid.get(&rem_coord) {
                        Some(Cell::Rock) => print!("#"),
                        Some(Cell::Garden(_)) => print!("."),
                        _ => unreachable!("should always be valid"),
                    }
                }
            }
        }
        println!("");
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
        let (grid, width) = get_grid(input)?;
        assert_eq!(count_reachable(&grid, width, 6)[0], 16);
        // assert_eq!(process(input, 10).unwrap(), 50);
        // assert_eq!(process(input, 50).unwrap(), 1594);
        // assert_eq!(process(input, 100).unwrap(), 6536);
        // assert_eq!(process(input, 500).unwrap(), 167004);
        // assert_eq!(process(input, 1000).unwrap(), 668697);
        // assert_eq!(process(input, 5000).unwrap(), 16733044);
        Ok(())
    }
}
