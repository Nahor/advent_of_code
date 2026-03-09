use miette::Result;
use num::BigUint;

use crate::parse::parse;

pub fn run(content: &[u8]) -> Result<u64> {
    let (mut laser, mask, splitters) = parse(content)?;

    let len = mask.bits() as usize;

    let mut laser_count = vec![0_u64; len];
    laser_count[laser.trailing_zeros().unwrap() as usize] = 1;

    splitters.iter().for_each(|splitter| {
        let split_laser = &laser & splitter;

        let through_laser = &laser & (&split_laser ^ &mask);

        let left: BigUint = &split_laser << 1;
        let right: BigUint = &split_laser >> 1;

        laser = &left | &through_laser | &right;

        let mut new_count = vec![0; len];
        (0..len).for_each(|bit| {
            new_count[bit] = (if bit == 0 { 0 } else { laser_count[bit - 1] })
                * (left.bit(bit as u64) as u64)
                + laser_count[bit] * (through_laser.bit(bit as u64) as u64)
                + (if bit + 1 >= len {
                    0
                } else {
                    laser_count[bit + 1]
                }) * (right.bit(bit as u64) as u64)
        });
        laser_count = new_count;
    });

    let result = laser_count.iter().sum::<u64>();

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

        assert_eq!(run(input).unwrap(), 40);
    }
}
