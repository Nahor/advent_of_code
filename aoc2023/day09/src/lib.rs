use aocerror::{AocError, AocSourceChunk};

pub mod aocerror;

pub struct Data {
    pub first: u64,
    pub second: u64,
}

pub fn parse(input: &str) -> Result<Vec<Vec<i64>>, AocError> {
    let data = input
        .lines()
        .enumerate()
        .map(|(lineno, line)| {
            let mut col = 0;
            line.split(' ')
                .map(|num| {
                    let trimmed = num.trim();
                    let col_start =
                        col + unsafe { trimmed.as_ptr().offset_from(num.as_ptr()) } as usize;

                    col = col + num.len() + 1;

                    (col_start, trimmed)
                })
                .filter(|(_, num)| !num.is_empty())
                .map(|(col, num_str)| {
                    num_str
                        .parse::<i64>()
                        .map_err(|err| AocError::InvalidLineError {
                            desc: "expected i64".to_owned(),
                            src: AocSourceChunk::new(line.to_owned(), lineno),
                            span: (col, num_str.len()).into(),
                            inner: Some(Box::new(err)),
                        })
                })
                .collect::<Result<Vec<_>, AocError>>()
        })
        .collect::<Result<Vec<_>, AocError>>()?;

    Ok(data)
}
