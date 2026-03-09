use std::thread::available_parallelism;

use common::error::AdventError;
use good_lp::{
    Expression, ProblemVariables, Solution, SolverModel, constraint, default_solver, variable,
};
use miette::Result;
use rayon::prelude::*;

use crate::{int::Machine, parse::int::parse};

fn process_machine(machine: &Machine) -> u64 {
    // Create a variable for each button with constraints (can't be <0 or >min joltage)
    let mut problem = ProblemVariables::new();
    let vars = machine
        .buttons
        .iter()
        .map(|btn| {
            let max_presses = machine
                .joltage
                .iter()
                .copied()
                .enumerate()
                .filter_map(|(jolt_idx, jolts)| ((btn & 1 << jolt_idx) != 0).then_some(jolts))
                .min()
                .unwrap() as u64;

            problem.add(variable().integer().min(0).max(max_presses as f64))
        })
        .collect::<Vec<_>>();

    // Create model
    let objective: Expression = vars.iter().sum();
    let mut model = problem.minimise(objective).using(default_solver);

    // Add constraints
    machine
        .joltage
        .iter()
        .enumerate()
        .for_each(|(jolt_idx, &jolts)| {
            let sum: Expression = machine
                .buttons
                .iter()
                .enumerate()
                .filter_map(|(btn_idx, btn)| ((btn & 1 << jolt_idx) != 0).then_some(vars[btn_idx]))
                .sum();
            model.add_constraint(constraint!(sum == jolts as f64));
        });

    // // At most, each button increases one joltage by one unit. So the total
    // // number of button presses cannot exceed the total number of joltages.
    // let total_vars = vars.iter().fold(Int::from_u64(0), |acc, v| acc + v);
    // let total_jolts = machine.joltage.iter().sum::<u32>() as u64;
    // solver.assert(total_vars.le(total_jolts));

    let solution = model.solve().unwrap();

    vars.iter()
        .map(|var| solution.value(*var).round() as u64)
        .sum::<u64>()
}

pub fn run(content: &[u8]) -> Result<u64, AdventError> {
    let machines = parse(content.into())?;

    // Z3 doesn't seem to play well with hyper-threading
    // Assumes all cores are equals and supports two threads
    // (note, in some cases, e.g. when running tests, the code is run multiple
    // times in the same app instances, so the pool might have already been
    // initialized, so don't check for errors)
    let _ = rayon::ThreadPoolBuilder::new()
        .num_threads(available_parallelism().unwrap().get() / 2)
        .build_global();

    // LOG: let percent = AtomicUsize::default();
    // LOG: print!("{:>5.01}%", 0.0,);
    // LOG: let _ = std::io::stdout().flush();
    let result = machines
        .par_iter()
        // .iter()
        .enumerate()
        .map(|(idx, machine)| (idx, machine, process_machine(machine)))
        .map(|(_idx, _machine, result)| {
            // LOG: let percent = percent.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            // LOG: print!(
            // LOG:     "\r{_idx}/{} - {_machine:?}\n    solution: {result})\n{:>5.01}%",
            // LOG:     machines.len(),
            // LOG:     ((percent + 1) as f32) * 100.0 / (machines.len() as f32),
            // LOG: );
            // LOG: let _ = std::io::stdout().flush();
            result
        })
        .sum::<u64>();
    // LOG: println!();
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
    fn single(#[case] input: &[u8], #[case] expected: u64) {
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
