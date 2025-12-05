use miette::Result;
use rangemap::RangeInclusiveSet;

use crate::parse::custom::parse;

pub fn run(content: &[u8]) -> Result<usize> {
    let ranges = parse(content)?;
    let rangemap = RangeInclusiveSet::from_iter(ranges);
    let result = rangemap.iter().map(|range| range.clone().count()).sum();
    Ok(result)
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

        assert_eq!(run(input).unwrap(), 14);
    }
}
