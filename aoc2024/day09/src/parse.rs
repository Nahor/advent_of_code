use common::error::AdventError;

#[derive(Debug, Clone, Copy)]
pub struct File {
    pub id: usize,
    pub offset: usize,
    pub length: usize,
}

pub fn parse(content: &[u8]) -> Result<Vec<File>, AdventError> {
    // assert if the data always finish with a file

    // Ensure we always have a pair of <file, freespace>
    assert_eq!(content.len() % 2, 1);
    let mut content = Vec::from(content);
    content.push(b'0');

    Ok(content
        .chunks_exact(2)
        .enumerate()
        .scan(0, |offset, (id, data)| {
            let file_len = (data[0] - b'0') as usize;
            let free_len = (data[1] - b'0') as usize;

            let file_offset = *offset;
            *offset += file_len + free_len;

            Some(File {
                id,
                offset: file_offset,
                length: file_len,
            })
        })
        .collect::<Vec<_>>())
}
