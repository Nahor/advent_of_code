/// Same as part2 but without a heuristic (i.e. dijkstra algorithm)
use common::position::{Direction, Grid, Position};
use itertools::Itertools;
use miette::{miette, Result};
use pathfinding::prelude::astar_bag_collect;

use crate::parse::{parse, Cell};

const COST_ROTATION: usize = 1000;
const COST_TRANSLATION: usize = 1;

pub fn run(content: &[u8]) -> Result<u64> {
    let (grid, start, end) = parse(content)?;

    let (paths, _) = astar_bag_collect(
        &(start, Direction::Right),
        |state| successors(state, &grid),
        |state| heuristic(state, end),
        |&(pos, _)| pos == end,
    )
    .ok_or_else(|| miette!("no path to end"))?;

    let result = paths.iter().flatten().map(|(pos, _)| pos).unique().count();

    Ok(result as u64)
}

fn successors(
    &(pos, direction): &(Position, Direction),
    grid: &Grid<Cell>,
) -> Vec<((Position, Direction), usize)> {
    let mut successors = Vec::with_capacity(3);
    if grid
        .get(pos + direction)
        .is_some_and(|cell| cell == Cell::Empty)
    {
        successors.push(((pos + direction, direction), COST_TRANSLATION));
    }
    if grid
        .get(pos + direction.left())
        .is_some_and(|cell| cell == Cell::Empty)
    {
        successors.push((
            (pos + direction.left(), direction.left()),
            COST_ROTATION + COST_TRANSLATION,
        ));
    }
    if grid
        .get(pos + direction.right())
        .is_some_and(|cell| cell == Cell::Empty)
    {
        successors.push((
            (pos + direction.right(), direction.right()),
            COST_ROTATION + COST_TRANSLATION,
        ));
    }
    successors
}

fn heuristic(&(_, _): &(Position, Direction), _: Position) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    // cspell:disable
    const INPUT_SAMPLE_1: &[u8] = br#"
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
"#;

    const INPUT_SAMPLE_2: &[u8] = br#"
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
"#;
    // cspell:enable

    #[test]
    fn sample1() {
        let input = &INPUT_SAMPLE_1[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 45);
    }

    #[test]
    fn sample2() {
        let input = &INPUT_SAMPLE_2[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 64);
    }

    #[test]
    fn compare_base() {
        assert_eq!(
            run(&INPUT_SAMPLE_1[1..]).unwrap(),
            crate::part2::run(&INPUT_SAMPLE_1[1..]).unwrap()
        );
        assert_eq!(
            run(&INPUT_SAMPLE_2[1..]).unwrap(),
            crate::part2::run(&INPUT_SAMPLE_2[1..]).unwrap()
        );

        let input = common::read_input_u8!(None).unwrap();
        assert_eq!(run(&input).unwrap(), crate::part2::run(&input).unwrap());
    }
}
