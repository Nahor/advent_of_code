use std::collections::{BTreeMap, HashMap};

use day14::*;
use miette;
//use owo_colors::{OwoColorize, Style};

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

fn process(input: &str) -> Result<usize, AocError> {
    let (size, size_other) = input.lines().fold((0, 0), |(max_y, max_x), line| {
        let width = line.len();
        assert!(max_y == 0 || width == max_x);
        (max_y + 1, width)
    });
    assert_eq!(size, size_other);

    let mut rock_map = parse(input)?;
    if size <= 10 {
        println!("{}", get_map_drawing(&rock_map, size));
    }

    let mut cache = HashMap::new();

    const CYCLE_COUNTS: usize = 1_000_000_000;
    let mut iter = (0..CYCLE_COUNTS).into_iter();
    while let Some(i) = iter.next() {
        // println!("Processing cycle {i}");
        rock_map = process_one_cycle(rock_map, size, i);
        if size <= 10 {
            println!("{}", get_map_drawing(&rock_map, size));
        }

        let entry = cache.get(&rock_map);
        match entry {
            Some(start_cycle) => {
                let loop_size = i - start_cycle;
                let remaining_loops = (CYCLE_COUNTS - i) / loop_size;
                let new_i = i + remaining_loops * loop_size;
                println!(
                    "Found loop at {}, looping to {} (size: {})",
                    i, start_cycle, loop_size
                );
                println!("Skipping {} loop ([{}..{}])", remaining_loops, i + 1, new_i);

                // -1 since "nth" starts at 0, "0" means "skips one cycle"
                if remaining_loops > 0 {
                    let _ = iter.nth(remaining_loops * loop_size - 1);
                }

                break;
            }
            None => {
                cache.insert(rock_map.clone(), i);
            }
        }
    }
    //let mut iter = iter.chain((CYCLE_COUNTS..(CYCLE_COUNTS + 10)).into_iter());
    while let Some(i) = iter.next() {
        // println!("Finishing with cycle {i}");
        rock_map = process_one_cycle(rock_map, size, i);
    }

    let load = compute_load_before_tilt(&rock_map, size);
    let loads = compute_loads(&rock_map, size);
    println!(
        "Load without moving: {load}  (N: {}, W: {}, S: {}, E: {})",
        loads.0, loads.1, loads.2, loads.3
    );

    Ok(load)
}

fn process_one_cycle(
    rock_map: BTreeMap<Coord, RockType>,
    size: usize,
    _cycle_number: usize,
) -> BTreeMap<Coord, RockType> {
    let mut rock_map = rock_map;
    for _ in 0..4 {
        rock_map = process_one_direction(rock_map, size);
    }
    rock_map
}

fn get_map_drawing(rock_map: &BTreeMap<Coord, RockType>, size: usize) -> String {
    (0..size)
        .into_iter()
        .map(|y| {
            (0..size)
                .into_iter()
                .map(|x| match rock_map.get(&Coord { x, y }) {
                    Some(RockType::Round) => 'O',
                    Some(RockType::Square) => '#',
                    None => '.',
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() -> miette::Result<()> {
        let input = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";
        assert_eq!(process(input).unwrap(), 64);

        Ok(())
    }
}
