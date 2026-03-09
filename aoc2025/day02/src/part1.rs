use common::error::AdventError;
use miette::Result;

use crate::parse::parse;

fn process_start(str: &str) -> Result<u64, AdventError> {
    let len = str.len();
    if len % 2 == 1 {
        let p = len / 2;
        Ok(10_u64.pow(p as u32))
    } else {
        let first = str[0..len / 2].parse::<u64>()?;
        let second = str[len / 2..len].parse::<u64>()?;
        if first < second {
            Ok(first + 1)
        } else {
            Ok(first)
        }
    }
}

fn process_end(str: &str) -> Result<u64, AdventError> {
    let len = str.len();
    if len % 2 == 1 {
        let p = len / 2;
        Ok(10_u64.pow(p as u32) - 1)
    } else {
        let first = str[0..len / 2].parse::<u64>()?;
        let second = str[len / 2..len].parse::<u64>()?;
        if first > second {
            Ok(first - 1)
        } else {
            Ok(first)
        }
    }
}

fn process_part((start, end): (&str, &str)) -> Result<u64, AdventError> {
    let start_val = process_start(start)?;
    let end_val = process_end(end)?;
    if start_val > end_val {
        Ok(0)
    } else {
        Ok((start_val..=end_val)
            .map(|val| {
                let digits = val.ilog10() + 1;
                val * 10_u64.pow(digits) + val
            })
            .sum())
    }
}

pub fn run(content: &str) -> Result<u64, AdventError> {
    let lines = parse(content)?;

    lines.into_iter().map(process_part).sum()
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &str = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 1227775554);
    }

    #[rstest]
    #[case(("11","22"), 33)]
    #[case(("95","115"), 99)]
    #[case(("998","1012"), 1010)]
    #[case(("1188511880","1188511890"), 1188511885)]
    #[case(("222220","222224"), 222222)]
    #[case(("1698522","1698528"), 0)]
    #[case(("446443","446449"), 446446)]
    #[case(("38593856","38593862"), 38593859)]
    #[case(("565653","565659"), 0)]
    #[case(("824824821","824824827"), 0)]
    #[case(("2121212118","2121212124"), 0)]
    fn singles(#[case] input: (&str, &str), #[case] expected: u64) {
        assert_eq!(process_part(input).unwrap(), expected);
    }
}
