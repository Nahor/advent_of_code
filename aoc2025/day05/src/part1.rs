use std::ops::RangeInclusive;

use miette::Result;

use crate::parse::parse;

pub fn process(content: (Vec<RangeInclusive<u64>>, Vec<u64>)) -> Result<usize> {
    let (ranges, ids) = content;
    let result = ids
        .into_iter()
        .filter(|id| ranges.iter().any(|range| range.contains(id)))
        .count();

    Ok(result)
}

pub fn run(content: &[u8]) -> Result<usize> {
    process(parse(content)?)
}

#[cfg(test)]
mod test {
    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"
3-5
10-14
16-20
12-18

1
5
8
11
17
32
"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 3);
    }
}
