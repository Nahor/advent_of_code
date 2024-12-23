use core::str;
use std::fmt::{Debug, Display};

use common::error::AdventError;
use rustc_hash::FxHashMap;
use winnow::{
    ascii::line_ending,
    combinator::{repeat, separated_pair, terminated, trace},
    prelude::*,
    token::take,
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Machine(pub [u8; 2]);

impl Debug for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //f.debug_tuple("Machine").field(&self.0).finish()
        write!(f, "{}", unsafe { str::from_utf8_unchecked(&self.0) })
    }
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //f.debug_tuple("Machine").field(&self.0).finish()
        write!(f, "{}", unsafe { str::from_utf8_unchecked(&self.0) })
    }
}

pub fn parse(content: &[u8]) -> Result<FxHashMap<Machine, Vec<Machine>>, AdventError> {
    let mut connections = trace(
        "parser",
        repeat(1.., parse_line).fold(
            FxHashMap::default,
            |mut map: FxHashMap<_, Vec<Machine>>, (m1, m2)| {
                map.entry(m1).or_default().push(m2);
                map
            },
        ),
    )
    .parse(content)?;

    connections.values_mut().for_each(|v| v.sort());
    Ok(connections)
}

fn parse_line(input: &mut &[u8]) -> PResult<(Machine, Machine)> {
    trace(
        "parse_line",
        terminated(
            separated_pair(
                take(2_usize).map(|d: &[u8]| Machine([d[0], d[1]])),
                b'-',
                take(2_usize).map(|d: &[u8]| Machine([d[0], d[1]])),
            )
            .map(|(m1, m2)| if m1 < m2 { (m1, m2) } else { (m2, m1) }),
            line_ending,
        ), // technically, this consumes everything until eof, not just the line
    )
    .parse_next(input)
}
