use itertools::{Either, Itertools};
use miette::Result;

use crate::parse::{parse, Schematic};

pub fn run(content: &[u8]) -> Result<u64> {
    let schematics = parse(content)?;

    let (locks, keys): (Vec<_>, Vec<_>) =
        schematics
            .into_iter()
            .partition_map(|schematic| match schematic {
                Schematic::Lock(lock) => Either::Left(lock),
                Schematic::Key(key) => Either::Right(key),
            });

    let result: usize = locks
        .iter()
        .cartesian_product(keys)
        .filter(|(lock, key)| {
            // Note that the keys include their last line in the count
            // unlike the online example, so "6" is the number to match
            (0..5).fold(true, |acc, i| acc & ((lock[i] + key[i]) <= 6))
        })
        .count();

    Ok(result as u64)
}

#[cfg(test)]
mod test {
    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 3);
    }

    #[test]
    fn compare_base() {
        assert_eq!(
            run(&INPUT_SAMPLE[1..]).unwrap(),
            crate::part1::run(&INPUT_SAMPLE[1..]).unwrap()
        );

        let input = common::read_input_u8!(None).unwrap();
        assert_eq!(run(&input).unwrap(), crate::part1::run(&input).unwrap());
    }
}
