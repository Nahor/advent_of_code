use day_template::*;

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
    let (size, size_other) = input.lines().fold((0, 0), |(max_y, max_x), line| {
        let width = line.len();
        assert!(max_y == 0 || width == max_x);
        (max_y + 1, width)
    });
    assert_eq!(size, size_other);

    let _data = parse(input)?;
    let output = 1234;

    Ok(output)
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
        assert_eq!(process(input).unwrap(), 1234);

        Ok(())
    }
}
