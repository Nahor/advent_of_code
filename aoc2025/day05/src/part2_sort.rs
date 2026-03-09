use std::ops::RangeInclusive;

use miette::Result;

use crate::parse::parse;

#[inline]
fn is_overlapping<T>(r1: &RangeInclusive<T>, r2: &RangeInclusive<T>) -> bool
where
    T: PartialOrd,
{
    (r1.start() <= r2.end()) && (r1.end() >= r2.start())
}

#[inline]
fn union<T>(r1: &RangeInclusive<T>, r2: &RangeInclusive<T>) -> RangeInclusive<T>
where
    T: Ord + Copy,
{
    (*r1.start()).min(*r2.start())..=(*r1.end()).max(*r2.end())
}

pub fn process(content: (Vec<RangeInclusive<u64>>, Vec<u64>)) -> Result<usize> {
    let (mut ranges, _) = content;

    ranges.sort_unstable_by_key(|r| *r.start());

    // check if the new range bridged the gap between already inserted ranges
    let mut idx = 0;
    while idx < (ranges.len() - 1) {
        let first = ranges[idx].clone();
        let second = ranges[idx + 1].clone();
        if is_overlapping(&first, &second) {
            ranges[idx] = union(&first, &second);
            ranges.remove(idx + 1);
        } else {
            idx += 1;
        }
    }

    let result = ranges.iter().map(|range| range.clone().count()).sum();
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

        assert_eq!(run(input).unwrap(), 14);
    }
}
