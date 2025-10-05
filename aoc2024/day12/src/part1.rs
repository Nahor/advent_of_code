use common::position::{Direction, Position};
use miette::Result;

use crate::parse::parse;

pub fn run(content: &[u8]) -> Result<u64> {
    let mut grid = parse(content)?;

    let mut to_check = Vec::with_capacity(grid.width() * grid.height());
    let mut id = 0;
    let result: u64 = (0..grid.size())
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
            // Since index is scanning right then down, up and left have already
            // been processed and are not part of the region
            let mut perimeter = 2;
            let mut area = 1;
            to_check.push((pos, Direction::Right));
            to_check.push((pos, Direction::Down));

            while let Some((pos, dir)) = to_check.pop() {
                let next_pos = pos + dir;
                if let Some(new_cell) = grid.get_mut(next_pos) {
                    if new_cell.id == id {
                        // Already visited cell => ignore
                    } else if new_cell.region != region {
                        // Boundary with another region
                        perimeter += 1;
                    } else {
                        area += 1;
                        new_cell.id = id;
                        Direction::list().iter().for_each(|&new_dir| {
                            if new_dir != dir.reverse() {
                                to_check.push((next_pos, new_dir));
                            }
                        });
                    }
                } else {
                    // grid boundary
                    perimeter += 1;
                }
            }
            // println!("Area: {}: a={area}, p={perimeter}", (region + b'A') as char);
            perimeter * area
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
