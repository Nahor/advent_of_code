use day13::{aocerror::AocError, parse};
//use owo_colors::{OwoColorize, Style};

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

fn process(input: &str) -> Result<usize, AocError> {
    let output = parse(input, 1)?;

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
        assert_eq!(process(input).unwrap(), 400);

        Ok(())
    }
}
