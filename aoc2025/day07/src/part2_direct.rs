use common::error::AdventError;
use miette::Result;

pub fn run(content: &[u8]) -> Result<u64, AdventError> {
    // Skip "empty" lines (i.e. without splitter nor starting point)
    let mut lines = content
        .split(|b| *b == b'\n')
        .enumerate()
        .filter_map(|(lineno, line)| ((lineno % 2 == 0) && !line.is_empty()).then_some(line));

    let start = lines.next().ok_or("empty data")?;
    let width = start.len();
    let idx = start
        .iter()
        .copied()
        .position(|b| b == b'S')
        .ok_or("no starting point")?;

    let mut laser = vec![0_u64; width];
    laser[idx] = 1;

    laser = lines.try_fold(laser, |laser, line| {
        let mut next_laser = vec![0_u64; width];
        line.iter().copied().enumerate().try_for_each(|(idx, b)| {
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
                _ => Err(format!("Invalid char '{}' (0x{:x})", b as char, b).to_string()),
            }
        })?;
        Ok::<_, AdventError>(next_laser)
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
