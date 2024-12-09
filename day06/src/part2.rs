use std::{collections::HashSet, ops::Range};

use miette::Result;

use crate::{
    document::{Direction, IPosition},
    parse_simple::parse,
};

pub fn run(content: &[u8]) -> Result<u64> {
    let (grid, start) = parse(content)?;

    // Compute the bounds. Assumes there are blockers on the grid boundary
    // (which is true for both the sample and my input).
    // It would be more efficient to do it during parsing, but that complicates
    // the code (more variables and output parameters)
    let mut min_x = isize::MAX;
    let mut max_x = isize::MIN;
    let mut min_y = isize::MAX;
    let mut max_y = isize::MIN;
    for IPosition { x, y } in &grid {
        min_x = min_x.min(*x);
        max_x = max_x.max(*x);
        min_y = min_y.min(*y);
        max_y = max_y.max(*x);
    }
    let range_x = min_x..(max_x + 1);
    let range_y = min_y..(max_y + 1);

    Ok(check(&grid, (range_x, range_y), start, Direction::Up, None))
}

fn check(
    grid: &HashSet<IPosition>,
    range: (Range<isize>, Range<isize>),
    mut position: IPosition,
    mut direction: Direction,
    supplemental: Option<(&HashSet<(IPosition, Direction)>, IPosition)>,
) -> u64 {
    let mut visited = HashSet::new();
    let mut count = 0;
    loop {
        if visited.contains(&(position, direction))
            || supplemental.is_some_and(|previsited| previsited.0.contains(&(position, direction)))
        {
            // Found a loop
            //print_grid(grid, range, &visited, supplemental);
            return count + 1;
        }
        visited.insert((position, direction));

        let next = position + direction;

        if !range.0.contains(&next.x) || !range.1.contains(&next.y) {
            // found the exit
            return count;
        }

        if grid.contains(&next) || supplemental.is_some_and(|blocker| blocker.1 == next) {
            // Path is blocked => turn and try again
            direction = direction.right();
            continue;
        }

        // check if blocking the path would create loop
        // if we can block twice (i.e. supplemental must be None) and we can't
        // block a place we already visited
        if supplemental.is_none()
            && !visited.contains(&(next, Direction::Up))
            && !visited.contains(&(next, Direction::Right))
            && !visited.contains(&(next, Direction::Down))
            && !visited.contains(&(next, Direction::Left))
        {
            count += check(
                grid,
                range.clone(),
                position,
                direction.right(),
                Some((&visited, next)),
            );
        }

        position = next;
    }
}

#[allow(dead_code)]
fn print_grid(
    grid: &HashSet<IPosition>,
    range: (Range<isize>, Range<isize>),
    visited: &HashSet<(IPosition, Direction)>,
    supplemental: Option<(&HashSet<(IPosition, Direction)>, IPosition)>,
) {
    let (range_x, range_y) = range;
    println!("Grid:");
    for y in range_y.clone() {
        print!("    ");
        for x in range_x.clone() {
            let position = IPosition { x, y };
            if grid.contains(&position) {
                print!("#");
            } else if supplemental.is_some_and(|s| s.1 == position) {
                print!("O");
            } else {
                let h = visited.contains(&(position, Direction::Left))
                    || visited.contains(&(position, Direction::Right))
                    || supplemental.is_some_and(|s| {
                        s.0.contains(&(position, Direction::Left))
                            || s.0.contains(&(position, Direction::Right))
                    });
                let v = visited.contains(&(position, Direction::Up))
                    || visited.contains(&(position, Direction::Down))
                    || supplemental.is_some_and(|s| {
                        s.0.contains(&(position, Direction::Up))
                            || s.0.contains(&(position, Direction::Down))
                    });
                match (h, v) {
                    (true, true) => print!("+"),
                    (true, false) => print!("-"),
                    (false, true) => print!("+"),
                    (false, false) => print!("."),
                }
            }
        }
        println!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 6);
    }

    // #[test]
    // fn sample_sorted() {
    //     assert_eq!(
    //         run_sorted(&INPUT_SAMPLE[1..]).unwrap(),
    //         run(&INPUT_SAMPLE[1..]).unwrap()
    //     );

    //     let input = read_input_u8(None).unwrap();
    //     assert_eq!(run_sorted(&input).unwrap(), run(&input).unwrap());
    // }
}
