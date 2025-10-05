use common::{
    error::AdventError,
    position::{Direction, Grid},
};
use winnow::{
    ascii::line_ending,
    combinator::{dispatch, empty, fail, repeat, rest, separated_pair, terminated, trace},
    prelude::*,
    token::any,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    #[default]
    Empty,
    Wall,
    Box,
    Robot,
}
impl From<Cell> for &str {
    fn from(value: Cell) -> Self {
        match value {
            Cell::Empty => ".",
            Cell::Wall => "#",
            Cell::Box => "O",
            Cell::Robot => "@",
        }
    }
}

pub fn parse(content: &[u8]) -> Result<(Grid<Cell>, Vec<Direction>), AdventError> {
    Ok(trace(
        "parser",
        separated_pair(grid_parser, line_ending, move_parser),
    )
    .parse(content)?)
}

pub fn parse_grid(content: &[u8]) -> Result<Grid<Cell>, AdventError> {
    Ok(trace("parse_grid", terminated(grid_parser, rest)).parse(content)?)
}

fn grid_parser(input: &mut &[u8]) -> PResult<Grid<Cell>> {
    let lines: Vec<_> = trace(
        "grid_parser",
        repeat(1.., terminated(grid_line_parser, line_ending)),
    )
    .parse_next(input)?;

    let height = lines.len();
    let width = lines[0].len();
    let data = lines.into_iter().flatten().collect();
    Ok(Grid::new(data, width, height))
}

fn grid_line_parser(input: &mut &[u8]) -> PResult<Vec<Cell>> {
    trace("grid_line_parser", repeat(1.., grid_cell_parser)).parse_next(input)
}

fn grid_cell_parser(input: &mut &[u8]) -> PResult<Cell> {
    trace(
        "grid_cell_parser",
        dispatch! {any;
            b'.' => empty.value(Cell::Empty),
            b'#' => empty.value(Cell::Wall),
            b'O' => empty.value(Cell::Box),
            b'@' => empty.value(Cell::Robot),
            _ => fail::<_, Cell, _>,
        },
    )
    .parse_next(input)
}

fn move_parser(input: &mut &[u8]) -> PResult<Vec<Direction>> {
    trace(
        "move_parser",
        repeat(
            1..,
            dispatch! {any;
                b'<' => empty.value(Some(Direction::Left)),
                b'>' => empty.value(Some(Direction::Right)),
                b'^' => empty.value(Some(Direction::Up)),
                b'v' => empty.value(Some(Direction::Down)),
                b'\r' => empty.value(None),
                b'\n' => empty.value(None),
                _ => fail::<_, Option<Direction>, _>,
            },
        )
        .map(|v: Vec<Option<Direction>>| v.into_iter().flatten().collect()),
    )
    .parse_next(input)
}
