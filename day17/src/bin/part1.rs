use std::{
    collections::{BTreeMap, HashSet},
    fmt::Display,
};

use day17::*;
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

fn process(input: &str) -> Result<u32, AocError> {
    let grid = parse(input)?;
    let output = min_heat_loss(&grid);

    Ok(output)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn reverse(&self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
    fn move_from(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Direction::Left => (x.wrapping_sub(1), y),
            Direction::Right => (x.wrapping_add(1), y),
            Direction::Up => (x, y.wrapping_sub(1)),
            Direction::Down => (x, y.wrapping_add(1)),
        }
    }
}
impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Left => '<',
                Direction::Right => '>',
                Direction::Up => '^',
                Direction::Down => 'V',
            }
        )
    }
}

fn min_heat_loss(grid: &Vec<Vec<u32>>) -> u32 {
    let height = grid.len();
    let width = grid[0].len();

    // let mut loss_path = vec![vec![None; width]; height];
    // K = (x, y, dir, count)
    let mut done = HashSet::new();

    // K = heat loss so far
    // V = ((x, y), dir, dir_count, vec)
    let mut pending = BTreeMap::new();
    pending.insert(
        0,
        vec![
            ((1, 0), Direction::Right, 1, vec![(0, 0, 0)]),
            ((0, 1), Direction::Down, 1, vec![(0, 0, 0)]),
        ],
    );

    while !pending.is_empty() {
        //while let Some((loss, ((x, y), dir, dir_count))) = pending.pop_first().and_then(|v| v.pop()) {

        let (loss, ((x, y), dir, dir_count, mut path)) = {
            let mut entry = pending.first_entry().unwrap();
            let loss = *entry.key();
            let path = entry.get_mut().pop().unwrap();
            if entry.get().is_empty() {
                let _ = entry.remove_entry();
            }
            // println!("Processing {loss}-{path:?}");
            (loss, path)
        };

        let cell_loss = match grid.get(y).and_then(|line| line.get(x)) {
            Some(cell_loss) => cell_loss,
            None => continue, // out of bound
        };

        if !done.insert((x, y, dir, dir_count)) {
            // Already processed that combination
            continue;
        }

        let new_loss = loss + cell_loss;
        path.push((x, y, new_loss));

        // Store the loss in the path
        // let loss_cell = loss_path
        //     .get_mut(y)
        //     .and_then(|line| line.get_mut(x))
        //     .unwrap();
        // if let Some(_) = loss_cell {
        //     // Already found a shorter path
        //     continue;
        // } else {
        //     *loss_cell = Some((dir, new_loss));
        // }

        if x == (width - 1) && y == (height - 1) {
            // Reach the destination
            // loss_path.iter().enumerate().for_each(|(_, line)| {
            //     line.iter().enumerate().for_each(|(_, loss_opt)| {
            //         match loss_opt {
            //             Some((dir, loss)) => print!("[{dir}{:>3}]", loss),
            //             None => print!("[    ]"),
            //         };
            //     });
            //     println!("");
            // });
            println!("Path: {path:?}");
            return new_loss;
        }

        [
            Direction::Left,
            Direction::Right,
            Direction::Up,
            Direction::Down,
        ]
        .into_iter()
        .for_each(|new_dir| {
            if new_dir == dir.reverse() {
                // Can't go back
                return;
            }
            if new_dir == dir {
                if dir_count == 3 {
                    // Too far, can't go this direction anymore
                    return;
                }
                // println!("Cont {dir:?} after {dir_count}");
                pending.entry(new_loss).or_default().push((
                    dir.move_from(x, y),
                    dir,
                    dir_count + 1,
                    path.clone(),
                ));
                return;
            }
            // println!("Change from {dir:?} {new_dir:?}");
            pending.entry(new_loss).or_default().push((
                new_dir.move_from(x, y),
                new_dir,
                1,
                path.clone(),
            ));
        });
        // println!("pending: {pending:?}");
    }

    panic!("Shouldn't be here: {pending:?}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() -> miette::Result<()> {
        let input = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";
        assert_eq!(process(input).unwrap(), 102);

        Ok(())
    }
}
