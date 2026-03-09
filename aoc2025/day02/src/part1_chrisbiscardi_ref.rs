// cspell:words hundos
use common::error::AdventError;
use miette::Result;

use crate::parse::parse;

fn process_part((start, end): (&str, &str)) -> Result<u64, AdventError> {
    let start = start.parse::<u64>()?;
    let end = end.parse::<u64>()?;
    let range = (start..=end).clone();
    let mut total = 0;
    for id in range {
        // a number from 0-5, which is half of the
        // number of digits in the number
        let places = id.ilog10().div_ceil(2);
        // 10^n, which is 10, 100, 1000, etc
        let hundos = 10u64.pow(places);
        // 204204 == 204 === 204
        if id / hundos == id % hundos {
            total += id;
        }
    }

    Ok(total)
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
