// Same as optimize2_skip but avoid checking the tuples before the skipped
// level (since they haven't changed)
// This is currently slower, probably because the reports are too short and we
// don't save enough compared to the extra cost of the window filtering.
use itertools::Itertools;
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
    let Some(i) = check_error(report, usize::MAX) else {
        return true;
    };

    // Try removing a level around the failure location
    if check_error(report, i).is_none() {
        return true;
    }
    if check_error(report, i + 1).is_none() {
        return true;
    }

    if i == 1 {
        // Also try by removing 0, since that could change the sign/direction
        if check_error(report, 0).is_none() {
            return true;
        }
    }

    false
}

// Return the index of the error if any
fn check_error(report: &[i64], skip: usize) -> Option<usize> {
    let mut sign: Option<i64> = None;
    let mut iter = report
        .iter()
        .cloned()
        .enumerate()
        .filter_map(|(level_idx, v)| if level_idx == skip { None } else { Some(v) })
        .tuple_windows()
        .enumerate()
        .filter(|(window_idx, _)| {
            // Skip the windows that have already been checked previously
            match (window_idx, skip) {
                (_, usize::MAX) => true, // Not skipping
                (0, _) => true, // Always keep the first window, since that defines the sign/direction
                (window_idx, skip) if window_idx + 1 < skip => {
                    // The condition above is OK since the window index is the
                    // same as the level index until the skipped level.
                    // This also means that the window has already been checked
                    // before and can be skipped
                    false
                }
                (_, _) => true,
            }
        })
        .filter_map(|(i, (l1, l2))| {
            let diff = l1 - l2;
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
