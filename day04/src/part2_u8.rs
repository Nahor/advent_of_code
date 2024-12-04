// Same as part2 but use `[u8]` rather than `str`

use miette::Result;

pub fn run(content: &[u8]) -> Result<u64> {
    let data = content
        .split(|b| *b == b'\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let line_count = data.len() as i32;
    assert!(line_count > 0);
    let line_len = data[0].len() as i32;
    assert!(data.iter().all(|line| line.len() as i32 == line_len));

    let mut result = 0;
    for y in 1..(line_count - 1) {
        for x in 1..(line_len - 1) {
            if data[y as usize][x as usize] != b'A' {
                continue;
            }

            const DIAG_V: u8 = b'M' + b'S';
            let diag_1 =
                data[(y - 1) as usize][(x - 1) as usize] + data[(y + 1) as usize][(x + 1) as usize];
            let diag_2 =
                data[(y + 1) as usize][(x - 1) as usize] + data[(y - 1) as usize][(x + 1) as usize];
            if (diag_1 == DIAG_V) && (diag_2 == DIAG_V) {
                result += 1;
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use crate::{
        input::{read_input_str, read_input_u8},
        part2,
    };

    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"
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
    fn compare_u8() {
        assert_eq!(
            run(&INPUT_SAMPLE[1..]).unwrap(),
            part2::run(std::str::from_utf8(&INPUT_SAMPLE[1..]).unwrap()).unwrap()
        );

        let input_u8 = read_input_u8(None).unwrap();
        let input_str = read_input_str(None).unwrap();
        assert_eq!(run(&input_u8).unwrap(), part2::run(&input_str).unwrap());
    }
}
