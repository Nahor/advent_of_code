use std::collections::HashSet;

use miette::Result;

use crate::{
    document::{Direction, Position},
    parse_fancy::parse,
};

pub fn run(content: &[u8]) -> Result<u64> {
    let (cols, lines, mut next) = parse(content)?;

    let mut direction = Direction::Up;
    let mut distinct_pos = HashSet::new();
    loop {
        let mut position = next;

        next = match direction {
            Direction::Up => {
                let blocked_list = &cols[position.x];
                let Err(idx) = blocked_list.binary_search(&position.y) else {
                    panic!("Our position is blocked");
                };

                Position {
                    x: position.x,
                    y: blocked_list.get(idx - 1).map_or(0, |y| y + 1),
                }
            }
            Direction::Right => {
                let blocked_list = &lines[position.y];
                let Err(idx) = blocked_list.binary_search(&position.x) else {
                    panic!("Our position is blocked at {:?}", position);
                };

                Position {
                    x: blocked_list.get(idx).map_or(cols.len() - 1, |x| x - 1),
                    y: position.y,
                }
            }
            Direction::Down => {
                let blocked_list = &cols[position.x];
                let Err(idx) = blocked_list.binary_search(&position.y) else {
                    panic!("Our position is blocked");
                };

                Position {
                    x: position.x,
                    y: blocked_list.get(idx).map_or(lines.len() - 1, |y| y - 1),
                }
            }
            Direction::Left => {
                let blocked_list = &lines[position.y];
                let Err(idx) = blocked_list.binary_search(&position.x) else {
                    panic!("Our position is blocked");
                };

                Position {
                    x: blocked_list.get(idx - 1).map_or(0, |x| x + 1),
                    y: position.y,
                }
            }
        };

        // Add the intermediate positions
        while position != next {
            distinct_pos.insert(position);
            position += direction;
        }

        if (next.x == 0)
            || (next.y == 0)
            || (next.x >= (cols.len() - 1))
            || (next.y >= (lines.len() - 1))
        {
            distinct_pos.insert(next);
            break;
        }
        direction = match direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }

    Ok(distinct_pos.len() as u64)
}

#[cfg(test)]
mod test {
    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 41);
    }

    // #[test]
    // fn sample_sorted() {
    //     assert_eq!(
    //         run_sorted(&INPUT_SAMPLE[1..]).unwrap(),
    //         run(&INPUT_SAMPLE[1..]).unwrap()
    //     );

    //     let input = read_input_u8(None).unwrap();
    //     assert_eq!(run_sorted(&input).unwrap(), run(&input).unwrap());
    // }
}
