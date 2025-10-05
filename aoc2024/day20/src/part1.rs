use miette::Result;

use crate::parse::parse;

#[rustfmt::skip]
const DIR: [(isize, isize);8] = [
             (0,-2),
        (-1,-1), (1, -1),
    (-2,0),          (2,0),
        (-1, 1), (1, 1),
             (0, 2)
];

pub fn run(content: &[u8], min_save: usize) -> Result<u64> {
    let path = parse(content)?;

    let result: usize = path
        .iter()
        .map(|(pos, start_cheat)| {
            DIR.iter()
                .map(|dir| (pos.0 + dir.0, pos.1 + dir.1))
                .filter_map(|pos| path.get(&pos))
                .filter_map(|end_cheat| {
                    if end_cheat > start_cheat {
                        Some(end_cheat.abs_diff(*start_cheat) - 2)
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

        assert_eq!(run(input, 64).unwrap(), 1);
        assert_eq!(run(input, 40).unwrap(), 1 + 1);
        assert_eq!(run(input, 38).unwrap(), 1 + 1 + 1);
        assert_eq!(run(input, 36).unwrap(), 1 + 1 + 1 + 1);
        assert_eq!(run(input, 20).unwrap(), 1 + 1 + 1 + 1 + 1);
        assert_eq!(run(input, 12).unwrap(), 1 + 1 + 1 + 1 + 1 + 3);
        assert_eq!(run(input, 10).unwrap(), 1 + 1 + 1 + 1 + 1 + 3 + 2);
        assert_eq!(run(input, 8).unwrap(), 1 + 1 + 1 + 1 + 1 + 3 + 2 + 4);
        assert_eq!(run(input, 6).unwrap(), 1 + 1 + 1 + 1 + 1 + 3 + 2 + 4 + 2);
        assert_eq!(
            run(input, 4).unwrap(),
            1 + 1 + 1 + 1 + 1 + 3 + 2 + 4 + 2 + 14
        );
        assert_eq!(
            run(input, 2).unwrap(),
            1 + 1 + 1 + 1 + 1 + 3 + 2 + 4 + 2 + 14 + 14
        );
    }

    // #[test]
    // fn compare_base() {
    //     assert_eq!(
    //         run(&INPUT_SAMPLE[1..]).unwrap(),
    //         crate::part1::run(&INPUT_SAMPLE[1..]).unwrap()
    //     );

    //     let input = common::read_input_u8!(None).unwrap();
    //     assert_eq!(run(&input).unwrap(), crate::part1::run(&input).unwrap());
    // }
}
