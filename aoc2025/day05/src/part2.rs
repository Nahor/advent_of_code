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

pub fn run(content: &[u8]) -> Result<usize> {
    let (ranges, _) = parse(content)?;

    let mut sorted_ranges: Vec<RangeInclusive<u64>> = vec![];
    ranges.iter().for_each(|range| {
        let result = sorted_ranges.binary_search_by_key(range.start(), |r| *r.start());
        let idx = match result {
            Ok(idx) => {
                sorted_ranges[idx] = union(&sorted_ranges[idx], range);
                idx
            }
            Err(idx) => {
                sorted_ranges.insert(idx, range.clone());
                if (idx > 0) && is_overlapping(&sorted_ranges[idx - 1], range) {
                    idx - 1
                } else {
                    idx
                }
            }
        };

        // check if the new range bridged the gap between already inserted ranges
        while idx < (sorted_ranges.len() - 1) {
            let first = sorted_ranges[idx].clone();
            let second = sorted_ranges[idx + 1].clone();
            if is_overlapping(&first, &second) {
                sorted_ranges[idx] = union(&first, &second);
                sorted_ranges.remove(idx + 1);
            } else {
                break;
            }
        }
    });

    let result = sorted_ranges
        .iter()
        .map(|range| range.clone().count())
        .sum();
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
