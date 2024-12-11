use miette::Result;
use rustc_hash::FxHashMap;

use crate::parse::parse;

pub fn run(content: &[u8], blinks: usize) -> Result<u64> {
    let stones = parse(content)?;

    // Key: stone number,
    // Value: map of <blink count-1, number of stones>
    let mut cache = FxHashMap::<(u64, usize), usize>::default();

    let result: usize = stones
        .iter()
        .map(|stone| expand(*stone, blinks, &mut cache))
        .sum();

    Ok(result as u64)
}

fn expand(stone: u64, blink_left: usize, cache: &mut FxHashMap<(u64, usize), usize>) -> usize {
    let blink_left = blink_left - 1;

    if let Some(count) = cache.get(&(stone, blink_left)) {
        // println!("= Expanding {stone}@{blink_left}: {count}");
        return *count;
    }

    // println!("> Expanding {stone}@{blink_left}");
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
    cache.insert((stone, blink_left), count);

    // println!("< Expanding {stone}@{blink_left}: {count}");
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

        // result not from website, but after successfully passing part2
        assert_eq!(run(input, 75).unwrap(), 65601038650482);
    }

    #[test]
    fn compare_base() {
        assert_eq!(
            run(&INPUT_SAMPLE[1..], 75).unwrap(),
            crate::part2::run(&INPUT_SAMPLE[1..], 75).unwrap()
        );

        let input = common::input::read_input_u8(None).unwrap();
        assert_eq!(
            run(&input, 75).unwrap(),
            crate::part2::run(&input, 75).unwrap()
        );
    }
}
