use common::position::{Direction, Grid, Position};
use miette::{miette, Result};
use pathfinding::prelude::astar;

use crate::parse::{parse, Cell};

const COST_ROTATION: usize = 1000;
const COST_TRANSLATION: usize = 1;

pub fn run(content: &[u8]) -> Result<u64> {
    let (grid, start, end) = parse(content)?;
    let (_, cost) = astar(
        &(start, Direction::Right),
        |state| successors(state, &grid),
        |state| heuristic(state, end),
        |&(pos, _)| pos == end,
    )
    .ok_or_else(|| miette!("no path to end"))?;

    Ok(cost as u64)
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

fn heuristic(&(pos, direction): &(Position, Direction), end: Position) -> usize {
    let diff_x = end.x.abs_diff(pos.x);
    let diff_y = end.y.abs_diff(pos.y);
    let mut cost_rotation = 0;
    let need_dir_x = match end.x.cmp(&pos.x) {
        std::cmp::Ordering::Less => Some(Direction::Left),
        std::cmp::Ordering::Equal => None,
        std::cmp::Ordering::Greater => Some(Direction::Right),
    };
    let need_dir_y = match end.y.cmp(&pos.y) {
        std::cmp::Ordering::Less => Some(Direction::Up),
        std::cmp::Ordering::Equal => None,
        std::cmp::Ordering::Greater => Some(Direction::Down),
    };
    if Some(direction) != need_dir_x {
        cost_rotation += COST_ROTATION;
    }
    if need_dir_x == Some(direction.reverse()) && need_dir_y.is_none() {
        // Account for the cost of a U-turn if not already done to match
        // the y direction
        cost_rotation += COST_ROTATION;
    }
    if Some(direction) != need_dir_y {
        cost_rotation += COST_ROTATION;
    }
    if need_dir_y == Some(direction.reverse()) && need_dir_x.is_none() {
        // Account for the cost of a U-turn if not already done to match
        // the x direction
        cost_rotation += COST_ROTATION;
    }
    let cost_translation = (diff_x + diff_y) * COST_TRANSLATION;

    cost_translation + cost_rotation
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

        assert_eq!(run(input).unwrap(), 7036);
    }

    #[test]
    fn sample2() {
        let input = &INPUT_SAMPLE_2[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 11048);
    }

    // #[test]
    // fn compare_base() {
    //     assert_eq!(
    //         run(&INPUT_SAMPLE[1..]).unwrap(),
    //         crate::part1::run(&INPUT_SAMPLE[1..]).unwrap()
    //     );

    //     let input = common::read_input_u8!(None).unwrap();
    //     assert_eq!(run(&input).unwrap(), crate::part1::run(&input).unwrap());
    // }
}
