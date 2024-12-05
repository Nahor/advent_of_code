// Same as optimize but create the modified reports by skipping the unwanted
// level rather than cloning the whole report then removing the level.
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

    let mut new_report = Vec::with_capacity(report.len() - 1);
    new_report.extend(&report[..i]);
    new_report.extend(&report[(i + 1)..]);
    if check_error(&new_report).is_none() {
        return true;
    }

    new_report.clear();
    new_report.extend(&report[..(i + 1)]);
    new_report.extend(&report[(i + 2)..]);
    if check_error(&new_report).is_none() {
        return true;
    }

    if i == 1 {
        // Also try by removing 0, since that will change the sign we use
        new_report.clear();
        new_report.extend(&report[1..]);
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
    use common::input::read_input_str;

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
        let input = read_input_str(None).unwrap();
        assert_eq!(
            run(&input).unwrap(),
            part2_brute_force::run(&input).unwrap()
        );
    }
}
