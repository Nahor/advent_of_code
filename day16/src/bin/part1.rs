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

    println!("Dump map:");
    grid.iter().for_each(|line| {
        print!("    ");
        line.iter().for_each(|cell| {
            print!(
                "{}",
                match cell.mirror {
                    Mirror::None => '.',
                    Mirror::Degree0 => '_',
                    Mirror::Degree45 => '/',
                    Mirror::Degree90 => '|',
                    Mirror::Degree135 => '\\',
                }
            );
        });
        println!("");
    });

    process_cell(&mut grid, (0, 0), Direction::Right);

    println!("Dump energy:");
    grid.iter().for_each(|line| {
        print!("    ");
        line.iter().for_each(|cell| {
            let c = if cell.energy.is_empty() { '.' } else { '#' };
            print!("{}", c);
        });
        println!("");
    });

    let output = grid
        .iter()
        .flatten()
        .filter(|cell| !cell.energy.is_empty())
        .count();

    Ok(output)
}

fn process_cell(grid: &mut Grid, (x, y): (usize, usize), dir: Direction) {
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
        assert_eq!(process(input).unwrap(), 46);

        Ok(())
    }
}
