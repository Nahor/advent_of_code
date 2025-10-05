use std::iter::successors;

use common::position::{Direction, Grid, Position};
use miette::{miette, Result};

use crate::parse::{parse, Cell};

pub fn run(content: &[u8]) -> Result<u64> {
    let grid = parse_and_apply(content)?;

    let result: usize = (0..grid.height())
        .flat_map(|y| (0..grid.width()).map(move |x| Position::new(x, y)))
        .enumerate()
        .map(|(idx, position)| {
            if unsafe { grid.get_idx_unchecked(idx) } == Cell::Box {
                position.y * 100 + position.x
            } else {
                0
            }
        })
        .sum();

    Ok(result as u64)
}

fn parse_and_apply(content: &[u8]) -> Result<Grid<Cell>> {
    let (mut grid, moves) = parse(content)?;
    apply_moves(&mut grid, &moves)?;
    Ok(grid)
}

fn apply_moves(grid: &mut Grid<Cell>, moves: &[Direction]) -> Result<()> {
    // Find the robot and replace it with an empty cell
    let width = grid.width();
    let Some(mut position) = grid
        .iter_mut()
        .enumerate()
        .find(|(_, cell)| **cell == Cell::Robot)
        .map(|(idx, cell)| {
            *cell = Cell::Empty;
            Position::from_index(idx, width)
        })
    else {
        return Err(miette!("No starting position"));
    };

    // Apply the move
    moves.iter().copied().for_each(|dir| {
        // New bot position
        let new_position = position + dir;

        // Find the next non-box cell in the direction of the move
        let next_non_box = successors(Some(new_position), |pos| Some(*pos + dir))
            .find(|pos| !grid.get(*pos).is_some_and(|cell| cell == Cell::Box))
            .unwrap();

        // If we found a bunch of boxes, try to move them (just swap the start
        // and end positions)
        if next_non_box != new_position {
            let cell = grid.get_mut(next_non_box).expect("out of grid bound");
            if *cell == Cell::Empty {
                // Swap the cell
                *cell = Cell::Box;
                *grid.get_mut(new_position).expect("out of grid bound") = Cell::Empty;
            }
        }

        if *grid.get_mut(new_position).expect("out of grid bound") != Cell::Empty {
            // New position is not empty => can't move the bot
            return;
        }

        // Set the new bot's position
        position = new_position;
    });

    // Set the bot
    *grid.get_mut(position).unwrap() = Cell::Robot;

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::parse::parse_grid;

    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"#;

    const INPUT_EXAMPLE: &[u8] = br#"
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 10092);
    }

    #[test]
    fn example() {
        let input = &INPUT_EXAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 2028);
    }

    #[test]
    fn grid_example() {
        let input = &INPUT_EXAMPLE[1..]; // remove leading \n
        let result_grid = &br#"
########
#....OO#
##.....#
#.....O#
#.#O@..#
#...O..#
#...O..#
########
"#[1..];
        assert_eq!(
            parse_and_apply(input).unwrap(),
            parse_grid(result_grid).unwrap()
        );
    }

    #[test]
    fn grid_sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n
        let result_grid = &br#"
##########
#.O.O.OOO#
#........#
#OO......#
#OO@.....#
#O#.....O#
#O.....OO#
#O.....OO#
#OO....OO#
##########
"#[1..];

        assert_eq!(
            parse_and_apply(input).unwrap(),
            parse_grid(result_grid).unwrap()
        );
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
