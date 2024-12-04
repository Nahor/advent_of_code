use miette::Result;

use crate::parse::parse;

#[derive(Debug, Clone, Copy)]
struct Direction {
    x: i32,
    y: i32,
}
const DIRECTION: [Direction; 8] = [
    Direction { x: -1, y: -1 },
    Direction { x: -1, y: 0 },
    Direction { x: -1, y: 1 },
    Direction { x: 0, y: -1 },
    Direction { x: 0, y: 1 },
    Direction { x: 1, y: -1 },
    Direction { x: 1, y: 0 },
    Direction { x: 1, y: 1 },
];

pub fn run(content: &str) -> Result<usize> {
    let data = parse(content)?;
    let line_count = data.len() as i32;
    assert!(line_count > 0);
    let line_len = data[0].len() as i32;
    assert!(data.iter().all(|line| line.len() as i32 == line_len));

    let mut result = 0;
    for (x, y, dir) in (0..line_count).flat_map(|y| {
        (0..line_len).flat_map(move |x| DIRECTION.iter().cloned().map(move |dir| (x, y, dir)))
    }) {
        const WORD: [char; 4] = ['X', 'M', 'A', 'S'];
        // Check if too near the border that we can't find the whole XMAS word
        if !(0..line_len).contains(&x.wrapping_add((WORD.len() as i32 - 1) * dir.x)) {
            continue;
        }
        if !(0..line_count).contains(&y.wrapping_add((WORD.len() as i32 - 1) * dir.y)) {
            continue;
        }
        let found = WORD.into_iter().enumerate().all(|(idx, letter)| {
            data[(y + dir.y * idx as i32) as usize][(x + dir.x * idx as i32) as usize] == letter
        });
        if found {
            result += 1;
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

        assert_eq!(run(input).unwrap(), 18);
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
