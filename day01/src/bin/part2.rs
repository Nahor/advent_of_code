use day01::part2::*;
use miette::Result;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    let output = part2(input)?;
    dbg!(output);

    Ok(())
}
