use miette::Result;

use crate::parse::parse;

pub fn run(content: &str) -> Result<usize> {
    let reports = parse(content)?;

    let result = reports.into_iter().filter(|report| is_safe(report)).count();

    Ok(result)
}

fn is_safe(report: &[i64]) -> bool {
    let mut iter = report.windows(2).map(|levels| levels[1] - levels[0]);

    let Some(first) = iter.next() else {
        return false;
    };
    if !(1..=3).contains(&first.abs()) {
        return false;
    }
    let sign = first.signum(); // only 1 or -1 since first != 0
    iter.all(|diff| diff.abs() <= 3 && diff.signum() == sign)
}

#[cfg(test)]
mod test {
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

        assert_eq!(run(input).unwrap(), 2);
    }
}
