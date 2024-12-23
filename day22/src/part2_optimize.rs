use std::iter;

use itertools::Itertools;
use miette::Result;
use rustc_hash::FxHashMap;

use crate::parse::parse;

pub fn run(content: &[u8]) -> Result<u64> {
    let secrets = parse(content)?;

    let result: u64 = merge_changes(&secrets);

    Ok(result)
}

fn merge_changes(list: &[u32]) -> u64 {
    // Combine the list of changes to see how many bananas we can buy for
    // each possible change
    let v = list.iter().map(|&secret| change_list::<2000>(secret)).fold(
        // theoretically, the key is a tuple with 4 value in the range -9..9
        // included, i.e. 19*19*19*19 possibilities = 130321.
        // In practice, for my input, it's ~40k, so round up to 50k.
        FxHashMap::<_, u64>::with_capacity_and_hasher(50000, Default::default()),
        |mut map, change_list| {
            change_list.into_iter().for_each(|(change, price)| {
                *map.entry(change).or_default() += price as u64;
            });
            map
        },
    );

    // Find which change has the most bananas
    let (_best_change, best_count) = v
        .iter()
        .max_by(|(_, count1), (_, count2)| count1.cmp(count2))
        .expect("should have one best (change,count)");

    //println!("Best: {best_count} for {best_change:?}");
    *best_count
}

// List of 4 changes and the matching price for a given secret/buyer
fn change_list<const N: usize>(secret: u32) -> FxHashMap<(i8, i8, i8, i8), i8> {
    iter::successors(Some(secret), |&secret| Some(next_secret(secret)))
        .map(|secret| (secret % 10) as i8)
        .tuple_windows()
        .map(|(p1, p2)| (p2 - p1, p2))
        .take(N) // We need N price changes, not N prices, so `take` needs to be here, not earlier
        .tuple_windows()
        .map(|((c1, _), (c2, _), (c3, _), (c4, p))| ((c1, c2, c3, c4), p))
        .fold(
            // Technically a bit less than N since we need a minimum of 5
            // entry to generate one output
            FxHashMap::with_capacity_and_hasher(N, Default::default()),
            |mut map, (change, value)| {
                // Can't use `collect()` because we want the price the first time a key
                // matches, while `collect()` will return the last match
                map.entry(change).or_insert(value);
                map
            },
        )
}

fn next_secret(mut secret: u32) -> u32 {
    secret ^= secret * 64;
    secret %= 16777216;
    secret ^= secret / 32;
    secret %= 16777216;
    secret ^= secret * 2048;
    secret %= 16777216;
    secret
}

#[cfg(test)]
mod test {
    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"
1
2
3
2024
"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 23);
    }

    #[test]
    fn simple_10() {
        let mut map = FxHashMap::default();
        map.extend([
            ((-3, 6, -1, -1), 4),
            ((6, -1, -1, 0), 4),
            ((-1, -1, 0, 2), 6),
            ((-1, 0, 2, -2), 4),
            ((0, 2, -2, 0), 4),
            ((2, -2, 0, -2), 2),
        ]);

        assert_eq!(change_list::<9>(123), map);
    }

    #[test]
    fn compare_base() {
        assert_eq!(
            run(&INPUT_SAMPLE[1..]).unwrap(),
            crate::part2::run(&INPUT_SAMPLE[1..]).unwrap()
        );

        let input = common::read_input_u8!(None).unwrap();
        assert_eq!(run(&input).unwrap(), crate::part2::run(&input).unwrap());
    }
}
