use day09::{aocerror::AocError, parse};
use itertools::Itertools;
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

fn process(input: &str) -> Result<i64, AocError> {
    let data = dbg!(parse(input)?);

    let output = data.into_iter().map(|line| get_next(&line)).sum();

    Ok(output)
}

fn get_next(data: &[i64]) -> i64 {
    if data.iter().all(|v| (*v) == 0) {
        return 0;
    }

    let diff = data
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect::<Vec<_>>();

    let inc = get_next(&diff);
    data.last().unwrap() + inc
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() -> miette::Result<()> {
        let input = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
        assert_eq!(process(input).unwrap(), 114);

        Ok(())
    }
}
