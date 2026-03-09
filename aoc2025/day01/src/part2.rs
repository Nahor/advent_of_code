use miette::Result;

use crate::parse::parse;

pub fn run(content: &[u8]) -> Result<i32> {
    let lines = parse(content)?;

    let password = lines
        .into_iter()
        .scan(50, |dial, rotation| {
            // When rotating left, mirror the rotation so that we don't need to
            // deal with negative number or worry about the corner case of 0
            let (abs_dial, abs_rotation) = if rotation < 0 {
                ((100 - *dial) % 100, -rotation)
            } else {
                (*dial, rotation)
            };

            let mut new_abs_dial = abs_dial + abs_rotation;
            let clicks = new_abs_dial / 100;
            new_abs_dial %= 100;

            *dial = if rotation < 0 {
                (100 - new_abs_dial) % 100
            } else {
                new_abs_dial
            };

            Some(clicks)
        })
        .sum();

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

        assert_eq!(run(input).unwrap(), 6);
    }

    #[test]
    fn multi() {
        let input = br#"R1000"#; // remove leading \n

        assert_eq!(run(input).unwrap(), 10);
    }

    // #[test]
    // fn compare_base() {
    //     assert_eq!(
    //         run(&INPUT_SAMPLE[1..]).unwrap(),
    //         crate::part2::run(&INPUT_SAMPLE[1..]).unwrap()
    //     );

    //     let input = common::read_input_u8!(None).unwrap();
    //     assert_eq!(run(&input).unwrap(), crate::part2::run(&input).unwrap());
    // }
}
