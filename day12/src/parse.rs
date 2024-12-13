use common::{error::AdventError, position::Grid};

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub id: u16,
    pub region: u8,
}

pub fn parse(content: &[u8]) -> Result<Grid<Cell>, AdventError> {
    let lines: Vec<_> = content
        .split(|b| *b == b'\n')
        .take_while(|l| !l.is_empty())
        .collect();
    let height = lines.len();
    assert!(height > 0);
    let width = lines[0].len();
    assert!(lines.iter().all(|l| l.len() == width));

    let data: Vec<_> = lines
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.iter().enumerate().map(move |(x, b)| match b {
                b'A'..=b'Z' => Ok(Cell {
                    id: 0,
                    region: b - b'A',
                }),
                _ => Err(AdventError::parse_error_u8(
                    content,
                    y * (width + 1) + x,
                    "Invalid character".to_owned(),
                )),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Grid::new(data, width, height))
}

pub fn parse_u8(content: &[u8]) -> Result<Grid<u8>, AdventError> {
    let lines: Vec<_> = content
        .split(|b| *b == b'\n')
        .take_while(|l| !l.is_empty())
        .collect();
    let height = lines.len();
    assert!(height > 0);
    let width = lines[0].len();
    assert!(lines.iter().all(|l| l.len() == width));

    let data: Vec<_> = lines
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.iter().enumerate().map(move |(x, b)| match b {
                b'A'..=b'Z' => Ok(b - b'A'),
                _ => Err(AdventError::parse_error_u8(
                    content,
                    y * (width + 1) + x,
                    "Invalid character".to_owned(),
                )),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Grid::new(data, width, height))
}
