// Same as `part2_straighten` but with rayon
use miette::Result;
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::parse::parse_ordered_vec;

// Same as part 1 but generalized: the cheat moves forms a diamond with a small
// hole inside, instead of a fixed 5x5 one, and the cost of the cheat depends on
// the number of moves instead of being 2
pub fn run(content: &[u8], min_save: usize) -> Result<u64> {
    let path = parse_ordered_vec(content)?;

    const NUM_MOVES: usize = 20;

    // let s = Instant::now();
    // To save min_save moves, with a min cost of a cheat being
    // 2, the exit must be at least min_save+2 away
    let min_skip = min_save + 2;
    let result: usize = path[0..(path.len() - min_skip)]
        .par_iter()
        .enumerate()
        .map(|(start_i, start_pos)| {
            path.iter()
                .enumerate()
                .skip(start_i + min_skip)
                .filter_map(|(end_i, end_pos)| {
                    let cheat_cost =
                        start_pos.0.abs_diff(end_pos.0) + start_pos.1.abs_diff(end_pos.1);
                    (2..=NUM_MOVES)
                        .contains(&cheat_cost)
                        .then_some((end_i, cheat_cost))
                })
                .filter(|&(end_i, cheat_cost)| {
                    let path_cost = end_i - start_i;
                    (cheat_cost < path_cost) && ((path_cost - cheat_cost) >= min_save)
                })
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
