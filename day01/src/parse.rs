use std::collections::HashMap;

use common::error::AdventError;
use winnow::{
    ascii::{dec_uint, line_ending, multispace1},
    combinator::{opt, repeat, separated, separated_pair, terminated, trace},
    prelude::*,
    stream::Accumulate,
};

pub fn parse(content: &str) -> Result<Vec<(u64, u64)>, AdventError> {
    Ok(trace(
        "parser",
        terminated(separated(1.., parse_line, line_ending), opt(line_ending)),
    )
    .parse(content)?)
}

fn parse_line(input: &mut &str) -> PResult<(u64, u64)> {
    trace(
        "parse_line",
        separated_pair(dec_uint, multispace1, dec_uint),
    )
    .parse_next(input)
}

#[expect(clippy::type_complexity)]
pub fn parse_map(content: &str) -> Result<(HashMap<u64, u64>, HashMap<u64, u64>), AdventError> {
    Ok(trace(
        "parser",
        repeat(1.., terminated(parse_line, line_ending)).fold(
            || (HashMap::new(), HashMap::new()),
            |(mut h1, mut h2), (v1, v2)| {
                h1.entry(v1).and_modify(|v| *v += 1).or_insert(1);
                h2.entry(v2).and_modify(|v| *v += 1).or_insert(1);
                (h1, h2)
            },
        ),
    )
    .parse(content)?)
}

pub fn parse_vecmap(content: &str) -> Result<(Vec<u64>, HashMap<u64, u64>), AdventError> {
    Ok(trace(
        "parser",
        repeat(1.., terminated(parse_line, line_ending)).fold(
            || (Vec::new(), HashMap::new()),
            |(mut h1, mut h2), (v1, v2)| {
                h1.push(v1);
                h2.entry(v2).and_modify(|v| *v += 1).or_insert(1);
                (h1, h2)
            },
        ),
    )
    .parse(content)?)
}

pub fn parse_vec(content: &str) -> Result<(Vec<u64>, Vec<u64>), AdventError> {
    Ok(trace(
        "parser",
        repeat(1.., terminated(parse_line, line_ending)).fold(
            || (Vec::new(), Vec::new()),
            |(mut h1, mut h2), (v1, v2)| {
                h1.push(v1);
                h2.push(v2);
                (h1, h2)
            },
        ),
    )
    .parse(content)?)
}

// Same as `parse_vec` but use a custom accumulator type instead of `fold`
pub fn parse_vec2(content: &str) -> Result<(Vec<u64>, Vec<u64>), AdventError> {
    let v: VecTuple =
        trace("parser", repeat(1.., terminated(parse_line, line_ending))).parse(content)?;
    Ok((v.0, v.1))
}

struct VecTuple(Vec<u64>, Vec<u64>);
impl Accumulate<(u64, u64)> for VecTuple {
    fn initial(capacity: Option<usize>) -> Self {
        if let Some(capacity) = capacity {
            Self(Vec::with_capacity(capacity), Vec::with_capacity(capacity))
        } else {
            Self(Vec::new(), Vec::new())
        }
    }

    fn accumulate(&mut self, acc: (u64, u64)) {
        self.0.push(acc.0);
        self.1.push(acc.1);
    }
}
