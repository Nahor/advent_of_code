use common::position::Direction;
use miette::{miette, Result};
use pathfinding::prelude::astar;

use crate::parse::{parse, Coord};

pub fn run(content: &[u8], max_coord: isize, amount: usize) -> Result<u64> {
    let bytes = parse(content)?;

    let Some((path, _)) = astar(
        &Coord { x: 0, y: 0 },
        |&pos| successors(pos, &bytes, max_coord, amount),
        |&pos| max_coord.abs_diff(pos.x) + max_coord.abs_diff(pos.y),
        |&pos| {
            pos == Coord {
                x: max_coord,
                y: max_coord,
            }
        },
    ) else {
        return Err(miette!("No solution"));
    };

    Ok((path.len() - 1) as u64)
}

fn successors(pos: Coord, bytes: &[Coord], max_coord: isize, amount: usize) -> Vec<(Coord, usize)> {
    let coord_range = 0..=max_coord;

    Direction::list()
        .iter()
        .map(|&dir| pos + dir)
        .filter(|pos| coord_range.contains(&pos.x) && coord_range.contains(&pos.y))
        .filter(|pos| bytes[0..amount].iter().all(|byte| byte != pos))
        .map(|pos| (pos, 1))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input, 6, 12).unwrap(), 22);
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
