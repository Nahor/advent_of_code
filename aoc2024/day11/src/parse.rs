use common::error::AdventError;
use winnow::{
    ascii::{dec_uint, line_ending},
    combinator::{opt, separated, terminated, trace},
    prelude::*,
};

pub fn parse(content: &[u8]) -> Result<Vec<u64>, AdventError> {
    Ok(trace(
        "parser",
        terminated(
            separated(1.., dec_uint::<_, u64, _>, b' '),
            opt(line_ending),
        ),
    )
    .parse(content)?)
}
