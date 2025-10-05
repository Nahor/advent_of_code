use common::position::{Direction, Grid, Position};
use miette::{miette, Result};
use rustc_hash::FxHashSet;

use crate::parse::{parse, Cell};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum CellW {
    #[default]
    Empty,
    Wall,
    BoxL,
    BoxR,
    Robot,
}
impl From<CellW> for &str {
    fn from(value: CellW) -> Self {
        match value {
            CellW::Empty => ".",
            CellW::Wall => "#",
            CellW::BoxL => "[",
            CellW::BoxR => "]",
            CellW::Robot => "@",
        }
    }
}

pub fn run(content: &[u8]) -> Result<u64> {
    let grid = parse_and_apply(content)?;

    let result: usize = (0..grid.height())
        .flat_map(|y| (0..grid.width()).map(move |x| Position::new(x, y)))
        .enumerate()
        .map(|(idx, position)| {
            if unsafe { grid.get_idx_unchecked(idx) } == CellW::BoxL {
                position.y * 100 + position.x
            } else {
                0
            }
        })
        .sum();

    Ok(result as u64)
}

fn parse_and_apply(content: &[u8]) -> Result<Grid<CellW>> {
    let (grid, moves) = parse(content)?;
    let mut grid = remap(grid);
    apply_moves(&mut grid, &moves)?;
    Ok(grid)
}

fn remap(grid: Grid<Cell>) -> Grid<CellW> {
    let width = grid.width();
    let height = grid.height();
    let new_data = grid
        .into_iter()
        .flat_map(|cell| {
            match cell {
                Cell::Empty => [CellW::Empty, CellW::Empty],
                Cell::Wall => [CellW::Wall, CellW::Wall],
                Cell::Box => [CellW::BoxL, CellW::BoxR],
                Cell::Robot => [CellW::Robot, CellW::Empty],
            }
            .into_iter()
        })
        .collect();

    Grid::new(new_data, width * 2, height)
}

fn apply_moves(grid: &mut Grid<CellW>, moves: &[Direction]) -> Result<()> {
    // Find the robot and replace it with an empty cell
    let width = grid.width();
    let Some(mut position) = grid
        .iter_mut()
        .enumerate()
        .find(|(_, cell)| **cell == CellW::Robot)
        .map(|(idx, cell)| {
            *cell = CellW::Empty;
            Position::from_index(idx, width)
        })
    else {
        return Err(miette!("No starting position"));
    };

    // Apply the move
    moves.iter().copied().try_for_each(|dir| {
        // New bot position
        let new_position = position + dir;

        // Get the boxes that need to be moved
        let Some(boxes) = get_boxes(grid, new_position, dir)? else {
            return Ok::<_, miette::Error>(());
        };

        // Sort the boxes so we don't clobber the result when we move them
        let mut boxes = boxes.iter().collect::<Vec<_>>();
        match dir {
            Direction::Up => boxes.sort_by(|a, b| a.y.cmp(&b.y)),
            Direction::Right => boxes.sort_by(|a, b| a.x.cmp(&b.x).reverse()),
            Direction::Down => boxes.sort_by(|a, b| a.y.cmp(&b.y).reverse()),
            Direction::Left => boxes.sort_by(|a, b| a.x.cmp(&b.x)),
        }

        // Move the boxes
        boxes.into_iter().for_each(|&pos| {
            grid.swap(pos, pos + dir);
        });

        position = new_position;

        Ok(())
    })?;

    // Set the bot
    *grid.get_mut(position).unwrap() = CellW::Robot;

    Ok(())
}

fn get_boxes(
    grid: &Grid<CellW>,
    start: Position,
    dir: Direction,
) -> Result<Option<FxHashSet<Position>>> {
    // We only need to store the boxes' left side.
    // (we could further reduce the capacity since the moved boxes should form
    // a triangle of sort. But if the grid is not square, this gets a bit more
    // complicated, so don't bother, the grid is not big enough for that)
    let mut to_check = Vec::with_capacity(grid.size() / 2);
    let mut boxes = FxHashSet::default();
    to_check.push(start);

    while let Some(position) = to_check.pop() {
        if boxes.contains(&position) {
            // already checked
            continue;
        }
        match grid.get(position) {
            Some(CellW::Empty) => {}
            Some(CellW::Wall) => return Ok(None), // can't move
            Some(CellW::BoxL) => {
                boxes.insert(position);
                boxes.insert(position + Direction::Right);
                to_check.push(position + dir);
                to_check.push(position + Direction::Right + dir);
            }
            Some(CellW::BoxR) => {
                boxes.insert(position + Direction::Left);
                boxes.insert(position);
                to_check.push(position + Direction::Left + dir);
                to_check.push(position + dir);
            }
            Some(CellW::Robot) => return Err(miette!("more than one bot")),
            None => return Err(miette!("out of bound")),
        }
    }

    Ok(Some(boxes))
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
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 9021);
    }

    #[test]
    fn sample_moves() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n
        const EXPECTED: &str = r#"
  ####################
  ##[].......[].[][]##
  ##[]...........[].##
  ##[]........[][][]##
  ##[]......[]....[]##
  ##..##......[]....##
  ##..[]............##
  ##..@......[].[][]##
  ##......[][]..[]..##
  ####################
"#;

        let grid = parse_and_apply(input).unwrap();
        let grid_str = format!("{grid:?}").to_string();

        assert_eq!(&grid_str, EXPECTED);
    }

    #[test]
    fn example_moves() {
        let input = &INPUT_EXAMPLE[1..]; // remove leading \n
        const EXPECTED: &str = r#"
  ##############
  ##...[].##..##
  ##...@.[]...##
  ##....[]....##
  ##..........##
  ##..........##
  ##############
"#;

        let grid = parse_and_apply(input).unwrap();
        let grid_str = format!("{grid:?}").to_string();

        assert_eq!(&grid_str, EXPECTED);
    }

    #[test]
    fn example_remap() {
        let input = &INPUT_EXAMPLE[1..]; // remove leading \n
        const EXPECTED: &str = r#"
  ##############
  ##......##..##
  ##..........##
  ##....[][]@.##
  ##....[]....##
  ##..........##
  ##############
"#;

        let grid = parse_grid(input).unwrap();
        let grid = remap(grid);
        let grid_str = format!("{grid:?}").to_string();

        assert_eq!(&grid_str, EXPECTED);
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
