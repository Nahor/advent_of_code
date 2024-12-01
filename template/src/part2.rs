use miette::Result;

use crate::parse::parse;

pub fn run(content: &str) -> Result<u64> {
    let _lines = parse(content)?;

    let result: u64 = 0;

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let input = &r#"
blablabla
"#[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 31);
    }
}
