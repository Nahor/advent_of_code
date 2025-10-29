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
    let v = list.iter().map(|&secret| change_list::<2000>(secret)).fold(
        FxHashMap::<_, u64>::default(),
        |mut map, change_list| {
            change_list.into_iter().for_each(|(change, price)| {
                *map.entry(change).or_default() += price as u64;
            });
            map
        },
    );
    let (_best_change, best_count) = v
        .iter()
        .max_by(|(_, count1), (_, count2)| count1.cmp(count2))
        .expect("should have one best (change,count)");
    //println!("Best: {best_count} for {best_change:?}");
    *best_count
}

fn change_list<const N: usize>(secret: u32) -> FxHashMap<(i8, i8, i8, i8), i8> {
    // We need N "price changes", so we need N+1 prices (the first one does not
    // generate change)
    (0..(N + 1))
        .scan(secret, |secret, _| {
            let tmp = *secret;
            *secret = next_secret(*secret);
            Some(tmp)
        })
        .map(|secret| (secret % 10) as i8)
        .tuple_windows()
        .map(|(p1, p2)| (p2 - p1, p2))
        .tuple_windows::<(_, _, _, _)>()
        .map(|((c1, _), (c2, _), (c3, _), (c4, p))| ((c1, c2, c3, c4), p))
        .fold(FxHashMap::default(), |mut map, (change, value)| {
            // Can't use `collect()` because we want the price the first time a key
            // matches, while `collect()` will return the last match
            map.entry(change).or_insert(value);
            map
        })
}

fn next_secret(mut secret: u32) -> u32 {
    secret ^= secret.wrapping_mul(64);
    secret %= 16777216;
    secret ^= secret.wrapping_div(32);
    secret %= 16777216;
    secret ^= secret.wrapping_mul(2048);
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
