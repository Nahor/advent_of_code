use miette::Result;

use crate::{op::Op, parse_regex, parse_winnow};

pub fn run(content: &str) -> Result<u64> {
    let ops = parse_regex::parse_part2(content)?;
    Ok(compute(ops))
}

pub fn run_winnow(content: &str) -> Result<u64> {
    let ops = parse_winnow::parse_part2(content)?;
    Ok(compute(ops))
}

pub fn run_winnow_skip(content: &str) -> Result<u64> {
    let ops = parse_winnow::parse_part2_skip(content)?;
    Ok(ops.into_iter().map(|(a, b)| a * b).sum())
}

pub fn run_winnow_compute(content: &str) -> Result<u64> {
    Ok(parse_winnow::parse_part2_compute(content)?)
}

fn compute(ops: Vec<Op>) -> u64 {
    ops.into_iter()
        .scan(true, |enabled, op| {
            let r = match (*enabled, op) {
                (true, Op::Mul(a, b)) => Some(a * b),
                (false, Op::Mul(_, _)) => None,
                (_, Op::Dont) => {
                    *enabled = false;
                    None
                }
                (_, Op::Do) => {
                    *enabled = true;
                    None
                }
            };
            Some(r)
        })
        .flatten()
        .sum::<u64>()
}

#[cfg(test)]
mod test {
    use crate::input::read_input;

    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &str = r#"
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 48);
    }

    #[test]
    fn parsers() {
        assert_eq!(
            parse_regex::parse_part2(&INPUT_SAMPLE[1..]).unwrap(),
            parse_winnow::parse_part2(&INPUT_SAMPLE[1..]).unwrap()
        );

        let input = read_input(None).unwrap();
        assert_eq!(
            parse_regex::parse_part2(&input).unwrap(),
            parse_winnow::parse_part2(&input).unwrap()
        );
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

    #[test]
    fn winnow_skip() {
        assert_eq!(
            run_winnow(&INPUT_SAMPLE[1..]).unwrap(),
            run(&INPUT_SAMPLE[1..]).unwrap()
        );

        let input = read_input(None).unwrap();
        assert_eq!(run_winnow_skip(&input).unwrap(), run(&input).unwrap());
    }

    #[test]
    fn winnow_compute() {
        assert_eq!(
            run_winnow(&INPUT_SAMPLE[1..]).unwrap(),
            run(&INPUT_SAMPLE[1..]).unwrap()
        );

        let input = read_input(None).unwrap();
        assert_eq!(run_winnow_compute(&input).unwrap(), run(&input).unwrap());
    }
}
