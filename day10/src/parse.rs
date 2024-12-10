use common::{error::AdventError, position::Position};

pub struct Grid {
    data: Vec<u8>,
    width: usize,
    height: usize,
}
impl Grid {
    pub fn new(data: Vec<u8>, width: usize, height: usize) -> Self {
        assert_eq!(data.len(), width * height);
        Self {
            data,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, position: Position) -> Option<u8> {
        if !(0..self.width).contains(&position.x) {
            return None;
        }
        if !(0..self.height).contains(&position.y) {
            return None;
        }
        Some(self.data[position.to_index(self.width)])
    }
}

pub fn parse(content: &[u8]) -> Result<Grid, AdventError> {
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
        .flat_map(|l| {
            l.iter().map(|b| match b {
                b'0'..=b'9' => b - b'0',
                b'.' => u8::MAX, // for the examples only
                _ => panic!("Invalid character"),
            })
        })
        .collect();
    Ok(Grid::new(data, width, height))
}
