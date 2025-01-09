use miette::Result;

use crate::{machine::Machine, parse};

pub fn run(content: &[u8]) -> Result<u64> {
    let machines = parse::parse(content)?;
    let result: usize = machines.into_iter().map(process_machine).sum();

    Ok(result as u64)
}

fn process_machine(machine: Machine) -> usize {
    // let mut claw_position = Position::default();
    // let mut tokens = 0;

    // let mut heads = BinaryHeap::new();
    // heads.push(Head::default());

    // while let Some(head) = heads.pop() {
    //     let head_a = Head{pos: head+machine.btn_a.claw_move, tokens: }
    // }

    // tokens

    // If the button moves are not colinear, we have the following equations
    //     n * A.x + m * B.x = M.x
    //     n * A.y + m * B.y = M.y
    // where n,m are the number of time button A, button B are pressed
    //
    // so (https://en.wikipedia.org/wiki/Cramer's_rule):
    //    n = (B.x * M.y - M.x * B.y) / (B.x * A.y - A.x * B.y)
    //    m = (A.x * M.y - M.x * A.y) / (A.x * B.y - B.x * A.y)
    assert_ne!(
        machine.btn_a.claw_move.x as f64 / machine.btn_a.claw_move.y as f64,
        machine.btn_b.claw_move.x as f64 / machine.btn_b.claw_move.y as f64,
    );

    let ax = machine.btn_a.claw_move.x as f64;
    let ay = machine.btn_a.claw_move.y as f64;
    let bx = machine.btn_b.claw_move.x as f64;
    let by = machine.btn_b.claw_move.y as f64;
    let mx = machine.prize.x as f64 + 10000000000000.0;
    let my = machine.prize.y as f64 + 10000000000000.0;

    let n = (bx * my - mx * by) / (bx * ay - ax * by);
    let m = mx / bx - n * ax / bx;

    let token_a = n.round();
    let token_b = m.round();

    const EPSILON: f64 = 0.001;
    if (token_a - n).abs() > EPSILON || (token_b - m).abs() > EPSILON {
        // we can't get a whole number of steps => the prize cannot be reach
        return 0;
    }

    token_a as usize * machine.btn_a.tokens + token_b as usize * machine.btn_b.tokens
}

#[cfg(test)]
mod test {
    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        // Not an official value, but based on the assumption the algorithm is
        // valid since we passed the puzzle
        assert_eq!(run(input).unwrap(), 875318608908);
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
