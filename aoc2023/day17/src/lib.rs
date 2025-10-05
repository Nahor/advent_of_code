pub mod aocerror;
pub mod progress;
pub use aocerror::*;

pub type Grid = Vec<Vec<u32>>;

pub fn parse(input: &str) -> Result<Grid, AocError> {
    let grid = input
        .lines()
        .filter(|line| !line.is_empty())
        .enumerate()
        .map(|(lineno, line)| {
            line.chars()
                .enumerate()
                .map(|(charno, c)| {
                    c.to_digit(10).ok_or_else(|| AocError::InvalidLineError {
                        desc: format!("expected digit, got {c}").to_owned(),
                        src: AocSourceChunk::new(line.to_owned(), lineno),
                        span: (charno, 1).into(),
                        inner: None,
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(grid)
}
