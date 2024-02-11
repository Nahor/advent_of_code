use day18::part1::*;
use miette;

fn main() -> miette::Result<()> {
    let input = include_str!("input.txt");
    let output = process(input)?;
    dbg!(output);
    Ok(())
}

fn process(input: &str) -> Result<usize, AocError> {
    let grid = parse(input)?;

    // Get the grid boundaries and direction of the last step
    let (mut min_coord, mut max_coord) = (Coord::default(), Coord::default());
    let mut cur_coord = Coord { x: 0, y: 0 };
    loop {
        let cell = grid.get(&cur_coord).unwrap();
        cur_coord = cur_coord.move_to(cell.out_dir, cell.len);
        min_coord = Coord {
            x: min_coord.x.min(cur_coord.x),
            y: min_coord.y.min(cur_coord.y),
        };
        max_coord = Coord {
            x: max_coord.x.max(cur_coord.x + 1),
            y: max_coord.y.max(cur_coord.y + 1),
        };
        if cur_coord == Coord::default() {
            break;
        }
    }
    println!("Min: {min_coord:?}, Max: {max_coord:?}");

    let mut count = 0;
    for y in min_coord.y..(max_coord.y + 1) {
        let mut inside = false;
        let mut was_up = false; // initial value does not matter
        for x in min_coord.x..(max_coord.x + 1) {
            match grid.get(&Coord { x, y }) {
                None if inside => {
                    // print!("#");
                    count += 1;
                }
                None => {
                    // print!(".");
                }
                Some(cell) => {
                    // Compute which sides of the cell we traverse
                    // (the input direction is the opposite of the side
                    // traversed, e.g. if we go right, we cross the left boundary)
                    let side1 = cell.out_dir.min(cell.in_dir.reverse());
                    let side2 = cell.out_dir.max(cell.in_dir.reverse());
                    // print!("#");
                    count += 1;
                    match (side1, side2) {
                        (Direction::Up, Direction::Down) => inside = !inside,
                        (Direction::Up, Direction::Left) => {
                            if !was_up {
                                inside = !inside
                            }
                        }
                        (Direction::Up, Direction::Right) => was_up = true,
                        (Direction::Down, Direction::Right) => was_up = false,
                        (Direction::Down, Direction::Left) => {
                            if was_up {
                                inside = !inside
                            }
                        }
                        (Direction::Left, Direction::Right) => {}

                        (Direction::Up, Direction::Up)
                        | (Direction::Down, Direction::Down)
                        | (Direction::Down, Direction::Up)
                        | (Direction::Left, Direction::Down)
                        | (Direction::Left, Direction::Up)
                        | (Direction::Left, Direction::Left)
                        | (Direction::Right, Direction::Up)
                        | (Direction::Right, Direction::Down)
                        | (Direction::Right, Direction::Left)
                        | (Direction::Right, Direction::Right) => {
                            panic!("invalid combination of direction {side1:?}-{side2:?}")
                        }
                    }
                }
            }
        }
        // println!("");
    }

    Ok(count)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() -> miette::Result<()> {
        let input = "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";
        assert_eq!(process(input).unwrap(), 62);

        Ok(())
    }
}
