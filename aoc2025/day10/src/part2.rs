use common::error::AdventError;
use miette::Result;

use crate::{int::Machine, parse::int::parse};

fn process_button(machine: &Machine, button: usize, missing_joltage: &[i64]) -> Option<i64> {
    // Check for stopping condition
    if missing_joltage.iter().all(|j| *j == 0) {
        // We have a match
        return Some(0);
    };
    if button >= machine.buttons.len() {
        return None;
    }

    let press_range = 0..=missing_joltage
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(idx, j)| {
            if (machine.buttons[button] & (1 << idx)) == 0 {
                None
            } else {
                Some(j)
            }
        })
        .min()?;

    let mut new_missing = vec![0; missing_joltage.len()];
    let mut min_press = i64::MAX;
    for press in press_range {
        // let prefix = format!("{prefix}{press}");
        for idx in 0..new_missing.len() {
            new_missing[idx] =
                missing_joltage[idx] - press * ((machine.buttons[button] & (1 << idx)) != 0) as i64;

            // Because we capped the range, this should never be negative
            assert!(new_missing[idx] >= 0);
        }
        let r = process_button(machine, button + 1, &new_missing);
        if let Some(sub_press) = r {
            min_press = min_press.min(press + sub_press);
        }
    }

    if min_press == i64::MAX {
        // no solution
        None
    } else {
        Some(min_press)
    }
}

fn process_machine(machine: &Machine) -> Result<i64, AdventError> {
    let v = machine
        .joltage
        .iter()
        .map(|j| *j as i64)
        .collect::<Vec<_>>();
    let r = process_button(machine, 0, &v).ok_or("couldn't find mapping")?;
    Ok(r)
}

pub fn run(content: &[u8]) -> Result<i64, AdventError> {
    let machines = parse(content.into())?;

    let result = machines
        .iter()
        .map(process_machine)
        .sum::<Result<i64, _>>()?;
    Ok(result)
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 33);
    }

    #[rstest]
    #[case(b"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}", 10)]
    #[case(b"[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}", 12)]
    #[case(
        b"[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        11
    )]
    fn single(#[case] input: &[u8], #[case] expected: i64) {
        assert_eq!(run(input).unwrap(), expected);
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
