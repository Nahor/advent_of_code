use bitvec::prelude::BitStore;
use common::error::AdventError;
use itertools::Itertools;
use miette::Result;
use rayon::prelude::*;

use crate::{int::Machine, parse::int::parse};

const TIMEOUT: std::time::Duration = std::time::Duration::from_secs(60);

fn process_button(
    machine: &Machine,
    button: usize,
    missing_joltage: &[i64],
    start: std::time::Instant,
) -> Option<i64> {
    // Check for stopping condition
    if missing_joltage.iter().all(|j| *j == 0) {
        // We have a match
        return Some(0);
    };
    if button >= machine.buttons.len() {
        return None;
    }

    // The max number of press for this button is the lowest missing joltage
    // incremented by that button.
    let max_press = missing_joltage
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

    // Find which joltage must be satisfied by this button, i.e. which joltages
    // don't have any more buttons after this one to increment them.
    let remaining_button_mask = machine.buttons[button + 1..]
        .iter()
        .copied()
        .reduce(|a, b| a | b)
        .unwrap_or_default();
    let needed_joltages = machine.buttons[button] & !remaining_button_mask;
    let needed_minmax = missing_joltage
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(idx, j)| {
            if (needed_joltages & (1 << idx)) == 0 {
                None
            } else {
                Some(j)
            }
        })
        .minmax()
        .into_option();
    let press_range = if let Some((min, max)) = needed_minmax {
        if min != max {
            // We have different joltages amounts that must be satisfied by this
            // button => no solution
            return None;
        } else if min > max_press {
            // We must increment by `min` to satisfy the joltages we must handle
            // fully, but doing so, we'll overshoot joltages that can also be
            // handled later buttons
            return None;
        } else {
            // The joltage matches and this is the only amount can be do
            min..=min
        }
    } else {
        // All our joltages are also covered by later buttons so we have a range
        // to try
        0..=max_press
    };

    let mut new_missing = vec![0; missing_joltage.len()];
    let mut min_press = i64::MAX;
    for press in press_range {
        if start.elapsed() > TIMEOUT {
            return None;
        }
        // let prefix = format!("{prefix}{press}");
        for idx in 0..new_missing.len() {
            new_missing[idx] =
                missing_joltage[idx] - press * ((machine.buttons[button] & (1 << idx)) != 0) as i64;

            // Because we capped the range, this should never be negative
            assert!(new_missing[idx] >= 0);
        }
        let r = process_button(machine, button + 1, &new_missing, start);
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
    // Sort the button by values, from hight to low values
    // This way will process the counters in order (find the number of press for
    // the last counter, then the next-to-last, ...)
    let mut dup = machine.clone();
    dup.buttons.sort_unstable();
    dup.buttons.reverse();
    let v = machine
        .joltage
        .iter()
        .map(|j| *j as i64)
        .collect::<Vec<_>>();

    let start = std::time::Instant::now();
    let r = process_button(&dup, 0, &v, start).unwrap_or(-1);
    Ok(r)
}

pub fn run(content: &[u8]) -> Result<i64, AdventError> {
    let machines = parse(content.into())?;

    let timeout_count = std::sync::atomic::AtomicUsize::default();

    // let percent = std::sync::atomic::AtomicUsize::default();
    // print!("{:>5.01}%", 0.0,);
    // let _ = std::io::Write::flush(&mut std::io::stdout());
    let result = machines
        .par_iter()
        //.iter()
        .enumerate()
        .map(|(idx, machine)| {
            let s = std::time::Instant::now();
            let r = process_machine(machine);
            (idx, machine, s.elapsed(), r)
        })
        .filter_map(|(_idx, _machine, _time, result)| {
            // let percent = percent.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            // print!(
            //     "\r{_idx}/{} - {_time:?} - {_machine:?}\n    solution: {result:?})\n{:>5.01}%",
            //     machines.len(),
            //     ((percent + 1) as f32) * 100.0 / (machines.len() as f32),
            // );
            // let _ = std::io::Write::flush(&mut std::io::stdout());
            if matches!(result, Ok(-1)) {
                timeout_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                println!("Too slow: {_machine:?}");
                None
            } else {
                Some(result)
            }
        })
        .sum::<Result<i64, _>>();
    // println!();

    println!(
        "{} timed out ({:.2}%)",
        timeout_count.load_value(),
        timeout_count.load_value() as f32 * 100.0 / machines.len() as f32
    );

    result
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
