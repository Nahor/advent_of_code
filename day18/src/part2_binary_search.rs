// Same as `part2_brute_force` but doing a binary search to find the place
// where the path becomes blocked
use common::position::Direction;
use miette::Result;
use pathfinding::prelude::astar;

use crate::parse::{parse, Coord};

pub fn run(content: &[u8], max_coord: isize, min_amount: usize) -> Result<String> {
    let bytes = parse(content)?;

    // From part1, we already know `min_amount` can reach the end.
    //
    // Ideally, we would use Rust's `partition_point` or `binary_search_by`,
    // but those are only available with slices, not ranges. And we can't use
    // `bytes` as a slice because we need the index/amount to build the path.
    let mut range = min_amount..bytes.len();
    while range.len() > 1 {
        let mid = (range.end + range.start) / 2;
        // println!("range: {range:?} - mid {mid}");
        if astar(
            &Coord { x: 0, y: 0 },
            |&pos| successors(pos, &bytes, max_coord, mid),
            |&pos| max_coord.abs_diff(pos.x) + max_coord.abs_diff(pos.y),
            |&pos| {
                pos == Coord {
                    x: max_coord,
                    y: max_coord,
                }
            },
        )
        .is_some()
        {
            range = mid..range.end;
        } else {
            range = range.start..mid;
        }
    }

    let b = bytes[range.start];
    Ok(format!("{},{}", b.x, b.y))
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

        assert_eq!(run(input, 6, 12).unwrap(), "6,1");
    }

    #[test]
    fn compare_base() {
        assert_eq!(
            run(&INPUT_SAMPLE[1..], 6, 12).unwrap(),
            crate::part2::run(&INPUT_SAMPLE[1..], 6, 12).unwrap()
        );

        let input = common::read_input_u8!(None).unwrap();
        assert_eq!(
            run(&input, 70, 1024).unwrap(),
            crate::part2::run(&input, 70, 1024).unwrap()
        );
    }
}
