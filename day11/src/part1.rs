use std::collections::{BTreeMap, HashMap};

use miette::Result;

use crate::parse::parse;

pub fn run(content: &[u8]) -> Result<u64> {
    let stones = parse(content)?;

    // Key: stone number,
    // Value: map of <blink count-1, number of stones>
    let mut cache = HashMap::<u64, BTreeMap<usize, usize>>::new();

    let result: usize = stones
        .iter()
        .map(|stone| expand(*stone, 25, &mut cache))
        .sum();

    Ok(result as u64)
}

fn expand(
    stone: u64,
    blink_left: usize,
    cache: &mut HashMap<u64, BTreeMap<usize, usize>>,
) -> usize {
    let blink_left = blink_left - 1;

    if let Some(count) = cache.get(&stone).and_then(|list| list.get(&blink_left)) {
        return *count;
    }

    let new_stones = if stone == 0 {
        vec![1]
    } else {
        let str = format!("{}", stone).to_string();
        let half_len = str.len() / 2;
        if str.len() % 2 == 0 {
            vec![
                str[0..half_len].parse().unwrap(),
                str[half_len..].parse().unwrap(),
            ]
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
    entry.insert(blink_left, count);

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

        assert_eq!(run(input).unwrap(), 55312);
    }

    // #[test]
    // fn sample_sorted() {
    //     assert_eq!(
    //         run_sorted(&INPUT_SAMPLE[1..]).unwrap(),
    //         run(&INPUT_SAMPLE[1..]).unwrap()
    //     );

    //     let input = read_input_u8(None).unwrap();
    //     assert_eq!(run_sorted(&input).unwrap(), run(&input).unwrap());
    // }
}
