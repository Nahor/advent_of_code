use day11::{aocerror::AocError, distance_sum, parse};
//use owo_colors::{OwoColorize, Style};

fn main() -> miette::Result<()> {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../advent_of_code_input/2023/",
        env!("CARGO_PKG_NAME"),
        "/input.txt"
    ));
    let output = process(input, 999_999)?;
    dbg!(output);
    Ok(())
}

fn process(input: &str, age: usize) -> Result<isize, AocError> {
    let g_list = parse(input, age)?;

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
        assert_eq!(process(input, 9).unwrap(), 1030);

        Ok(())
    }

    #[test]
    fn test2() -> miette::Result<()> {
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
        assert_eq!(process(input, 99).unwrap(), 8410);

        Ok(())
    }
}
