use common::error::AdventError;
use itertools::Itertools;
use miette::Result;

use crate::parse::parse;

pub fn run(content: &[u8]) -> Result<i64, AdventError> {
    let points = parse(content)?;

    Ok(points
        .iter()
        .tuple_combinations()
        .map(|(p1, p2)| {
            let d = (p2 - p1).abs();
            ((d.x + 1.0) * (d.y + 1.0)) as i64
        })
        .max()
        .ok_or("No box found")?)
}

#[cfg(test)]
mod test {
    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 50);
    }
}
