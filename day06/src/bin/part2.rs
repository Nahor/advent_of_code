use day06::{aocerror::AocError, part2::parse};
use miette;
//use owo_colors::{OwoColorize, Style};

fn main() -> miette::Result<()> {
    let input = include_str!("input.txt");
    let output = process(input)?;
    dbg!(output);
    Ok(())
}

fn process(input: &str) -> Result<u64, AocError> {
    let data = dbg!(parse(input)?);

    let race_time = data.time;
    let race_distance = data.distance;
    let first = (1..race_time)
        .take_while(|time| {
            let distance = time * (race_time - time);
            distance <= race_distance
        })
        .last()
        .unwrap()
        + 1;
    let last = (1..race_time)
        .rev()
        .take_while(|time| {
            let distance = time * (race_time - time);
            distance <= race_distance
        })
        .last()
        .unwrap();

    println!("{first} - {last}");

    Ok(last - first)
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
        assert_eq!(process(input).unwrap(), 71503);

        Ok(())
    }
}
