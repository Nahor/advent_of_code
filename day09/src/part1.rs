use miette::Result;

use crate::parse::{parse, File};

pub fn run(content: &[u8]) -> Result<u64> {
    let mut drive = parse(content)?;

    // let mut last_offset = 0;
    // for file in drive {
    //     let next_offset = file.offset + file.length;
    //     print!(
    //         "{number:.>width$}",
    //         number = String::from_utf8_lossy(vec![(file.id as u8) + b'0'; file.length].as_slice()),
    //         width = next_offset - last_offset
    //     );
    //     last_offset = next_offset;
    // }
    // println!();

    let mut compacted = Vec::with_capacity(content.len());
    let mut offset = 0;
    let mut index = 0;
    loop {
        let Some(file) = drive.get(index).copied() else {
            break;
        };

        if offset == file.offset {
            // We got the file for this offset
            compacted.push(file);
            offset += file.length;
            index += 1;
        } else {
            // Fill the free space with files from the end

            // We should be able to at least pop ourself
            let mut end_file = drive.pop().expect("No file in drive");

            // Move what we can,
            let to_move = end_file.length.min(file.offset - offset);
            compacted.push(File {
                id: end_file.id,
                offset,
                length: to_move,
            });
            end_file.length -= to_move;
            if end_file.length > 0 {
                drive.push(end_file);
            }
            offset += to_move;
        }
    }

    // let mut last_offset = 0;
    // for file in &compacted {
    //     let next_offset = file.offset + file.length;
    //     assert_eq!(file.offset, last_offset);
    //     print!(
    //         "{number:.>width$}",
    //         number = String::from_utf8_lossy(vec![(file.id as u8) + b'0'; file.length].as_slice()),
    //         width = next_offset - last_offset
    //     );
    //     last_offset = next_offset;
    // }
    // println!();

    let result: usize = compacted
        .into_iter()
        //.map(|file| (file.offset..(file.offset + file.length)).sum::<usize>() * file.id)
        .map(|file| (file.offset * file.length + (file.length - 1) * file.length / 2) * file.id)
        .sum();

    Ok(result as u64)
}

#[cfg(test)]
mod test {
    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"2333133121414131402"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = INPUT_SAMPLE; // remove leading \n

        assert_eq!(run(input).unwrap(), 1928);
    }

    // #[test]
    // fn sample_sorted() {
    //     assert_eq!(
    //         run_sorted(&INPUT_SAMPLE[1..]).unwrap(),
    //         run(&INPUT_SAMPLE[1..]).unwrap()
    //     );

    //     let input = read_input_u8(None).unwrap();
    //     assert_eq!(run_sorted(&input).unwrap(), run(&input).unwrap());
    // }
}
