use miette::Result;

use crate::parse::parse;

// result: <dial, clicks>
pub fn rotate(dial: i32, rotation: i32) -> (i32, i32) {
    let full_dial = dial + rotation;
    let new_dial = full_dial.rem_euclid(100);

    let clicks =
        // not div_euclid because we want to behave like positive for numbers
        // i.e. only count the clicks for -100, -200, ...
        (full_dial / 100).abs()
        // +1 to account for crossing from positive to negative numbers
        + if full_dial <= 0 && dial != 0 { 1 } else { 0 };

    (new_dial, clicks)
}

pub fn run(content: &[u8]) -> Result<i32> {
    let lines = parse(content)?;

    let password = lines
        .into_iter()
        .scan(50, |dial, rotation| {
            let (new_dial, clicks) = rotate(*dial, rotation);
            *dial = new_dial;
            Some(clicks)
        })
        .sum();

    Ok(password)
}

#[cfg(test)]
mod test {
    use rstest::rstest;

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

    #[rstest]
    #[case(99, 1, (0, 1))]
    #[case(1, -1, (0, 1))]
    #[case(99, 2, (1, 1))]
    #[case(1, -2, (99, 1))]
    #[case(0, 1, (1, 0))]
    #[case(0, -1, (99, 0))]
    #[case(0, 100, (0, 1))]
    #[case(0, -100, (0, 1))]
    fn corner(#[case] dial: i32, #[case] rotation: i32, #[case] expected: (i32, i32)) {
        assert_eq!(crate::part2_euclid::rotate(dial, rotation), expected);
    }

    #[rstest]
    #[case(50, -68, (82, 1))]
    #[case(82, -30, (52, 0))]
    #[case(52, 48, (0, 1))]
    #[case(0, -5, (95, 0))]
    #[case(95, 60, (55, 1))]
    #[case(55, -55, (0, 1))]
    #[case(0, -1, (99, 0))]
    #[case(99, -99, (0, 1))]
    #[case(0, 14, (14, 0))]
    #[case(14, -82, (32, 1))]
    fn sample_rotate(#[case] dial: i32, #[case] rotation: i32, #[case] expected: (i32, i32)) {
        assert_eq!(crate::part2_euclid::rotate(dial, rotation), expected);
    }

    #[test]
    fn multi() {
        let input = br#"R1000"#; // remove leading \n

        assert_eq!(run(input).unwrap(), 10);
    }

    #[test]
    fn compare_base() {
        assert_eq!(
            run(&INPUT_SAMPLE[1..]).unwrap(),
            crate::part2::run(&INPUT_SAMPLE[1..]).unwrap()
        );

        let input = common::read_input_u8!(None).unwrap();
        assert_eq!(run(&input).unwrap(), crate::part2::run(&input).unwrap());
    }
}
