use std::iter::successors;

use day18::part2::*;
use miette;

fn main() -> miette::Result<()> {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../advent_of_code_input/2023/",
        env!("CARGO_PKG_NAME"),
        "/input.txt"
    ));
    let output = process(input)?;
    dbg!(output);
    Ok(())
}

fn process(input: &str) -> Result<i64, AocError> {
    let grid = parse(input)?;

    let min = grid.iter().fold(Coord::default(), |min, (coord, _)| Coord {
        x: min.x.min(coord.x),
        y: min.y.min(coord.y),
    });
    let last_dir = grid
        .iter()
        .find_map(|(coord, cell)| {
            if coord.move_to(cell.dir, cell.len) == Coord::default() {
                Some(cell.dir)
            } else {
                None
            }
        })
        .unwrap();

    println!("Min: {min:?}, last_dir: {last_dir:?}");

    // Since we have only "rectangles", the total area is the absolute value of
    // alternate sum of each rectangle's area:
    //      abs(<area1> - <area2> + <area3> - <area4> + ...)
    // which is the same as:
    //      abs(... - (area4 - (area3 - (area2 - area1)))...)
    // To simplify, for the rectangle, we can use the <min_coord>x<cell_coord>
    //
    // But because the cell have a thickness and we don't know which side is
    // "inside" and which is "outside", so compute both areas. The "outside"
    // will be the biggest of the two.
    // We also need to use the direction to know which of the 4 cells corners
    // is inside and which is outside (the other two being part of the "flat wall")
    let (_, left_area, right_area) = successors(Some(Coord::default()), |coord| {
        let cell = grid.get(&coord).unwrap();
        let next_coord = coord.move_to(cell.dir, cell.len);
        if next_coord == Coord::default() {
            None
        } else {
            Some(next_coord)
        }
    })
    .fold(
        (last_dir, 0, 0),
        |(prev_dir, left_area, right_area), coord| {
            let cell = grid.get(&coord).unwrap();
            let (left_corner, right_corner) = match (prev_dir, cell.dir) {
                (Direction::Up, Direction::Left) |  (Direction::Left, Direction::Up) => (
                    Coord {
                        x: coord.x,
                        y: coord.y + 1,
                    },
                    Coord {
                        x: coord.x + 1,
                        y: coord.y,
                    },
                ),
                (Direction::Up, Direction::Right) | (Direction::Right, Direction::Up) => (
                    Coord {
                        x: coord.x,
                        y: coord.y,
                    },
                    Coord {
                        x: coord.x + 1,
                        y: coord.y + 1,
                    },
                ),
                (Direction::Down, Direction::Left) | (Direction::Left, Direction::Down) => (
                    Coord {
                        x: coord.x + 1,
                        y: coord.y + 1,
                    },
                    Coord {
                        x: coord.x,
                        y: coord.y,
                    },
                ),
                (Direction::Down, Direction::Right) |(Direction::Right, Direction::Down)  => (
                    Coord {
                        x: coord.x + 1,
                        y: coord.y,
                    },
                    Coord {
                        x: coord.x,
                        y: coord.y + 1,
                    },
                ),

                (Direction::Up, Direction::Up)
                | (Direction::Up, Direction::Down)
                | (Direction::Down, Direction::Up)
                | (Direction::Down, Direction::Down)
                | (Direction::Left, Direction::Left)
                | (Direction::Left, Direction::Right)
                | (Direction::Right, Direction::Left)
                | (Direction::Right, Direction::Right) => panic!(
                    "invalid direction combination {:?}-{:?}",
                    prev_dir, cell.dir
                ),
            };
            println!(
                "Checking: {prev_dir:?} -> {coord:?}@{cell:?}:\n\t{} * {} - {} = {}\n\t{} * {} - {} = {}",
                (left_corner.x - min.x),
                (left_corner.y - min.y),
                left_area,
                (left_corner.x - min.x) * (left_corner.y - min.y) - left_area,
                (right_corner.x - min.x),
                (right_corner.y - min.y),
                right_area,
                (right_corner.x - min.x) * (right_corner.y - min.y) - right_area,
            );

            (
                cell.dir,
                (left_corner.x - min.x) * (left_corner.y - min.y) - left_area,
                (right_corner.x - min.x) * (right_corner.y - min.y) - right_area,
            )
        },
    );
    println!("areas: {}  vs  {}", left_area.abs(), right_area.abs());

    Ok(left_area.abs().max(right_area.abs()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1a() -> miette::Result<()> {
        // Same as map as part1 but coded using part2
        let input = "\
(#000060)
(#000051)
(#000022)
(#000021)
(#000020)
(#000021)
(#000052)
(#000023)
(#000012)
(#000023)
(#000020)
(#000033)
(#000022)
(#000023)
";
        assert_eq!(process(input).unwrap(), 62);

        Ok(())
    }

    #[test]
    fn test1b() -> miette::Result<()> {
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
        assert_eq!(process(input).unwrap(), 952408144115);

        Ok(())
    }
}
