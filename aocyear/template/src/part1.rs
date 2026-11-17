use anyhow::Result;

use crate::parse::parse;

pub fn run(filename: Option<String>, content: &[u8]) -> Result<u64> {
    let lines = parse(filename, content)?;
    let result = lines
        .into_iter()
        .map(|(t1, t2, s, _)| if (t1 + t2) == s { 1 } else { 0 })
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

        assert_eq!(run(None, input).unwrap(), 2);
    }

    // #[test]
    // fn compare_base() {
    //     assert_eq!(
    //         run(&INPUT_SAMPLE[1..]).unwrap(),
    //         crate::part1::run(&INPUT_SAMPLE[1..]).unwrap()
    //     );

    //     let input = common::read_input_u8!(None).unwrap();
    //     assert_eq!(run(&input).unwrap(), crate::part1::run(&input).unwrap());
    // }
}
