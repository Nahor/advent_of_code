use std::{collections::VecDeque, ops::Range};

use common::position::{Direction, Position};
use miette::Result;
use rustc_hash::FxHashMap;

use crate::parse::parse;

pub fn run(content: &[u8]) -> Result<u64> {
    let mut grid = parse(content)?;

    let mut to_check =
        VecDeque::<(Position, Direction)>::with_capacity(grid.width() * grid.height());
    let mut id = 0;
    let result: usize = (0..grid.size())
        .map(|idx| {
            id += 1;
            let region = {
                let cell = grid.get_idx_mut(idx).unwrap();
                if cell.id != 0 {
                    // Already processed
                    return 0;
                }
                let region = cell.region;
                cell.id = id;
                region
            };

            let pos = Position::from_index(idx, grid.width());

            to_check.clear();

            // Key: column (resp. line) for vertical (resp. horizontal) fences
            // Value: range of lines (resp. columns) for each side
            // Because of region that meets are corners, we need to distinguish
            // between left vs right and up vs down
            let mut sides_up = FxHashMap::<usize, Vec<Range<usize>>>::default();
            let mut sides_down = FxHashMap::<usize, Vec<Range<usize>>>::default();
            let mut sides_left = FxHashMap::<usize, Vec<Range<usize>>>::default();
            let mut sides_right = FxHashMap::<usize, Vec<Range<usize>>>::default();

            // Since index is scanning right then down, up and left have already
            // been processed and are not part of the region
            sides_left.insert(pos.x, vec![{ pos.y..pos.y + 1 }]);
            sides_up.insert(pos.y, vec![{ pos.x..pos.x + 1 }]);

            let mut area = 1_usize;

            to_check.push_back((pos, Direction::Right));
            to_check.push_back((pos, Direction::Down));

            // We must `pop_front` to do a "breadth-first" algorithm, which
            // ensures that we'll never have two partial views of the same side
            // (i.e. for a [0..5] side, we'll never have [0..2]+[3..5] in the
            // hashmap.
            while let Some((pos, dir)) = to_check.pop_front() {
                let next_pos = pos + dir;
                if let Some(new_cell) = grid.get_mut(next_pos) {
                    if new_cell.id == id {
                        // Already visited cell => ignore
                        continue;
                    } else if new_cell.region == region {
                        area += 1;
                        new_cell.id = id;
                        Direction::list().iter().for_each(|&new_dir| {
                            if new_dir != dir.reverse() {
                                to_check.push_back((next_pos, new_dir));
                            }
                        });
                        continue;
                    }
                }

                // Boundary with another region or the outside
                let update_fn = |sides_map: &mut FxHashMap<usize, Vec<Range<usize>>>,
                                 major: usize,
                                 minor: usize| {
                    let side_list = sides_map.entry(major).or_default();
                    if let Some(range) = side_list.iter_mut().find(|side_range| {
                        side_range.end == minor || side_range.start == (minor + 1)
                    }) {
                        *range = range.start.min(minor)..range.end.max(minor + 1);
                    } else {
                        side_list.push(minor..(minor + 1));
                    }
                };
                match dir {
                    Direction::Up => update_fn(&mut sides_up, pos.y, pos.x),
                    Direction::Right => update_fn(&mut sides_right, pos.x, pos.y),
                    Direction::Down => update_fn(&mut sides_down, pos.y, pos.x),
                    Direction::Left => update_fn(&mut sides_left, pos.x, pos.y),
                };
            }
            let side_count = sides_left
                .values()
                .map(|side_list| side_list.len())
                .sum::<usize>()
                + sides_right
                    .values()
                    .map(|side_list| side_list.len())
                    .sum::<usize>()
                + sides_up
                    .values()
                    .map(|side_list| side_list.len())
                    .sum::<usize>()
                + sides_down
                    .values()
                    .map(|side_list| side_list.len())
                    .sum::<usize>();
            // println!(
            //     "Area: {}: area={area}, sides={side_count}",
            //     (region + b'A') as char
            // );
            side_count * area
        })
        .sum();

    Ok(result as u64)
}

#[cfg(test)]
mod test {
    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"#;

    const INPUT_EXAMPLE_1: &[u8] = br#"
AAAA
BBCD
BBCC
EEEC
"#;

    const INPUT_EXAMPLE_2: &[u8] = br#"
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
"#;

    const INPUT_EXAMPLE_3: &[u8] = br#"
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
"#;

    const INPUT_EXAMPLE_4: &[u8] = br#"
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
"#;

    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 1206);
    }

    #[test]
    fn example1() {
        let input = &INPUT_EXAMPLE_1[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 80);
    }

    #[test]
    fn example2() {
        let input = &INPUT_EXAMPLE_2[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 436);
    }

    #[test]
    fn example3() {
        let input = &INPUT_EXAMPLE_3[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 236);
    }

    #[test]
    fn example4() {
        let input = &INPUT_EXAMPLE_4[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 368);
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
