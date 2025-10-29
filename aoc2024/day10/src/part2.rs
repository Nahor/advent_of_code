use std::collections::HashMap;

use common::position::{Direction, Position};
use miette::Result;

use crate::parse::parse;

pub fn run(content: &[u8]) -> Result<u64> {
    let grid = parse(content)?;

    let mut list = HashMap::new();
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let position = Position::new(x, y);
            if matches!(grid.get(position), Some(0)) {
                check_trailhead(&grid, position, 0, &mut list);
            }
        }
    }
    let result: u64 = list.values().sum();

    Ok(result)
}

fn check_trailhead(
    grid: &crate::parse::Grid,
    position: Position,
    val: u8,
    list: &mut HashMap<Position, u64>,
) {
    let expected_val = val + 1;
    Direction::list().iter().for_each(|dir| {
        let new_position = position + *dir;
        let Some(new_val) = grid.get(new_position) else {
            return;
        };
        if new_val == expected_val {
            if new_val == 9 {
                *list.entry(new_position).or_default() += 1;
            } else {
                check_trailhead(grid, new_position, new_val, list);
            }
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 81);
    }

    #[test]
    fn example1() {
        let input = &br#"
.....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....
"#[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 3);
    }

    #[test]
    fn example2() {
        let input = &br#"
..90..9
...1.98
...2..7
6543456
765.987
876....
987....
"#[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 13);
    }

    #[test]
    fn example3() {
        let input = &br#"
012345
123456
234567
345678
4.6789
56789.
"#[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 227);
    }

    // #[test]
    // fn sample_sorted() {
    //     assert_eq!(
    //         run_sorted(&INPUT_SAMPLE[1..]).unwrap(),
    //         run(&INPUT_SAMPLE[1..]).unwrap()
    //     );

    //     let input = read_input_u8!(None).unwrap();
    //     assert_eq!(run_sorted(&input).unwrap(), run(&input).unwrap());
    // }
}
