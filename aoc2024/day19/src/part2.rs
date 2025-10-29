use miette::Result;
use rustc_hash::FxHashMap;

use crate::parse::parse;

pub fn run(content: &[u8]) -> Result<u64> {
    let data = parse(content)?;

    let mut cache = FxHashMap::default();

    let result: usize = data
        .designs
        .iter()
        .map(|design| count_valid_designs(design, &data.patterns, &mut cache))
        .sum();

    Ok(result as u64)
}

fn count_valid_designs<'a>(
    design: &'a [u8],
    patterns: &[&'a [u8]],
    cache: &mut FxHashMap<&'a [u8], usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(count) = cache.get(design) {
        return *count;
    }

    let count = patterns
        .iter()
        .map(|pattern| {
            if !design.starts_with(pattern) {
                return 0;
            }
            let sub_design = &design[pattern.len()..];
            count_valid_designs(sub_design, patterns, cache)
        })
        .sum();

    cache.insert(design, count);
    count
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
