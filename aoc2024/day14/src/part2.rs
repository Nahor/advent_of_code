use miette::Result;

use crate::parse::parse;

pub fn run(content: &[u8], tiles: (isize, isize), _seconds: isize) -> Result<u64> {
    let mut robots = parse(content)?;

    // Because the grid width is prime and the horizontal velocity of a robot is
    // less than that, the robot must occupy each column once before it cycles.
    //
    // The same is true vertically.
    //
    // And because both the width and height of the grid are both primes, and
    // different primes, a robot will need to occupy all the cells of the grid
    // once before cycling.
    //
    // And this is true for all the robots.
    //
    // So the repeat cycle has to be width*height
    //
    // Formulas (maybe):
    //   x cycle = width / gcd(width, vel.x)
    //   y cycle = height / gcd(height, vel.y)
    //   robot cycle = lcm(x cycle, y cycle)
    //   grid cycle = lcm(all robot cycles)
    //
    // Code:
    //   let cycle = robots
    //       .iter()
    //       .map(|r| {
    //           let x_cycle = tiles.0 / tiles.0.gcd(&r.vel.0);
    //           let y_cycle = tiles.1 / tiles.1.gcd(&r.vel.1);
    //           x_cycle.lcm(&y_cycle)
    //       })
    //       .reduce(|acc, l| acc.lcm(&l));
    //   println!("cycle: {cycle:?}");
    let repeat_cycle = tiles.0 * tiles.1;
    assert!(robots.iter().all(|r| {
        (r.pos.0 + repeat_cycle * r.vel.0).rem_euclid(tiles.0) == r.pos.0
            && (r.pos.1 + repeat_cycle * r.vel.1).rem_euclid(tiles.1) == r.pos.1
    }));

    for i in 1..repeat_cycle {
        robots.iter_mut().for_each(|robot| {
            robot.pos.0 = (robot.pos.0 + robot.vel.0).rem_euclid(tiles.0);
            robot.pos.1 = (robot.pos.1 + robot.vel.1).rem_euclid(tiles.1);
        });

        // Hint from others: robots must not overlap
        let mut occupation_map = vec![false; (tiles.0 * tiles.1) as usize];
        if robots.iter().any(|robot| {
            let idx = (robot.pos.0 + robot.pos.1 * tiles.0) as usize;
            let occupied = occupation_map[idx];
            occupation_map[idx] = true;
            occupied
        }) {
            continue;
        }

        // No overlap => possible solution
        println!("Candidate: {i}");
        for y in 0..tiles.1 {
            print!("    ");
            for x in 0..tiles.0 {
                if robots
                    .iter()
                    .any(|robot| robot.pos.0 == x && robot.pos.1 == y)
                {
                    print!("*");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    let result: u64 = 0;

    Ok(result)
}
