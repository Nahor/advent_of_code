use miette::Result;
// use num::BigUint;

use crate::parse::parse;

// fn print_laser(laser: &BigUint, mask: &BigUint) {
//     let len = mask.bits();
//     let mut str = String::with_capacity(len as usize);

//     (0..len).for_each(|bit| {
//         if laser.bit(bit) {
//             str.push('|');
//         } else {
//             str.push('.');
//         }
//     });

//     println!("{str}");
// }

pub fn run(content: &[u8]) -> Result<u64> {
    let (mut laser, mask, splitters) = parse(content)?;
    // print_laser(&laser, &mask);

    let result = splitters
        .iter()
        .map(|splitter| {
            let split_laser = &laser & splitter;

            let through_laser = &laser & (&split_laser ^ &mask);

            let left = &split_laser << 1;
            let right = &split_laser >> 1;

            laser = left | through_laser | right;
            // print_laser(&laser, &mask);

            split_laser.count_ones()
        })
        .sum::<u64>();

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 21);
    }
}
