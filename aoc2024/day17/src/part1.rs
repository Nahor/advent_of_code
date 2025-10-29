use miette::Result;

use crate::parse::parse;

pub fn run(content: &[u8]) -> Result<String> {
    let mut machine = parse(content)?;
    machine.exec();
    Ok(machine.output())
}

#[cfg(test)]
mod test {
    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), "4,6,3,5,6,3,5,2,1,0");
    }

    // #[test]
    // fn compare_base() {
    //     assert_eq!(
    //         run(&INPUT_SAMPLE[1..]).unwrap(),
    //         crate::part1::run(&INPUT_SAMPLE[1..]).unwrap()
    //     );

    //     let input = common::read_input_u8!(None).unwrap();
    //     assert_eq!(run(&input).unwrap(), crate::part1::run(&input).unwrap());
    // }
}
