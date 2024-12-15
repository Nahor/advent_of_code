use day06::{aocerror::AocError, part1::parse};
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
    let data = dbg!(parse(input)?);

    assert_eq!(data.time.len(), data.distance.len());
    let output = (0..data.time.len())
        .map(|index| {
            let race_time = data.time[index];
            let race_distance = data.distance[index];
            (1..race_time)
                .map(|time| time * (race_time - time))
                .filter(|distance| distance > &race_distance)
                .count()
        })
        .product::<usize>();

    Ok(output)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() -> miette::Result<()> {
        let input = "\
Time:      7  15   30
Distance:  9  40  200
";
        assert_eq!(process(input).unwrap(), 288);

        Ok(())
    }
}
