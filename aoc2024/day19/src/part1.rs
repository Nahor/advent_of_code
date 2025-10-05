use miette::Result;
use rustc_hash::FxHashMap;

use crate::parse::parse;

pub fn run(content: &[u8]) -> Result<u64> {
    let data = parse(content)?;

    let mut cache = FxHashMap::default();

    let result: usize = data
        .designs
        .iter()
        .filter(|design| is_design_valid(design, 0, &data.patterns, &mut cache))
        .count();

    Ok(result as u64)
}

fn is_design_valid<'a>(
    design: &'a [u8],
    design_idx: usize,
    patterns: &Vec<&'a [u8]>,
    cache: &mut FxHashMap<&'a [u8], bool>,
) -> bool {
    if design_idx == design.len() {
        return true;
    }

    let sub_design = &design[design_idx..design.len()];
    if let Some(valid) = cache.get(sub_design) {
        return *valid;
    }

    for pattern in patterns {
        if sub_design.starts_with(pattern) {
            let len = design_idx + pattern.len();
            cache.insert(&design[0..len], true);
            if is_design_valid(design, len, patterns, cache) {
                cache.insert(sub_design, true);
                return true;
            }
        }
    }

    // We couldn't find a valid set of patterns matching this sub_design
    cache.insert(sub_design, false);
    false
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

        assert_eq!(run(input).unwrap(), 6);
    }

    #[test]
    fn multiple() {
        // cspell:disable
        let input = &br#"
bw, ru, b, wr

bwru
bwrb
"#[1..]; // remove leading \n
         // cspell:enable

        assert_eq!(run(input).unwrap(), 2);
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
