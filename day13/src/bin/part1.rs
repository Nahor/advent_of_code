use day13::{aocerror::AocError, parse};
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
    let output = parse(input, 0)?;

    Ok(output)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() -> miette::Result<()> {
        let input = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";
        assert_eq!(process(input).unwrap(), 405);

        Ok(())
    }

    #[test]
    fn test2() -> miette::Result<()> {
        let input = "\
.#.####.#.#
...####....
....##....#
.#.####.#.#
.#..##..#.#
##......#.#
.#..##..#.#
####..####.
.########..
.#..##..#.#
..######..#
##.####.###
##.####.###
";
        assert_eq!(process(input).unwrap(), 1200);

        Ok(())
    }
}
