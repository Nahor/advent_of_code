use std::collections::HashSet;

use common::position::{Direction, Position};
use miette::Result;

use crate::parse::parse;

pub fn run(content: &[u8]) -> Result<u64> {
    let grid = parse(content)?;

    let mut result: usize = 0;
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let position = Position::new(x, y);
            if matches!(grid.get(position), Some(0)) {
                let list = check_trailhead(&grid, position, 0);
                result += list.len();
            }
        }
    }

    Ok(result as u64)
}

fn check_trailhead(grid: &crate::parse::Grid, position: Position, val: u8) -> HashSet<Position> {
    Direction::list()
        .iter()
        .fold(HashSet::new(), |mut list, dir| {
            let new_position = position + *dir;
            let Some(new_val) = grid.get(new_position) else {
                return list;
            };
            if new_val == (val + 1) {
                if new_val == 9 {
                    list.insert(new_position);
                } else {
                    list.extend(check_trailhead(grid, new_position, new_val));
                }
            }
            list
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

        assert_eq!(run(input).unwrap(), 36);
    }

    #[test]
    fn example1() {
        let input = &br#"
0123
1234
8765
9876
"#[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 1);
    }

    #[test]
    fn example2() {
        let input = &br#"
...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9
"#[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 2);
    }

    #[test]
    fn example3() {
        let input = &br#"
..90..9
...1.98
...2..7
6543456
765.987
876....
987....
"#[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 4);
    }

    #[test]
    fn example4() {
        let input = &br#"
10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01
"#[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 3);
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
