use common::error::AdventError;
use miette::Result;

use crate::parse::parse;

pub fn run(content: &[u8]) -> Result<u64, AdventError> {
    let lines = parse(content)?;
    let result = lines
        .into_iter()
        .map(|(t1, t2, _, m)| if (t1 * t2) == m { 1 } else { 0 })
        .sum();
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"
0 0 0 0
10 10 20 100
"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 2);
    }

    // #[test]
    // fn compare_base() {
    //     assert_eq!(
    //         run(&INPUT_SAMPLE[1..]).unwrap(),
    //         crate::part2::run(&INPUT_SAMPLE[1..]).unwrap()
    //     );

    //     let input = common::read_input_u8!(None).unwrap();
    //     assert_eq!(run(&input).unwrap(), crate::part2::run(&input).unwrap());
    // }
}
