use day01::part2::*;
use miette::Result;

fn main() -> Result<()> {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../advent_of_code_input/2023/",
        env!("CARGO_PKG_NAME"),
        "/input.txt"
    ));
    let output = part2(input)?;
    dbg!(output);

    Ok(())
}
