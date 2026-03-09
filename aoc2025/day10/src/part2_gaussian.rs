use common::error::AdventError;
use rayon::prelude::*;
use rustc_hash::FxHashMap;

use crate::{
    int::Machine,
    parse::int::parse,
    tools::matrix::{GaussianMatrix, Matrix},
};

fn validate_presses(machine: &Machine, button_presses: &[u32]) -> bool {
    machine
        .joltage
        .iter()
        .copied()
        .enumerate()
        .all(|(jolt_idx, joltage)| {
            joltage
                == machine
                    .buttons
                    .iter()
                    .zip(button_presses.iter().copied())
                    .map(|(btn, presses)| {
                        if (btn & (1 << jolt_idx)) != 0 {
                            presses
                        } else {
                            0
                        }
                    })
                    .sum::<u32>()
        })
}

fn get_all_vars(
    machine: &Machine,
    gaussian: &GaussianMatrix,
    independent_vars_map: &FxHashMap<usize, f64>,
) -> Vec<u32> {
    (0..machine.buttons.len())
        .map(|btn| gaussian.get_var_value(btn, independent_vars_map).round() as u32)
        .collect::<Vec<_>>()
}

fn process_machine(machine: &Machine) -> Result<u64, AdventError> {
    let matrix = machine
        .joltage
        .iter()
        .enumerate()
        .map(|(idx, &joltage)| {
            let mut row = machine
                .buttons
                .iter()
                .map(|btn| ((btn >> idx) & 1) as f64)
                .collect::<Vec<_>>();
            row.push(joltage as f64);
            row
        })
        .collect::<Vec<_>>();
    let matrix = Matrix::from(matrix);

    let gaussian = matrix.gaussian_elimination();

    // Get the independent variables/buttons and remap to vec of
    //   `(<index>, <max_presses>, <joltage_inc>, <current_presses>)`
    // where:
    // - `<index>` is the button number
    // - `<max_presses>` is the maximum number of presses the button can have
    //   (i.e. the smallest counter value affected by this button)
    // - `<current_presses>` is the current amount of presses we are trying.
    let mut independent_btns = gaussian
        .get_independent_vars()
        .into_iter()
        .map(|var| {
            (
                var,
                machine
                    .joltage
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, &v)| ((machine.buttons[var] & (1 << idx)) != 0).then_some(v))
                    .min()
                    .unwrap_or_default(),
                0,
            )
        })
        .collect::<Vec<(usize, u32, u32)>>();

    let mut current_total_presses = u32::MAX;
    let mut independent_vars_map = FxHashMap::<_, _>::default();
    let mut independent_button_total_presses = 0;
    loop {
        independent_btns.iter().for_each(|(var, _, val)| {
            *independent_vars_map.entry(*var).or_insert(0.0) = *val as f64
        });
        let btn_presses = get_all_vars(machine, &gaussian, &independent_vars_map);

        let new_total_presses = btn_presses.iter().sum::<u32>();
        if (new_total_presses < current_total_presses) && validate_presses(machine, &btn_presses) {
            // We have a possible better option. Check if it's valid
            current_total_presses = new_total_presses;
        }

        // Update the independent buttons
        // Basically, it's doing an increment with carry over to the next button.
        // The carry condition is either the button reach its max value, or
        // would overflow the total number of presses.
        if independent_btns.iter_mut().all(|(_, max, val)| {
            // Return true if the button was reset (and thus we need to increment
            // the next one) or false otherwise (and we can stop updating since
            // we have a new combination to try)
            if ((independent_button_total_presses + 1) >= current_total_presses)
                || ((*val + 1) > *max)
            {
                independent_button_total_presses -= *val;
                *val = 0;
                true
            } else {
                independent_button_total_presses += 1;
                *val += 1;
                false
            }
        }) {
            // All independent buttons needed to be reset => we've tried all combinations
            break;
        }
    }

    Ok(current_total_presses as u64)
}

pub fn run(content: &[u8]) -> Result<u64, AdventError> {
    let machines = parse(content.into())?;

    // let _ = rayon::ThreadPoolBuilder::new()
    //     .num_threads(std::thread::available_parallelism().unwrap().get() / 2)
    //     .build_global();

    let result = machines
        .par_iter()
        // .iter()
        .map(process_machine)
        .sum::<Result<u64, _>>()?;

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
    #[case(
        b"[..##.##.##] (5,6,7) (0,1,2,5,6,9) (0,1,2,3,4,6,7,9) (2,3,7,8,9) (0,1,2,3,4,5,6,8,9) (0,2,4,5,6,8,9) (5,8) (0,3,4,9) (2,3,4,5,6,7) (1,2,3,4,5,6,7,9) (2,7) (1,2,5,9) (1,2,3,5,7,8,9) {64,72,112,62,52,106,79,71,64,108}",
        128
    )]
    fn single(#[case] input: &[u8], #[case] expected: u64) {
        assert_eq!(run(input).unwrap(), expected);
    }

    #[test]
    #[cfg_attr(debug_assertions, ignore)] // This is too slow when in debug mode
    fn compare_base() {
        assert_eq!(
            run(&INPUT_SAMPLE[1..]).unwrap(),
            crate::part2_z3_optimize::run(&INPUT_SAMPLE[1..]).unwrap()
        );

        let input = common::read_input_u8!(None).unwrap();
        assert_eq!(
            run(&input).unwrap(),
            crate::part2_z3_optimize::run(&input).unwrap()
        );
    }
}
