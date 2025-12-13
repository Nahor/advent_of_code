use std::thread::available_parallelism;

use common::error::AdventError;
use miette::Result;
use rayon::prelude::*;
use z3::{Optimize, SatResult, ast::Int};

use crate::{int::Machine, parse::int::parse};

fn process_machine(machine: &Machine) -> Result<u64, AdventError> {
    let solver = Optimize::new();

    // Create the variables
    let vars = machine
        .buttons
        .iter()
        .enumerate()
        .map(|(idx, _)| Int::new_const(format!("btn{idx}")))
        .collect::<Vec<_>>();

    // Add constraints to the buttons (can't be <0)
    // Adding a constraint on the max value only makes thing slower (probably
    // because it's more assertions to check while being unnecessary since we
    // are minimizing the solution)
    vars.iter().for_each(|var| {
        solver.assert(&var.ge(0));
    });

    // Create the equations
    machine
        .joltage
        .iter()
        .enumerate()
        .for_each(|(jolt_idx, &jolts)| {
            let buttons = machine
                .buttons
                .iter()
                .enumerate()
                .filter_map(|(btn_idx, btn)| ((btn & 1 << jolt_idx) != 0).then_some(&vars[btn_idx]))
                .collect::<Vec<_>>();
            let sum = buttons.into_iter().sum::<Int>();
            let eq = sum.eq(jolts as u64);
            solver.assert(&eq);
        });

    // Equation to minimize
    let total_presses = vars.iter().sum::<Int>();
    solver.minimize(&total_presses);

    // Find the solution
    if solver.check(&[]) != SatResult::Sat {
        Err("unsatisfiable machine")?
    }
    let model = solver.get_model().unwrap();
    let result = model.eval(&total_presses, true).unwrap().as_u64().unwrap();
    Ok(result)
}

pub fn run(content: &[u8]) -> Result<u64, AdventError> {
    let machines = parse(content.into())?;

    let _ = rayon::ThreadPoolBuilder::new()
        .num_threads(available_parallelism().unwrap().get() / 2)
        .build_global();

    machines
        .par_iter()
        .enumerate()
        .map(|(idx, machine)| (idx, machine, process_machine(machine)))
        .map(|(_idx, _machine, result)| result)
        .sum::<Result<u64, _>>()
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
    fn single(#[case] input: &[u8], #[case] expected: u64) {
        assert_eq!(run(input).unwrap(), expected);
    }

    #[test]
    fn compare_base() {
        assert_eq!(
            run(&INPUT_SAMPLE[1..]).unwrap(),
            crate::part2_good_lp::run(&INPUT_SAMPLE[1..]).unwrap()
        );

        let input = common::read_input_u8!(None).unwrap();
        assert_eq!(
            run(&input).unwrap(),
            crate::part2_good_lp::run(&input).unwrap()
        );
    }
}
