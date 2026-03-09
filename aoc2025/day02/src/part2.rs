use std::collections::HashSet;

use common::error::AdventError;
use miette::Result;

use crate::parse::parse;

fn process_start(str: &str, repeat: usize) -> Result<u64, AdventError> {
    let len = str.len();
    if !len.is_multiple_of(repeat) {
        // Returns "10..." such as "10..."*`repeat` is longer than `str` and is
        // the smallest number bigger than `str` with `repeat` repetition
        // E.g.: str = "1234123", repeat = 2, "999'999" is too small to be in
        // the range, and "1000'1000" is the next number with a repeat of 2
        // greater than "1234123".
        // Or with repeat 3, "99'99'99" is too small and the next is "100'100'100"
        // Or with repeat 4, "9'9'9'9" is too small and the next is "10'10'10'10"
        let digits = len.div_ceil(repeat);
        let val = 10_u64.pow(digits as u32 - 1);
        // println!("start({str} * {repeat}): {val}");
        Ok(val)
    } else {
        let digits = len / repeat;
        let first = str[0..digits].parse::<u64>()?;
        // Check the ordering of the following chunks.
        // `first` is the smallest number is the following chunks are all equal
        // to `first` or if the first non-equal chunk is smaller
        // E.g. if `str` is "123123124xxx" and `digits` is 3, then "123123123xxx" is
        // too small to be in range, but "124124124xxx" is not. Inversely if `str` is
        // "123123122xxx", then "123123123xxx" is valid number to check.
        let ordering = (1..repeat)
            .map(|ith| {
                let start = ith * digits;
                let end = start + digits;
                // println!("start({str} * {repeat}) @ {ith}: {}", &str[start..end]);
                let val = str[start..end].parse::<u64>()?;
                Ok::<_, AdventError>(val.cmp(&first))
            })
            .find(|val| !val.as_ref().is_ok_and(|val| val.is_eq()))
            .transpose()?;

        if ordering.is_some_and(|ord| ord.is_gt()) {
            Ok(first + 1)
        } else {
            Ok(first)
        }
    }
}

fn process_end(str: &str, repeat: usize) -> Result<u64, AdventError> {
    let len = str.len();
    if !len.is_multiple_of(repeat) {
        // See similar comment in `process_start` except we want the biggest
        // smaller number i.e. "99..." instead of "100..."
        let digits = len.div_ceil(repeat);
        Ok(10_u64.pow(digits as u32 - 1) - 1)
    } else {
        let digits = len / repeat;
        let first = str[0..digits].parse::<u64>()?;
        // Check the ordering of the following chunks.
        // See similar comment in `process_start` except the first non-equal
        // chunk must be greater
        let ordering = (1..repeat)
            .map(|ith| {
                let start = ith * digits;
                let end = start + digits;
                // println!("start({str} * {repeat}) @ {ith}: {}", &str[start..end]);
                let val = str[start..end].parse::<u64>()?;
                Ok::<_, AdventError>(val.cmp(&first))
            })
            .find(|val| !val.as_ref().is_ok_and(|val| val.is_eq()))
            .transpose()?;
        // println!("end({str} * {repeat}): {first},{ordering:?}");
        if ordering.is_some_and(|ord| ord.is_lt()) {
            Ok(first - 1)
        } else {
            Ok(first)
        }
    }
}

fn process_part((start, end): (&str, &str)) -> Result<u64, AdventError> {
    let repeat_end = start.len().max(end.len()) + 1;
    let mut seen = HashSet::new();

    // println!("({start}-{end})");
    // println!("  repeat range: 2..{repeat_end}");

    (2..repeat_end)
        .map(|repeat| {
            let start_val = process_start(start, repeat)?;
            let end_val = process_end(end, repeat)?;
            // println!("({start}-{end}) * {repeat} = {start_val}..{end_val}");
            if start_val > end_val {
                Ok(0)
            } else {
                Ok((start_val..=end_val)
                    .map(|val| {
                        let digits = val.ilog10() + 1;
                        let val = (0_u32..(repeat as u32))
                            .map(|ith| val * 10_u64.pow(digits * ith))
                            .sum();
                        if seen.insert(val) {
                            // println!("found {val}");
                            val
                        } else {
                            // println!("again {val}");
                            0
                        }
                    })
                    .sum())
            }
        })
        .sum()
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

        assert_eq!(run(input).unwrap(), 4174379265);
    }

    #[rstest]
    #[case(("11","22"), 11 + 22)]
    #[case(("95","115"), 99 + 111)]
    #[case(("998","1012"), 999 + 1010)]
    #[case(("1188511880","1188511890"), 1188511885)]
    #[case(("222220","222224"), 222222)]
    #[case(("1698522","1698528"), 0)]
    #[case(("446443","446449"), 446446)]
    #[case(("38593856","38593862"), 38593859)]
    #[case(("565653","565659"), 565656)]
    #[case(("824824821","824824827"), 824824824)]
    #[case(("2121212118","2121212124"), 2121212121)]
    fn singles(#[case] input: (&str, &str), #[case] expected: u64) {
        assert_eq!(process_part(input).unwrap(), expected);
    }
}
