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

    const INPUT_SAMPLE: &str = r#"
bla
bla
"#;

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 11);
    }

    // #[test]
    // fn sample_sorted() {
    //     assert_eq!(
    //         run_sorted(&INPUT_SAMPLE[1..]).unwrap(),
    //         run(&INPUT_SAMPLE[1..]).unwrap()
    //     );

    //     let input = read_input(None).unwrap();
    //     assert_eq!(run_sorted(&input).unwrap(), run(&input).unwrap());
    // }
}
