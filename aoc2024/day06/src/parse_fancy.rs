use common::error::AdventError;

use crate::document::Position;

#[allow(clippy::type_complexity)]
pub fn parse(input: &[u8]) -> Result<(Vec<Vec<usize>>, Vec<Vec<usize>>, Position), AdventError> {
    // My input was 130 lines/columns. Don't set the size yet since the input
    // can be smaller (e.g. sample).
    let mut cols = Vec::with_capacity(130);
    let mut lines = Vec::with_capacity(130);
    let mut start = Position::default();

    input
        .split(|b| *b == b'\n')
        .enumerate()
        .for_each(|(y, line)| {
            if line.is_empty() {
                return;
            }
            if y == 0 {
                // We now know the number of columns, so we can correctly size `cols`
                cols.resize(line.len(), Vec::new());
            }
            lines.push(Vec::new());

            line.iter().enumerate().for_each(|(x, b)| match b {
                b'#' => {
                    cols[x].push(y);
                    lines[y].push(x);
                }
                b'^' => start = Position { x, y },
                b'.' => {}
                _ => panic!("Unexpected char {}", *b as char),
            });
        });

    Ok((cols, lines, start))
}
