use miette::Result;

use crate::parse::parse;

pub fn process_line(line: Vec<u8>) -> u64 {
    assert!(line.len() >= 2);

    let (idx1, v1) = line[0..line.len() - 1]
        .iter()
        .enumerate()
        .rev() // reverse because max_by returns the last greater number but we want the first
        .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
        .unwrap();
    let v2 = line[idx1 + 1..line.len()].iter().max().unwrap();

    (v1 * 10_u8 + v2) as u64
}

pub fn run(content: &[u8]) -> Result<u64> {
    let lines = parse(content)?;

    let result = lines.into_iter().map(process_line).sum();

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

        assert_eq!(run(input).unwrap(), 357);
    }

    #[rstest]
    #[case(b"987654321111111", 98)]
    #[case(b"811111111111119", 89)]
    #[case(b"234234234234278", 78)]
    #[case(b"818181911112111", 92)]
    fn sample_single(#[case] input: &[u8], #[case] expected: u64) {
        assert_eq!(run(input).unwrap(), expected);
    }

    #[rstest]
    #[case(b"9782", 98)]
    #[case(b"9792", 99)]
    fn extra_single(#[case] input: &[u8], #[case] expected: u64) {
        assert_eq!(run(input).unwrap(), expected);
    }
}
