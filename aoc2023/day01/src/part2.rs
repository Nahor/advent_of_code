use regex::Regex;

use crate::aocerror::{AocError, AocSourceChunk};

pub fn part2(input: &str) -> Result<u32, AocError> {
    let re_first =
        Regex::new(r"([0-9]|zero|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let re_last =
        Regex::new(r".*([0-9]|zero|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    input
        .lines()
        .enumerate()
        .filter(|&(_, s)| !s.is_empty())
        .map(|(lineno, line)| {
            let first = re_first
                .captures(line)
                .ok_or_else(|| AocError::InputError {
                    src: AocSourceChunk::new(line.to_owned(), lineno),
                    bad_bit: (0, line.len()).into(),
                    inner: Some(Box::new(AocError::NoDigit)),
                })?
                .get(1)
                .expect("Invalid first regex");
            let second = re_last
                .captures(line)
                .ok_or_else(|| AocError::InputError {
                    src: AocSourceChunk::new(line.to_owned(), lineno),
                    bad_bit: (0, line.len()).into(),
                    inner: Some(Box::new(AocError::NoDigit)),
                })?
                .get(1)
                .expect("Invalid second regex");
            let val_first = convert(first.as_str());
            let val_second = convert(second.as_str());
            Ok(val_first * 10 + val_second)
        })
        .sum()
}

fn convert(s: &str) -> u32 {
    match s {
        "0" | "zero" => 0,
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => panic!("Invalid digit string ('{s}'), bad regex"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() -> miette::Result<(), AocError> {
        let input = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let output = part2(input)?;
        assert_eq!(output, 281);
        Ok(())
    }
}
