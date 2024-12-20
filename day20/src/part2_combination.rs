// Same as part2_cost_first, but use itertools's tuple_combinations to
// compute the start and end points of the jump
use itertools::Itertools;
use miette::Result;

use crate::parse::parse_ordered_vec;

// Same as part 1 but generalized: the cheat moves forms a diamond with a small
// hole inside, instead of a fixed 5x5 one, and the cost of the cheat depends on
// the number of moves instead of being 2
pub fn run(content: &[u8], min_save: usize) -> Result<u64> {
    let path = parse_ordered_vec(content)?;

    const NUM_MOVES: usize = 20;

    let result: usize = path
        .iter()
        .enumerate()
        .tuple_combinations()
        .filter(|((start_i, start_pos), (end_i, end_pos))| {
            let cheat_cost = start_pos.0.abs_diff(end_pos.0) + start_pos.1.abs_diff(end_pos.1);
            if cheat_cost > NUM_MOVES {
                return false;
            }
            let normal_cost = end_i - start_i;
            (normal_cost - cheat_cost) >= min_save
        })
        .count();

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
