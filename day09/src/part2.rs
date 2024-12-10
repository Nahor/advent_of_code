use miette::Result;

use crate::parse::{parse, File};

pub fn run(content: &[u8]) -> Result<u64> {
    let mut drive = parse(content)?;

    let mut index = drive.len() - 1;
    loop {
        let file = drive.get(index).copied().unwrap();

        if let Some(to_index) = drive[0..=index]
            .windows(2)
            .position(|files| files[1].offset - (files[0].offset + files[0].length) >= file.length)
        {
            // Found a spot
            let new_offset = drive[to_index].offset + drive[to_index].length;

            let mut file = drive.remove(index);
            file.offset = new_offset;
            drive.insert(to_index + 1, file);
        } else {
            // Couldn't find a spot for that file, leave it alone
            if index == 0 {
                break;
            }
            index -= 1;
        }
    }

    // print_drive(&drive);

    let result: usize = drive
        .into_iter()
        .map(|file| (file.offset * file.length + (file.length - 1) * file.length / 2) * file.id)
        .sum();

    Ok(result as u64)
}

#[allow(dead_code)]
fn print_drive(compacted: &Vec<File>) {
    let mut last_offset = 0;
    for file in compacted {
        let next_offset = file.offset + file.length;
        print!(
            "{number:.>width$}",
            number = String::from_utf8_lossy(vec![(file.id as u8) + b'0'; file.length].as_slice()),
            width = next_offset - last_offset
        );
        last_offset = next_offset;
    }
    println!();
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

        assert_eq!(run(input).unwrap(), 2858);
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
