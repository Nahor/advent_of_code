use miette::Result;

use crate::{parse_regex, parse_winnow};

pub fn run(content: &str) -> Result<u64> {
    let list = parse_regex::parse_part1(content)?;

    let result: u64 = list.into_iter().map(|(a, b)| a * b).sum();

    Ok(result)
}

pub fn run_winnow(content: &str) -> Result<u64> {
    let list = parse_winnow::parse_part1(content)?;

    let result: u64 = list.into_iter().map(|(a, b)| a * b).sum();

    Ok(result)
}

#[cfg(test)]
mod test {
    use crate::input::read_input;

    use super::*;

    const INPUT_SAMPLE: &str = r#"
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
"#;

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 161);
    }

    #[test]
    fn winnow() {
        assert_eq!(
            run_winnow(&INPUT_SAMPLE[1..]).unwrap(),
            run(&INPUT_SAMPLE[1..]).unwrap()
        );

        let input = read_input(None).unwrap();
        assert_eq!(run_winnow(&input).unwrap(), run(&input).unwrap());
    }
}
