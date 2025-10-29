use day24::*;
use z3::{ast::Int, Solver};

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

fn process(input: &'_ str) -> Result<i64, AocError<'_>> {
    // Using Z3 solver
    // Code from: https://github.com/arthomnix/aoc23/blob/master/src/days/day24.rs
    // To run:
    //    PATH="$PATH:$(cygpath -a day24/z3-4.12.5-x64-win/bin/)" target/debug/part2.exe
    //    PATH="$PATH:$(cygpath -a day24/z3-4.12.5-x64-win/bin/)" cargo test -p day24 --bin part2
    let hail = parse(input)?;

    let solver = Solver::new();

    let px = Int::new_const("px");
    let py = Int::new_const("py");
    let pz = Int::new_const("pz");
    let vx = Int::new_const("vx");
    let vy = Int::new_const("vy");
    let vz = Int::new_const("vz");

    for stone in hail {
        let pxn = Int::from_i64(stone.position.x as i64);
        let pyn = Int::from_i64(stone.position.y as i64);
        let pzn = Int::from_i64(stone.position.z as i64);
        let vxn = Int::from_i64(stone.velocity.x as i64);
        let vyn = Int::from_i64(stone.velocity.y as i64);
        let vzn = Int::from_i64(stone.velocity.z as i64);
        let tn = Int::fresh_const("t");

        solver.assert((&pxn + &vxn * &tn).eq(&(&px + &vx * &tn)));
        solver.assert((&pyn + &vyn * &tn).eq(&(&py + &vy * &tn)));
        solver.assert((&pzn + &vzn * &tn).eq(&(&pz + &vz * &tn)));
    }

    solver.check();
    let model = solver.get_model().unwrap();
    let x = model.get_const_interp(&px).unwrap().as_i64().unwrap();
    let y = model.get_const_interp(&py).unwrap().as_i64().unwrap();
    let z = model.get_const_interp(&pz).unwrap().as_i64().unwrap();

    Ok(x + y + z)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() -> miette::Result<()> {
        let input = "\
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
";
        assert_eq!(process(input)?, 47);

        Ok(())
    }
}
