use miette::Result;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;

use crate::parse::parse;

// Same as part 1 but generalized: the cheat moves forms a diamond with a small
// hole inside, instead of a fixed 5x5 one, and the cost of the cheat depends on
// the number of moves instead of being 2
pub fn run(content: &[u8], min_save: usize) -> Result<u64> {
    let path = parse(content)?;

    // Build the list of possible destination of the cheat around the starting
    // one. At most, we can move 20 times, and to cross a wall, we need at least
    // 2 moves.
    const NUM_MOVES: isize = 20;
    let dir_list = (-NUM_MOVES..=NUM_MOVES)
        .flat_map(move |y| (-NUM_MOVES..=NUM_MOVES).map(move |x| (x, y)))
        .filter(|(x, y)| {
            let dist = x.abs() + y.abs();
            (2..=NUM_MOVES).contains(&dist)
        })
        .collect::<Vec<(isize, isize)>>();

    let result: usize = path
        .clone()
        .into_par_iter()
        .map(|(pos, start_cheat)| {
            dir_list
                .iter()
                .map(|dir| {
                    let end = (pos.0 + dir.0, pos.1 + dir.1);
                    let cost = (dir.0.abs() + dir.1.abs()) as usize;
                    (end, cost)
                })
                .filter_map(|(pos, cost)| path.get(&pos).map(|end_idx| (end_idx, cost)))
                .filter_map(|(end_cheat, cost)| {
                    if *end_cheat > start_cheat {
                        let saving = end_cheat.abs_diff(start_cheat) - cost;
                        Some(saving)
                    } else {
                        None
                    }
                })
                .filter(|save| *save >= min_save)
                .count()
        })
        .sum();

    Ok(result as u64)
}

#[cfg(test)]
mod test {
    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        let mut count = 3;
        assert_eq!(run(input, 76).unwrap(), count);
        count += 4;
        assert_eq!(run(input, 74).unwrap(), count);
        count += 22;
        assert_eq!(run(input, 72).unwrap(), count);
        count += 12;
        assert_eq!(run(input, 70).unwrap(), count);
        count += 14;
        assert_eq!(run(input, 68).unwrap(), count);
        count += 12;
        assert_eq!(run(input, 66).unwrap(), count);
        count += 19;
        assert_eq!(run(input, 64).unwrap(), count);
        count += 20;
        assert_eq!(run(input, 62).unwrap(), count);
        count += 23;
        assert_eq!(run(input, 60).unwrap(), count);
        count += 25;
        assert_eq!(run(input, 58).unwrap(), count);
        count += 39;
        assert_eq!(run(input, 56).unwrap(), count);
        count += 29;
        assert_eq!(run(input, 54).unwrap(), count);
        count += 31;
        assert_eq!(run(input, 52).unwrap(), count);
        count += 32;
        assert_eq!(run(input, 50).unwrap(), count);
    }

    #[test]
    fn compare_base() {
        let input = common::read_input_u8!(None).unwrap();
        assert_eq!(
            run(&input, 100).unwrap(),
            crate::part2::run(&input, 100).unwrap()
        );
    }
}
