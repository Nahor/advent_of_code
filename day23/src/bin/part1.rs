#[cfg(not(feature = "petegraph"))]
use std::collections::VecDeque;

use day23::*;

fn main() -> miette::Result<()> {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../advent_of_code_input/2023/",
        env!("CARGO_PKG_NAME"),
        "/input.txt"
    ));
    let output = process(input)?;
    dbg!(output);
    Ok(())
}

#[cfg(feature = "petegraph")]
fn process(input: &str) -> Result<usize, AocError> {
    let maze = parse(input)?;
    //print(&maze);

    let condensed = condense(&maze, true);
    let output = petegraph(&maze, &condensed);
    Ok(output)
}

#[cfg(not(feature = "petegraph"))]
fn process(input: &str) -> Result<usize, AocError> {
    let maze = parse(input)?;
    //print(&maze);

    // Find starting coord
    let mut solution = None;
    let mut pending = VecDeque::from([(
        vec![maze.start, maze.start + Direction::South],
        Direction::South,
    )]);
    while let Some((path, dir)) = pending.pop_front() {
        let coord = *path.last().unwrap();
        if coord == maze.end {
            match solution {
                None => solution = Some(path),
                Some(other) if other.len() < path.len() => solution = Some(path),
                _ => {}
            }
            continue;
        }

        let dir_list = match (dir, maze.cells.get(&coord)) {
            (_, None) => continue, // Wall
            (_, Some(Cell::Slope(slope_dir))) if dir == slope_dir.reverse() => continue, // Trying to go up-slope
            (_, Some(Cell::Slope(slope_dir))) => vec![*slope_dir],
            (Direction::North, Some(Cell::Flat)) => {
                vec![Direction::North, Direction::East, Direction::West]
            }
            (Direction::East, Some(Cell::Flat)) => {
                vec![Direction::North, Direction::East, Direction::South]
            }
            (Direction::South, Some(Cell::Flat)) => {
                vec![Direction::East, Direction::South, Direction::West]
            }
            (Direction::West, Some(Cell::Flat)) => {
                vec![Direction::North, Direction::South, Direction::West]
            }
        };

        dir_list.into_iter().for_each(|new_dir| {
            if new_dir.reverse() == dir {
                return;
            }
            let new_coord = coord + new_dir;
            if path.contains(&new_coord) {
                // Going back or found a cycle
                return;
            }
            let mut path = path.clone();
            path.push(coord + new_dir);
            pending.push_front((path, new_dir));
        });
    }

    //print_path(&maze, solution.as_ref().unwrap());

    // -1 because we want the number of steps, not the number of cells
    let output = solution.unwrap().len() - 1;
    // */
    Ok(output)
}

#[allow(unused)]
fn print(maze: &Maze) {
    println!(
        "Maze of size ({:?}, {:?}), start {:?}, end {:?}:",
        maze.range_x, maze.range_y, maze.start, maze.end
    );

    for y in maze.range_y.clone() {
        print!("    ");
        // +1 because we only expect walls in the last column and walls are not
        // added to the maze (i.e. they are not included in the range computation)
        for x in maze.range_x.clone() {
            match maze.cells.get(&Coord::new(x, y)) {
                Some(Cell::Flat) => print!("."),
                Some(Cell::Slope(Direction::North)) => print!("^"),
                Some(Cell::Slope(Direction::East)) => print!(">"),
                Some(Cell::Slope(Direction::South)) => print!("v"),
                Some(Cell::Slope(Direction::West)) => print!("<"),
                None => print!("#"),
            }
        }
        println!();
    }
    println!();
}

#[allow(unused)]
fn print_path(maze: &Maze, path: &[Coord]) {
    println!("Maze ({:?}, {:?}):", maze.range_x, maze.range_y);

    for y in maze.range_y.clone() {
        print!("    ");
        // +1 because we only expect walls in the last column and walls are not
        // added to the maze (i.e. they are not included in the range computation)
        for x in maze.range_x.clone() {
            let coord = Coord::new(x, y);
            match maze.cells.get(&coord) {
                _ if path.contains(&coord) => print!("O"),
                Some(Cell::Flat) => print!("."),
                Some(Cell::Slope(Direction::North)) => print!("^"),
                Some(Cell::Slope(Direction::East)) => print!(">"),
                Some(Cell::Slope(Direction::South)) => print!("v"),
                Some(Cell::Slope(Direction::West)) => print!("<"),
                None => print!("#"),
            }
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() -> miette::Result<()> {
        let input = "\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
";
        assert_eq!(process(input)?, 94);

        Ok(())
    }
}
