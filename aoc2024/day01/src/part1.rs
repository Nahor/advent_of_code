use crate::parse::parse;
use miette::Result;

pub fn run(content: &str) -> Result<u64> {
    let lines = parse(content)?;
    let (mut first, mut second): (Vec<_>, Vec<_>) = lines.into_iter().unzip();
    first.sort();
    second.sort();
    let result: u64 = first
        .into_iter()
        .zip(second)
        .map(|(first, second)| first.abs_diff(second))
        .sum();

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_SAMPLE: &str = r#"
3   4
4   3
2   5
1   3
3   9
3   3
"#;

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 11);
    }
}
