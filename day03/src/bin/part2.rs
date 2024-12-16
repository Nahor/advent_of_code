use std::collections::HashSet;
use std::rc::Rc;

use day03::aocerror::AocError;
use day03::{parse, CellData, Part};
use owo_colors::{OwoColorize, Style};
use std::hash::Hash;

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

struct RcWrapper(Rc<Part>);
impl Hash for RcWrapper {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Rc::<Part>::as_ptr(&self.0).hash(state);
    }
}
impl Eq for RcWrapper {}
impl PartialEq for RcWrapper {
    fn eq(&self, other: &Self) -> bool {
        Rc::<Part>::ptr_eq(&self.0, &other.0)
    }
}

fn process(input: &str) -> Result<u32, AocError> {
    let (_, grid) = parse(input)?;

    let mut last_y = 0;
    let mut last_x = -1;
    grid.iter().for_each(|(coord, cell)| {
        if coord.y != last_y {
            println!();
            last_y = coord.y;
            last_x = -1;
        }
        for _ in last_x + 1..coord.x {
            print!("{}", '.'.red());
        }
        last_x = coord.x;
        match cell {
            CellData::Empty => panic!("not expecting CellData::Empty"),
            CellData::Symbol(c) => {
                print!("{}", c.red())
            }
            CellData::Number(part) => {
                let style = if part.symbols.borrow().is_empty() {
                    Style::new().yellow()
                } else {
                    Style::new().green()
                };
                match grid.get(&coord.add(-1, 0)) {
                    Some(CellData::Number(_)) => {}
                    Some(_) => print!("{}", part.number.style(style)),
                    None => print!("{}", part.number.style(style)),
                }
            }
        }
    });
    println!();

    let offsets = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    let output: u32 = grid
        .iter()
        .filter(|(_, cell)| matches!(cell, CellData::Symbol('*')))
        .map(|(&coord, _)| {
            // Allowing the lint. Didn't check if there is a better option
            #[allow(clippy::mutable_key_type)]
            let mut found = HashSet::new();
            for offset in offsets.iter() {
                match grid.get(&coord.add(offset.0, offset.1)) {
                    Some(CellData::Number(part)) => {
                        found.insert(RcWrapper(Rc::clone(part)));
                    }
                    Some(_) => {}
                    None => {}
                };
            }
            if found.len() != 2 {
                return 0;
            }
            let mut iter = found.iter();
            iter.next().unwrap().0.number * iter.next().unwrap().0.number
        })
        .sum();
    Ok(output)
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // #[should_panic]
    // fn no_header() {
    //     let input = "Game 1: foo\nbar";
    //     process(input).unwrap();
    // }

    #[test]
    fn test1() -> miette::Result<()> {
        let input = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        assert_eq!(process(input).unwrap(), 467835);

        Ok(())
    }
}
