use std::thread::available_parallelism;

use common::error::AdventError;
use miette::Result;
use rayon::prelude::*;
use z3::{Solver, ast::Int};

use crate::{int::Machine, parse::int::parse};

fn process_machine(machine: &Machine) -> (u64, usize) {
    let solver = Solver::new();

    // Create the variables
    let vars = machine
        .buttons
        .iter()
        .enumerate()
        .map(|(idx, _)| Int::new_const(format!("btn{idx}")))
        .collect::<Vec<_>>();

    // Add constraints to the buttons (can't be <0 or >min joltage)
    vars.iter().enumerate().for_each(|(idx, var)| {
        let btn = machine.buttons[idx];

        let max_count = machine
            .joltage
            .iter()
            .copied()
            .enumerate()
            .filter_map(|(jolt_idx, jolts)| ((btn & 1 << jolt_idx) != 0).then_some(jolts))
            .min()
            .unwrap() as u64;

        solver.assert(var.ge(0));
        solver.assert(var.le(max_count));
    });

    // Create the equations
    machine
        .joltage
        .iter()
        .enumerate()
        .for_each(|(jolt_idx, &jolts)| {
            let sum =
                machine
                    .buttons
                    .iter()
                    .enumerate()
                    .fold(Int::from_u64(0), |acc, (btn_idx, btn)| {
                        if (btn & 1 << jolt_idx) == 0 {
                            acc
                        } else {
                            acc + &vars[btn_idx]
                        }
                    });
            solver.assert(sum.eq(jolts as u64));
        });

    // At most, each button increases one joltage by one unit. So the total
    // number of button presses cannot exceed the total number of joltages.
    let total_vars = vars.iter().fold(Int::from_u64(0), |acc, v| acc + v);
    let total_jolts = machine.joltage.iter().sum::<u32>() as u64;
    solver.assert(total_vars.le(total_jolts));

    // Find the solution
    let mut count = 0;
    let mut result = u64::MAX;
    // We could add a new constraint at each iteration and restart the solver,
    // to force it to give us a smaller result each time, but that only speeds
    // up some cases (where there are a lot of solutions to begin with) and
    // slows down others (where there are few solutions). Overall, it is slower
    for solution in solver.solutions(&vars, true) {
        // println!("  solution: {solution:?}");
        count += 1;
        result = solution
            .iter()
            .map(Int::as_u64)
            .map(Option::unwrap)
            .sum::<u64>()
            .min(result);
    }

    (result, count)
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
        .map(|(_idx, _machine, (result, _count))| {
            // LOG: let percent = percent.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            // LOG: print!(
            // LOG:     "\r{_idx}/{} - {_machine:?}\n    {_count} solutions (best: {result})\n{:>5.01}%",
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
