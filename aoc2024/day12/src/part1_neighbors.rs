use common::position::{Direction, Position};
use miette::Result;
use rustc_hash::FxHashSet;

use crate::parse::parse_u8;

pub fn run(content: &[u8]) -> Result<u64> {
    let grid = parse_u8(content)?;

    // Node will already visited or are already scheduled for visitation
    let mut visited = FxHashSet::default();
    visited.reserve(grid.width() * grid.height());

    // List of cell to check, starting with the current one
    let mut to_check = Vec::with_capacity(grid.width() * grid.height());

    let result: u64 = (0..grid.size())
        .map(|idx| {
            let pos = Position::from_index(idx, grid.width());
            if visited.contains(&pos) {
                return 0;
            }

            let region = grid.get_idx(idx).unwrap();

            // List of cell to check, starting with the current one
            to_check.push(pos);
            visited.insert(pos);

            let (area, perimeter) = (0..)
                .scan(&mut to_check, |to_check, _| {
                    let pos = to_check.pop()?;

                    // Find the neighbors and them to the list to check if we
                    // haven't seen them yet
                    let neighbors = Direction::list()
                        .iter()
                        .map(|dir| pos + *dir)
                        .filter(|new_pos| grid.get(*new_pos).is_some_and(|cell| cell == region))
                        .inspect(|new_pos| {
                            if !visited.contains(new_pos) {
                                to_check.push(*new_pos);
                                visited.insert(*new_pos);
                            }
                        })
                        .count() as u64;

                    Some(4 - neighbors)
                })
                .fold((0, 0), |(total_area, total_perimeter), cell_perimeter| {
                    (total_area + 1, total_perimeter + cell_perimeter)
                });

            // println!("Area: {}: a={area}, p={perimeter}", (region + b'A') as char);
            area * perimeter
        })
        .sum();

    Ok(result)
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
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 1930);
    }

    #[test]
    fn example1() {
        let input = &INPUT_EXAMPLE_1[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 140);
    }

    #[test]
    fn example2() {
        let input = &INPUT_EXAMPLE_2[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 772);
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
