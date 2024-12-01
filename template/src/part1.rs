use crate::parse::parse;
use miette::Result;

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
bablabla
"#[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 11);
    }
}
