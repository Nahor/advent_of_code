use miette::Result;

use crate::parse::parse;

pub fn run(content: &str) -> Result<u64> {
    let data = parse(content)?;

    let line_count = data.len() as i32;
    assert!(line_count > 0);
    let line_len = data[0].len() as i32;
    assert!(data.iter().all(|line| line.len() as i32 == line_len));

    let mut result = 0;
    for y in 1..(line_count - 1) {
        for x in 1..(line_len - 1) {
            if data[y as usize][x as usize] != 'A' {
                continue;
            }

            const DIAG_V: u32 = 'M' as u32 + 'S' as u32;
            let diag_1 = data[(y - 1) as usize][(x - 1) as usize] as u32
                + data[(y + 1) as usize][(x + 1) as usize] as u32;
            let diag_2 = data[(y + 1) as usize][(x - 1) as usize] as u32
                + data[(y - 1) as usize][(x + 1) as usize] as u32;
            if (diag_1 == DIAG_V) && (diag_2 == DIAG_V) {
                result += 1;
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &str = r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 9);
    }

    // #[test]
    // fn sample_sorted() {
    //     assert_eq!(
    //         run_sorted(&INPUT_SAMPLE[1..]).unwrap(),
    //         run(&INPUT_SAMPLE[1..]).unwrap()
    //     );

    //     let input = read_input(None).unwrap();
    //     assert_eq!(run_sorted(&input).unwrap(), run(&input).unwrap());
    // }
}
