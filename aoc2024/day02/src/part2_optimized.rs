// Unlike brute-force, try to be more frugal when removing levels, i.e. only
// remove the level around a failure rather than trying all of them.
use miette::Result;

use crate::parse::parse;

pub fn run(content: &str) -> Result<usize> {
    let reports = parse(content)?;

    let result = reports
        .into_iter()
        .enumerate()
        .filter(|(_, report)| is_safe(report))
        .count();

    Ok(result)
}

fn is_safe(report: &[i64]) -> bool {
    let Some(i) = check_error(report) else {
        return true;
    };

    let mut new_report = Vec::from(report);
    new_report.remove(i);
    if check_error(&new_report).is_none() {
        return true;
    }

    let mut new_report = Vec::from(report);
    new_report.remove(i + 1);
    if check_error(&new_report).is_none() {
        return true;
    }

    if i == 1 {
        // Also try by removing 0, since that will change the sign we use
        let new_report = Vec::from(&report[1..]);
        if check_error(&new_report).is_none() {
            return true;
        }
    }

    false
}

// Return the index of the error if any
fn check_error(report: &[i64]) -> Option<usize> {
    let mut sign: Option<i64> = None;
    let mut iter = report.windows(2).enumerate().filter_map(|(i, levels)| {
        let diff = levels[1] - levels[0];
        match diff.abs() {
            x if !(1..=3).contains(&x) =>
            // not in range
            {
                Some(i)
            }
            _ =>
            // the levels are in range
            {
                match sign {
                    None => {
                        // No sign yet, so all good
                        sign = Some(diff.signum());
                        None
                    }
                    Some(sign) if sign == diff.signum() => {
                        // the signs match, so all good
                        None
                    }
                    Some(_) => {
                        // the signs don't match => error
                        Some(i)
                    }
                }
            }
        }
    });
    iter.next()
}

#[cfg(test)]
mod test {
    use common::read_input_str;

    use crate::part2_brute_force;

    use super::*;

    const INPUT_SAMPLE: &str = r#"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 4);
    }

    #[test]
    fn sample_compare() {
        let input = read_input_str!(None).unwrap();
        assert_eq!(
            run(&input).unwrap(),
            part2_brute_force::run(&input).unwrap()
        );
    }
}
