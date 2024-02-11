use std::collections::VecDeque;

use day20::*;
use miette;

fn main() -> miette::Result<()> {
    let input = include_str!("input.txt");
    let output = process(input)?;
    dbg!(output);
    Ok(())
}

fn process(input: &str) -> Result<u64, AocError> {
    let mut modules = parse(input)?;

    let mut low_count = 0;
    let mut high_count = 0;

    let mut pending_pulses = VecDeque::new();

    for _ in 0..1000 {
        pending_pulses.push_back(("", "broadcaster", Pulse::Low));
        low_count += 1;

        while let Some((from, to, pulse)) = pending_pulses.pop_front() {
            match modules.get_mut(to) {
                None => {} // "output" modules
                Some((module, output_list)) => {
                    let new_pulse = match module {
                        Module::Broadcaster(_) => Some(pulse),
                        Module::FlipFlop(flipflop) => flipflop.signal(pulse),
                        Module::Conjunction(conjunction) => conjunction.signal(from, pulse),
                    };
                    if let Some(pulse) = new_pulse {
                        match pulse {
                            Pulse::High => high_count += output_list.len() as u64,
                            Pulse::Low => low_count += output_list.len() as u64,
                        };
                        output_list
                            .iter()
                            .for_each(|new_to| pending_pulses.push_back((to, new_to, pulse)));
                    }
                }
            }
        }
    }

    println!("Counts: low: {low_count},  high: {high_count}");

    Ok(low_count * high_count)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() -> miette::Result<()> {
        let input = "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";
        assert_eq!(process(input).unwrap(), 32000000);

        Ok(())
    }
    #[test]
    fn example2() -> miette::Result<()> {
        let input = "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
";
        assert_eq!(process(input).unwrap(), 11687500);

        Ok(())
    }
}
