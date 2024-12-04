// Same as part2_u8 but use a 1D array instead of a 2D to avoid the
// double-indirection (data->line->column) and corresponding multiplications

use miette::{diagnostic, Context, Result};

pub fn run(content: &[u8]) -> Result<u64> {
    let line_len = content
        .iter()
        .position(|b| *b == b'\n')
        .wrap_err(diagnostic!("Invalid file"))?
        + 1;

    assert_eq!(content.len().rem_euclid(line_len), 0);

    let sift_l = line_len + 1; // up-left->down_right diagonal
    let sift_r = line_len - 1; // up-right->down_left diagonal

    let mut result = 0;
    for pos in sift_l..(content.len() - sift_l) {
        if content[pos] != b'A' {
            continue;
        }

        const DIAG_V: u8 = b'M' + b'S';
        let diag_l = content[pos - sift_l] + content[pos + sift_l];
        let diag_r = content[pos - sift_r] + content[pos + sift_r];
        if (diag_l == DIAG_V) && (diag_r == DIAG_V) {
            result += 1;
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
    fn compare_u8_linear() {
        assert_eq!(
            run(&INPUT_SAMPLE[1..]).unwrap(),
            part2::run(std::str::from_utf8(&INPUT_SAMPLE[1..]).unwrap()).unwrap()
        );

        let input_u8 = read_input_u8(None).unwrap();
        let input_str = read_input_str(None).unwrap();
        assert_eq!(run(&input_u8).unwrap(), part2::run(&input_str).unwrap());
    }
}
