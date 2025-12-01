use miette::Result;

use crate::parse::parse;

pub fn run(content: &[u8]) -> Result<usize> {
    let lines = parse(content)?;

    let password = lines
        .into_iter()
        .scan(50, |dial, rotation| {
            *dial = (*dial + rotation).rem_euclid(100);
            Some(*dial)
        })
        .filter(|dial| *dial == 0)
        .count();

    Ok(password)
}

#[cfg(test)]
mod test {
    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 3);
    }

    // #[test]
    // fn compare_base() {
    //     assert_eq!(
    //         run(&INPUT_SAMPLE[1..]).unwrap(),
    //         crate::part1::run(&INPUT_SAMPLE[1..]).unwrap()
    //     );

    //     let input = common::read_input_u8!(None).unwrap();
    //     assert_eq!(run(&input).unwrap(), crate::part2::run(&input).unwrap());
    // }
}
