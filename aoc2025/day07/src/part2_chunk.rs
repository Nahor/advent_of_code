use std::mem::swap;

use common::error::AdventError;
use miette::Result;

pub fn run(content: &[u8]) -> Result<u64, AdventError> {
    let start = content.split(|b| *b == b'\n').next().ok_or("empty data")?;
    let width = start.len();
    let idx = start
        .iter()
        .copied()
        .position(|b| b == b'S')
        .ok_or("no starting point")?;

    let mut laser = vec![0_u64; width];
    laser[idx] = 1;

    let mut next_laser = vec![0_u64; width];

    content
        .chunks_exact(width + 1)
        .skip(2) // Skip starting point and next empty line
        .step_by(2) // Skip every other line (i.e. those without splitters)
        .try_for_each(|line| {
            next_laser.fill(0);
            line[0..width].iter().enumerate().try_for_each(|(idx, b)| {
                let l = laser[idx];
                match b {
                    b'.' => {
                        // pass through
                        next_laser[idx] += l;
                        Ok(())
                    }
                    b'^' => {
                        // splitter, add to left and right
                        next_laser[idx - 1] += l;
                        next_laser[idx + 1] += l;
                        Ok(())
                    }
                    _ => Err(format!("Invalid char '{}' (0x{:x})", *b as char, b).to_string()),
                }
            })?;
            swap(&mut laser, &mut next_laser);
            Ok::<_, AdventError>(())
        })?;

    Ok(laser.iter().sum())
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

    #[test]
    fn compare_base() {
        assert_eq!(
            run(&INPUT_SAMPLE[1..]).unwrap(),
            crate::part2::run(&INPUT_SAMPLE[1..]).unwrap()
        );

        let input = common::read_input_u8!(None).unwrap();
        assert_eq!(run(&input).unwrap(), crate::part2::run(&input).unwrap());
    }
}
