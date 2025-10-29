// Same as optimized2 but use two passes, one for each direction with the goal
// of removing a branch. However, this is slower for some reason.
use miette::Result;

use crate::parse::parse;

pub fn run(content: &str) -> Result<usize> {
    let reports = parse(content)?;

    let result = reports
        .into_iter()
        .enumerate()
        .filter(|(_, report)| is_safe::<true>(report) || is_safe::<false>(report))
        .count();

    Ok(result)
}

fn is_safe<const DIR: bool>(report: &[i64]) -> bool {
    let Some(i) = check_error::<DIR>(report) else {
        return true;
    };

    let mut new_report = Vec::with_capacity(report.len() - 1);
    new_report.extend(&report[..i]);
    new_report.extend(&report[(i + 1)..]);
    if check_error::<DIR>(&new_report).is_none() {
        return true;
    }

    new_report.clear();
    new_report.extend(&report[..(i + 1)]);
    new_report.extend(&report[(i + 2)..]);
    if check_error::<DIR>(&new_report).is_none() {
        return true;
    }

    false
}

// Return the index of the error if any
fn check_error<const DIR: bool>(report: &[i64]) -> Option<usize> {
    let iter = report.windows(2).enumerate();

    iter.filter_map(|(i, levels)| {
        let diff = levels[1] - levels[0];
        if (DIR && !(1..=3).contains(&diff)) || (!DIR && !(-3..=-1).contains(&diff)) {
            // outside the range for the given direction
            Some(i)
        } else {
            // the signs match, so all good
            None
        }
    })
    .next()
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
