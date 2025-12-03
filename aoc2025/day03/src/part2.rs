use miette::Result;

use crate::parse::parse;

pub fn process_line(line: &[u8], nth: u32) -> u64 {
    let nth = nth - 1;
    let (idx, &v) = line[0..line.len() - nth as usize]
        .iter()
        .enumerate()
        .rev() // reverse because max_by returns the last greater number but we want the first
        .max_by_key(|(_, v1)| *v1)
        .unwrap();
    if nth > 0 {
        v as u64 * 10_u64.pow(nth) + process_line(&line[(idx + 1)..], nth)
    } else {
        v as u64
    }
}

pub fn run(content: &[u8]) -> Result<u64> {
    let lines = parse(content)?;

    let result = lines
        .into_iter()
        .map(|line| {
            assert!(line.len() >= 12);
            process_line(line.as_slice(), 12)
        })
        .sum();

    Ok(result)
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"
987654321111111
811111111111119
234234234234278
818181911112111
"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 3121910778619);
    }

    #[rstest]
    #[case(b"987654321111111", 987654321111)]
    #[case(b"811111111111119", 811111111119)]
    #[case(b"234234234234278", 434234234278)]
    #[case(b"818181911112111", 888911112111)]
    fn sample_single(#[case] input: &[u8], #[case] expected: u64) {
        assert_eq!(run(input).unwrap(), expected);
    }
}
