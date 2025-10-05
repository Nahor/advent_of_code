use day03::aocerror::AocError;
use day03::{parse, CellData};
use owo_colors::{OwoColorize, Style};

fn main() -> miette::Result<()> {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../input/2023/",
        env!("CARGO_PKG_NAME"),
        "/input.txt"
    ));
    let output = process(input)?;
    dbg!(output);
    Ok(())
}

fn process(input: &str) -> Result<u32, AocError> {
    let (parts, grid) = parse(input)?;

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

    Ok(parts
        .iter()
        .map(|part| {
            if part.symbols.borrow().is_empty() {
                0
            } else {
                part.number
            }
        })
        .sum())

    //Ok(4361)
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
        assert_eq!(process(input).unwrap(), 4361);

        Ok(())
    }
}
