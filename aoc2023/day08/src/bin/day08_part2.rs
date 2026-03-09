use std::collections::{HashMap, hash_map::Entry};

use day08::{aocerror::AocError, *};
//use owo_colors::{OwoColorize, Style};
//use rayon::prelude::*;

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

fn process(input: &str) -> Result<usize, AocError> {
    let data = parse(input)?;

    // Compute the number of steps needed to go from the start or from one stop
    // to the next stop (i.e. xxA=>yyZ, or xxZ=>yyZ)
    let path_stops = data
        .nodes
        .keys()
        .filter(|name| name.ends_with('A'))
        .map(|starting_name| {
            let mut step_index = data.steps.iter().enumerate().cycle();
            let mut step_count = 0;

            // (<number of step to the next stop>, <first stop of a cycle>)
            let mut stop_list = Vec::<(usize, bool)>::new();
            // K: (node name, step_index)
            // V: stop_list index
            let mut step_map = HashMap::<(&String, usize), usize>::new();

            let mut name: &String = starting_name;
            // println!("Mapping {name}");
            loop {
                step_count += 1;

                let (index, step) = step_index.next().unwrap();

                let node = match data.nodes.get(name) {
                    Some(node) => node,
                    None => panic!("node {name} not found"),
                };
                name = match step {
                    Step::Left => &node.left,
                    Step::Right => &node.right,
                };
                if name.ends_with('Z') {
                    stop_list.push((step_count, false));
                    step_count = 0;
                    match step_map.entry((name, index)) {
                        Entry::Occupied(entry) => {
                            stop_list[*entry.get()].1 = true;
                            break;
                        }
                        Entry::Vacant(entry) => {
                            entry.insert(stop_list.len());
                        }
                    };
                    // println!("\t{:?}", stop_list);
                    // println!("\t\t{:?}", step_map);
                }
            }
            (starting_name, stop_list)
        })
        .collect::<HashMap<_, _>>();
    println!("Results:");
    path_stops.iter().for_each(|(k, v)| {
        println!("    {k}: {v:?}");
    });

    // !!!!!!!!!
    // Looks like all the stops are the same distance from each other, make sure
    // it's true.
    assert!(
        path_stops
            .iter()
            .all(|(_, v)| { v.iter().all(|&(steps, _)| steps == v[0].0) })
    );

    // !!!!!!!!!
    // Since all the stops are the same distance away, we can use
    // the least/lowest-common-multiple to get the minimum number of steps
    let min_steps = path_stops.iter().fold(1, |acc, (_, v)| lcm(acc, v[0].0));
    Ok(min_steps)

    // let mut min_required = 1 as usize;
    // let mut loop_count = 0;
    // let mut last_instant = Instant::now();
    // loop {
    //     min_required = paths
    //         .par_iter_mut()
    //         .map(|path| {
    //             while path.0 < min_required {
    //                 count_step(&data, path);
    //             }
    //             path.0
    //         })
    //         .max()
    //         .unwrap();

    //     loop_count = loop_count + 1;
    //     if Instant::now() - last_instant > Duration::from_secs(2) {
    //         println!(
    //             "{loop_count:>5} (new_min: {min_required}): {:?}",
    //             paths
    //                 .iter()
    //                 .map(|(step_count, name, _)| (step_count, name))
    //                 .collect::<Vec<_>>()
    //         );
    //         last_instant = Instant::now();
    //     }

    //     if paths
    //         .iter()
    //         .all(|(step_count, _, _)| (*step_count) == min_required)
    //     {
    //         break;
    //     }
    // }

    // Ok(paths.first().unwrap().0)
}

// fn count_step<'a>(
//     data: &'a Data,
//     (step_count, node_name, step_iter): &mut (usize, String, Iter<'a, Step>),
// ) {
//     loop {
//         let step = match step_iter.next() {
//             Some(step) => step,
//             None => {
//                 *step_iter = data.steps.iter();
//                 continue;
//             }
//         };
//         (*step_count) = (*step_count) + 1;
//         let node = match data.nodes.get(node_name) {
//             Some(node) => node,
//             None => panic!("node {node_name} not found"),
//         };
//         *node_name = match step {
//             Step::Left => node.left.clone(),
//             Step::Right => node.right.clone(),
//         };
//         if node_name.ends_with('Z') {
//             break;
//         }
//     }
// }

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
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

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // #[should_panic]
    // fn no_header() {
    //     let input = "Game 1: foo\nbar";
    //     process(input).unwrap();
    // }

    #[test]
    fn test1() -> miette::Result<()> {
        let input = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";
        assert_eq!(process(input).unwrap(), 6);

        Ok(())
    }
}
