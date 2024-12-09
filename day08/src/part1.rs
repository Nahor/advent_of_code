use std::collections::HashSet;

use itertools::Itertools as _;
use miette::Result;

use crate::parse::parse;

pub fn run(content: &[u8]) -> Result<u64> {
    let (antennas, bounds) = parse(content)?;

    let result: HashSet<_> = antennas
        .values()
        .flat_map(|coord| {
            coord.iter().combinations(2).flat_map(|c| {
                let c1 = c[0];
                let c2 = c[1];
                let diff_x = c2.0 - c1.0;
                let diff_y = c2.1 - c1.1;
                let n1 = (c1.0 - diff_x, c1.1 - diff_y);
                let n2 = (c2.0 + diff_x, c2.1 + diff_y);
                [n1, n2].into_iter()
            })
        })
        .filter(|node| bounds.0.contains(&node.0) && bounds.1.contains(&node.1))
        .collect();

    // for y in bounds.1 {
    //     print!("    ");
    //     for x in bounds.0.clone() {
    //         if let Some(freq) = antennas.iter().find_map(|(freq, coords)| {
    //             coords
    //                 .iter()
    //                 .find_map(|coord| if *coord == (x, y) { Some(*freq) } else { None })
    //         }) {
    //             print!("{}", freq as char)
    //         } else if result.contains(&(x, y)) {
    //             print!("#")
    //         } else {
    //             print!(".")
    //         }
    //     }
    //     println!();
    // }

    Ok(result.len() as u64)
}

#[cfg(test)]
mod test {
    use super::*;

    // cspell:disable
    const INPUT_SAMPLE_1: &[u8] = br#"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"#;

    const INPUT_SAMPLE_2: &[u8] = br#"
..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........
"#;

    const INPUT_SAMPLE_3: &[u8] = br#"
..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
..........
"#;

    const INPUT_SAMPLE_4: &[u8] = br#"
..........
..........
..........
....a.....
........a.
.....a....
..........
......A...
..........
..........
"#;
    // cspell:enable

    #[test]
    fn sample_1() {
        let input = &INPUT_SAMPLE_1[1..]; // remove leading \n
        assert_eq!(run(input).unwrap(), 14);
    }

    #[test]
    fn sample_2() {
        let input = &INPUT_SAMPLE_2[1..]; // remove leading \n
        assert_eq!(run(input).unwrap(), 2);
    }

    #[test]
    fn sample_3() {
        let input = &INPUT_SAMPLE_3[1..]; // remove leading \n
        assert_eq!(run(input).unwrap(), 4);
    }

    #[test]
    fn sample_4() {
        let input = &INPUT_SAMPLE_4[1..]; // remove leading \n
        assert_eq!(run(input).unwrap(), 4);
    }

    // #[test]
    // fn sample_sorted() {
    //     assert_eq!(
    //         run_sorted(&INPUT_SAMPLE[1..]).unwrap(),
    //         run(&INPUT_SAMPLE[1..]).unwrap()
    //     );

    //     let input = read_input_u8(None).unwrap();
    //     assert_eq!(run_sorted(&input).unwrap(), run(&input).unwrap());
    // }
}
