use day11::{aocerror::AocError, distance_sum, parse};
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

fn process(input: &str) -> Result<isize, AocError> {
    let g_list = parse(input, 1)?;

    let output = distance_sum(g_list);

    Ok(output)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() -> miette::Result<()> {
        let input = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
        assert_eq!(process(input).unwrap(), 374);

        Ok(())
    }
}
