use day16::*;
use miette;

fn main() -> miette::Result<()> {
    let input = include_str!("input.txt");

    let output = process(input)?;
    dbg!(output);
    Ok(())
}

fn process(input: &str) -> Result<usize, AocError> {
    let mut grid = parse(input)?;

    let height = grid.len();
    let width = grid[0].len();
    let output = (0..height)
        .into_iter()
        .map(|y| ((0, y), Direction::Right))
        .chain(
            (0..height)
                .into_iter()
                .map(|y| ((width - 1, y), Direction::Left)),
        )
        .chain((0..width).into_iter().map(|x| ((x, 0), Direction::Down)))
        .chain(
            (0..width)
                .into_iter()
                .map(|x| ((x, height - 1), Direction::Up)),
        )
        .map(|(cell, dir)| get_energy_level(&mut grid, cell, dir))
        .max()
        .unwrap();

    Ok(output)
}

fn get_energy_level(grid: &mut Grid, cell: (usize, usize), dir: Direction) -> usize {
    // Reset the grid energy level
    grid.iter_mut()
        .flatten()
        .for_each(|cell| cell.energy = Default::default());

    // Energize
    energize(grid, cell, dir);

    // Compute energy level
    grid.iter()
        .flatten()
        .filter(|cell| !cell.energy.is_empty())
        .count()
}

fn energize(grid: &mut Grid, (x, y): (usize, usize), dir: Direction) {
    let mut pending = vec![(x, y, dir)];

    while let Some((x, y, dir)) = pending.pop() {
        // Get the cell (return if out-of-bounds)
        let cell = match grid.get_mut(y).and_then(|line| line.get_mut(x)) {
            Some(cell) => cell,
            None => continue,
        };

        // Check if that cell side is already energized (i.e. already processed)
        if cell.energy.contains(dir) {
            continue;
        }

        // Energize the input side
        cell.energy |= dir;

        // Determine output direction(s)
        let next_dir = match (cell.mirror, dir) {
            (Mirror::None, _)
            | (Mirror::Degree0, Direction::Left)
            | (Mirror::Degree0, Direction::Right)
            | (Mirror::Degree90, Direction::Up)
            | (Mirror::Degree90, Direction::Down) => vec![dir],

            (Mirror::Degree45, Direction::Left) | (Mirror::Degree135, Direction::Right) => {
                vec![Direction::Down]
            }
            (Mirror::Degree45, Direction::Right) | (Mirror::Degree135, Direction::Left) => {
                vec![Direction::Up]
            }
            (Mirror::Degree45, Direction::Up) | (Mirror::Degree135, Direction::Down) => {
                vec![Direction::Right]
            }
            (Mirror::Degree45, Direction::Down) | (Mirror::Degree135, Direction::Up) => {
                vec![Direction::Left]
            }

            (Mirror::Degree90, Direction::Left) | (Mirror::Degree90, Direction::Right) => {
                vec![Direction::Up, Direction::Down]
            }
            (Mirror::Degree0, Direction::Up) | (Mirror::Degree0, Direction::Down) => {
                vec![Direction::Left, Direction::Right]
            }
        };

        // Process that direction
        next_dir.into_iter().for_each(|dir| {
            match dir {
                Direction::Left => pending.push((x.wrapping_sub(1), y, dir)),
                Direction::Right => pending.push((x.wrapping_add(1), y, dir)),
                Direction::Up => pending.push((x, y.wrapping_sub(1), dir)),
                Direction::Down => pending.push((x, y.wrapping_add(1), dir)),
            };
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() -> miette::Result<()> {
        let input = r#"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#;
        assert_eq!(process(input).unwrap(), 51);

        Ok(())
    }
}
