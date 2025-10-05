use cached::{proc_macro::cached, Cached};
use miette::Result;

use crate::parse::parse;

pub fn run(content: &[u8]) -> Result<u64> {
    let data = parse(content)?;

    COUNT_VALID_DESIGNS.lock().unwrap().cache_reset();
    let result: usize = data
        .designs
        .iter()
        .map(|design| count_valid_designs(design, &data.patterns))
        .sum();

    // println!(
    //     "cache misses: {:?}",
    //     COUNT_VALID_DESIGNS.lock().unwrap().cache_misses()
    // );
    // println!(
    //     "cache hits: {:?}",
    //     COUNT_VALID_DESIGNS.lock().unwrap().cache_hits()
    // );

    Ok(result as u64)
}

#[cached(key = "Vec<u8>", convert = r#"{Vec::from({design})}"#)]
fn count_valid_designs(design: &[u8], patterns: &[&[u8]]) -> usize {
    if design.is_empty() {
        return 1;
    }
    patterns
        .iter()
        .map(|pattern| {
            if !design.starts_with(pattern) {
                return 0;
            }
            let sub_design = &design[pattern.len()..];

            count_valid_designs(sub_design, patterns)
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 16);
    }

    #[test]
    fn compare_base() {
        // assert_eq!(
        //     run(&INPUT_SAMPLE[1..]).unwrap(),
        //     crate::part2::run(&INPUT_SAMPLE[1..]).unwrap()
        // );

        let input = common::read_input_u8!(None).unwrap();
        assert_eq!(run(&input).unwrap(), crate::part2::run(&input).unwrap());
    }
}
