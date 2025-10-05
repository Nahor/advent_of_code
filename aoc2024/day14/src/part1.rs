use miette::Result;

use crate::parse::{parse, Robot};

pub fn run(content: &[u8], tiles: (isize, isize), seconds: isize) -> Result<u64> {
    let robots = parse(content)?;

    let quadrants = robots
        .into_iter()
        .map(|robot| Robot {
            pos: (
                (robot.pos.0 + seconds * robot.vel.0).rem_euclid(tiles.0),
                (robot.pos.1 + seconds * robot.vel.1).rem_euclid(tiles.1),
            ),
            vel: robot.vel,
        })
        .fold((0, 0, 0, 0), |mut quadrants, robot| {
            match (
                robot.pos.0.cmp(&(tiles.0 / 2)),
                robot.pos.1.cmp(&(tiles.1 / 2)),
            ) {
                (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => quadrants.0 += 1,
                (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => quadrants.1 += 1,
                (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => quadrants.2 += 1,
                (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => quadrants.3 += 1,
                _ => {} // equals => ignore
            };
            quadrants
        });

    let result: u64 = quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3;

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input, (11, 7), 100).unwrap(), 12);
    }

    // #[test]
    // fn compare_base() {
    //     assert_eq!(
    //         run(&INPUT_SAMPLE[1..]).unwrap(),
    //         crate::part1::run(&INPUT_SAMPLE[1..]).unwrap()
    //     );

    //     let input = common::read_input_u8!(None).unwrap();
    //     assert_eq!(run(&input).unwrap(), crate::part1::run(&input).unwrap());
    // }
}
