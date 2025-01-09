use miette::Result;

use crate::parse::parse;
use crate::part1::*;

pub fn run(content: &[u8]) -> Result<u64> {
    let codes = parse(content)?;

    let mut depth_cache = init_dir_cost_cache();

    let result: usize = codes
        .iter()
        .map(|code| get_code_length(code, 24, &mut depth_cache))
        .sum();

    Ok(result as u64)
}

#[cfg(test)]
mod test {
    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"
029A
980A
179A
456A
379A
"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        // Not an official value, but based on the assumption the algorithm is
        // valid since we passed the puzzle
        assert_eq!(run(input).unwrap(), 154115708116294);
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
