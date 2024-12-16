use common::{
    error::AdventError,
    position::{Grid, Position},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Wall,
}

impl From<Cell> for &str {
    fn from(value: Cell) -> Self {
        match value {
            Cell::Empty => ".",
            Cell::Wall => "#",
        }
    }
}

pub fn parse(content: &[u8]) -> Result<(Grid<Cell>, Position, Position), AdventError> {
    let mut start = Default::default();
    let mut end = Default::default();
    let mut data = Vec::with_capacity(content.len());
    let mut height = 1;

    content
        .split(|c| *c == b'\n')
        .enumerate()
        .filter(|(_, l)| !l.is_empty())
        .for_each(|(y, l)| {
            height = y;
            l.iter().enumerate().for_each(|(x, c)| match c {
                b'#' => data.push(Cell::Wall),
                b'.' => data.push(Cell::Empty),
                b'\r' => {}
                b'S' => {
                    start = Position { x, y };
                    data.push(Cell::Empty);
                }
                b'E' => {
                    end = Position { x, y };
                    data.push(Cell::Empty);
                }
                _ => panic!("Invalid char at Ln {y}, Col {x}"),
            })
        });
    height += 1;

    let width = data.len() / height;
    assert_eq!(data.len(), width * height);
    Ok((Grid::new(data, width, height), start, end))
}
