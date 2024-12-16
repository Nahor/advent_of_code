#[cfg(not(feature = "petegraph"))]
use std::collections::{HashMap, VecDeque};

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
    // print(&maze);

    let condensed = condense(&maze, false);

    let output = petegraph(&maze, &condensed);
    Ok(output)
}

#[cfg(not(feature = "petegraph"))]
fn process(input: &str) -> Result<usize, AocError> {
    let maze = parse(input)?;
    // print(&maze);

    let condensed = condense(&maze, false);

    let mut nodes_cost = HashMap::<Coord, usize>::new();
    let mut pending = VecDeque::from([(vec![maze.start], 0)]);
    let mut processed_count = 0;
    while let Some((path, cost)) = pending.pop_front() {
        processed_count += 1;
        // println!(
        //     "processing path (len {}, cost {}): {path:?}",
        //     path.len(),
        //     cost
        // );
        let coord = *path.last().unwrap();
        nodes_cost
            .entry(coord)
            .and_modify(|node_cost| *node_cost = cost.max(*node_cost))
            .or_insert(cost);

        condensed
            .get(&coord)
            .unwrap()
            .edges
            .iter()
            .for_each(|(next_coord, segment_cost)| {
                if !path.contains(next_coord) {
                    let mut path = path.clone();
                    path.push(*next_coord);
                    pending.push_front((path, cost + segment_cost))
                }
            });
    }
    println!("Number of iterations: {processed_count}");

    // -1 because we want the number of steps, not the number of cells
    let output = *nodes_cost.get(&maze.end).unwrap();

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

// fn can_remove(brick: &Brick) -> bool {
//     brick.supported_by.iter().filter(predicate)
// }

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
        assert_eq!(process(input)?, 154);

        Ok(())
    }
}
