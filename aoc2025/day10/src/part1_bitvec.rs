use common::error::AdventError;
use itertools::Itertools;
use miette::Result;

use crate::{
    bitvec::{Machine, MachineStorage},
    parse::bitvec::parse,
};

fn process_machine(machine: &Machine) -> Result<usize, AdventError> {
    let r = (1..=machine.buttons.len())
        .find(|&button_count| {
            machine
                .buttons
                .iter()
                .combinations(button_count)
                .any(|combination| {
                    combination
                        .iter()
                        .fold(MachineStorage::default(), |acc, button| acc ^ **button)
                        == machine.lights
                })
        })
        .ok_or("No combination found")?;
    Ok(r)
}

pub fn process(machines: &[Machine]) -> Result<usize, AdventError> {
    machines
        .iter()
        .map(process_machine)
        .sum::<Result<usize, _>>()
}

pub fn run(content: &[u8]) -> Result<usize, AdventError> {
    let machines = parse(content.into())?;

    process(&machines)
}

#[cfg(test)]
mod test {
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

        assert_eq!(run(input).unwrap(), 7);
    }

    #[test]
    fn compare_base() {
        assert_eq!(
            run(&INPUT_SAMPLE[1..]).unwrap(),
            crate::part1::run(&INPUT_SAMPLE[1..]).unwrap()
        );

        let input = common::read_input_u8!(None).unwrap();
        assert_eq!(run(&input).unwrap(), crate::part1::run(&input).unwrap());
    }
}
