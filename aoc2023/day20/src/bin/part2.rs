use std::collections::{HashMap, VecDeque};

use day20::*;

fn main() -> miette::Result<()> {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../input/2023/",
        env!("CARGO_PKG_NAME"),
        "/input.txt"
    ));
    let output = process(input)?;
    dbg!(output);
    Ok(())
}

fn process(input: &str) -> Result<u64, AocError> {
    let mut modules = parse(input)?;

    // Following works assuming  all the following is true:
    // - rx is an output node (i.e. there is no module of that name)
    // - there is a single node outputting to rx (let call it `final`)
    // - that final module is a conjunction
    // - all the inputs of that final module come from conjunctions as well
    //   (lets call them `prefinals`)
    //
    // Knowing AoC, the solution must be computationally expensive, which means
    // we must be able to predict when the solution will be true, which means
    // we must have cycles.
    //
    // - for `final` to send low, all its prefinals must be high
    // - experiments show that:
    //   - prefinals sending a high pulse is rare (one every few button presses)
    //   - there is only one high pulse per cycle (always H-H-H-... never HH-HH-HH and similar)
    //   - the prefinal gets a low pulse immediately after a high one (in the same button press)
    // => we should be able to just use the LCM for all the cycle sizes
    assert!(modules.contains_key("rx"));
    let final_modules = modules
        .iter()
        .filter(|(_, (_, output_list))| output_list.contains(&"rx"))
        .map(|(k, (module, _))| {
            assert!(matches!(module, Module::Conjunction(_)));
            *k
        })
        .collect::<Vec<_>>();
    assert_eq!(final_modules.len(), 1);
    // Get the input of that conjunction
    let mut prefinals = modules
        .iter()
        .filter(|(_, (_, output_list))| output_list.contains(&final_modules[0]))
        .map(|(k, (module, _))| {
            assert!(matches!(module, Module::Conjunction(_)));
            (*k, (None, false))
        })
        .collect::<HashMap<_, _>>();
    println!("Matching inputs: {prefinals:?}");

    let mut pending_pulses = VecDeque::new();
    for i in 1_u64.. {
        if prefinals.iter().all(|(_, v)| v.0.is_some()) {
            // We got a cycle for each prefinal
            break;
        }
        pending_pulses.push_back(("", "broadcaster", Pulse::Low));

        while let Some((from, to, pulse)) = pending_pulses.pop_front() {
            if pulse == Pulse::High {
                if let Some((count, was_high)) = prefinals.get_mut(&from) {
                    match count {
                        Some(init) => {
                            assert_ne!(i, *init);
                            assert!(i % *init == 0);
                        }
                        None => *count = Some(i),
                    };
                    *was_high = true;
                    // println!("prefinals@{i}: {prefinals:?}");
                }
            } else if let Some((count, was_high)) = prefinals.get_mut(&from)
                && *was_high
            {
                *was_high = false;
                // println!("Was high @{i} ({})", (i) % count.unwrap());
                assert!((i) % count.unwrap() == 0);
            }
            match modules.get_mut(to) {
                None => {}
                Some((module, output_list)) => {
                    let new_pulse = match module {
                        Module::Broadcaster(_) => Some(pulse),
                        Module::FlipFlop(flipflop) => flipflop.signal(pulse),
                        Module::Conjunction(conjunction) => conjunction.signal(from, pulse),
                    };
                    if let Some(pulse) = new_pulse {
                        output_list
                            .iter()
                            .for_each(|new_to| pending_pulses.push_back((to, new_to, pulse)));
                    }
                }
            }
        }
    }

    // Use the lcm as the more generic solution, but it looks like all
    // the numbers are prime, i.e. we could just do the product directly
    let output = prefinals.values().map(|v| v.0.unwrap()).fold(1, lcm);
    assert_eq!(output, prefinals.values().map(|v| v.0.unwrap()).product());

    Ok(output)
}

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
    let (mut min, mut max) = if first <= second {
        (first, second)
    } else {
        (second, first)
    };

    loop {
        (min, max) = (max % min, min);
        if min == 0 {
            return max;
        }
    }
}
