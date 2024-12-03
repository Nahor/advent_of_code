use regex::Regex;

use crate::{error::AdventError, op::Op};

pub fn parse_part1(content: &str) -> Result<Vec<(u64, u64)>, AdventError> {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    let mut results = Vec::new();
    for (_, [a, b]) in re.captures_iter(content).map(|c| c.extract()) {
        let a = a.parse()?;
        let b = b.parse()?;
        results.push((a, b));
    }

    Ok(results)
}

pub fn parse_part2(content: &str) -> Result<Vec<Op>, AdventError> {
    let re = Regex::new(
        r"(?<mul>mul\((?<a>[0-9]{1,3}),(?<b>[0-9]{1,3})\))|(?<dont>don't\(\))|(?<do>do\(\))",
    )
    .unwrap();

    let mut results = Vec::new();
    for capture in re.captures_iter(content) {
        if capture.name("mul").is_some() {
            let a: u64 = capture["a"].parse()?;
            let b: u64 = capture["b"].parse()?;
            results.push(Op::Mul(a, b));
        } else if capture.name("dont").is_some() {
            results.push(Op::Dont);
        } else if capture.name("do").is_some() {
            results.push(Op::Do);
        } else {
            unreachable!()
        }
    }

    Ok(results)
}
