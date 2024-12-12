/// Same as `part2_log` but using a vector instead of a `BTreeMap`
use std::collections::HashMap;

use miette::Result;

use crate::parse::parse;

pub fn run(content: &[u8], blinks: usize) -> Result<u64> {
    let stones = parse(content)?;

    // Key: stone number,
    // Value: map of <blink count-1, number of stones>
    let mut cache = HashMap::<u64, Vec<Option<usize>>>::new();

    let result: usize = stones
        .iter()
        .map(|stone| expand(*stone, blinks, &mut cache))
        .sum();

    // println!("{cache:?}");

    Ok(result as u64)
}

fn expand(stone: u64, blink_left: usize, cache: &mut HashMap<u64, Vec<Option<usize>>>) -> usize {
    let blink_left = blink_left - 1;

    if let Some(count) = cache
        .get(&stone)
        .and_then(|list| list.get(blink_left).cloned())
        .flatten()
    {
        return count;
    }

    let new_stones = if stone == 0 {
        vec![1]
    } else {
        let digits = stone.ilog10() + 1;
        if digits % 2 == 0 {
            let pow = 10_u64.pow(digits / 2);
            let left = stone / pow;
            let right = stone - (left * pow);
            vec![left, right]
        } else {
            vec![stone * 2024]
        }
    };
    let count = if blink_left == 0 {
        new_stones.len()
    } else {
        new_stones
            .into_iter()
            .map(|new_stone| expand(new_stone, blink_left, cache))
            .sum()
    };
    let entry = cache.entry(stone).or_default();
    entry.resize(entry.len().max(blink_left + 1), None);
    entry[blink_left] = Some(count);

    count
}

#[cfg(test)]
mod test {
    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"125 17"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE;

        assert_eq!(run(input, 75).unwrap(), 65601038650482);
    }

    #[test]
    fn compare_base() {
        assert_eq!(
            run(&INPUT_SAMPLE[1..], 75).unwrap(),
            crate::part2::run(&INPUT_SAMPLE[1..], 75).unwrap()
        );

        let input = common::read_input_u8!(None).unwrap();
        assert_eq!(
            run(&input, 75).unwrap(),
            crate::part2::run(&input, 75).unwrap()
        );
    }
}
