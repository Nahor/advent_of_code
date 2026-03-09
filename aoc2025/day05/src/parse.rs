use std::ops::RangeInclusive;

use common::error::AdventError;
use winnow::{
    ascii::{dec_uint, line_ending, newline},
    combinator::{eof, opt, separated, separated_pair, terminated, trace},
    prelude::*,
    token::rest,
};

pub fn parse(content: &[u8]) -> Result<(Vec<RangeInclusive<u64>>, Vec<u64>), AdventError> {
    Ok(trace(
        "parser",
        terminated(
            separated_pair(parse_ranges, (newline, newline), parse_ids),
            (opt(newline), eof),
        ),
    )
    .parse(content)?)
}

pub fn parse_ranges(input: &mut &[u8]) -> ModalResult<Vec<RangeInclusive<u64>>> {
    trace("parse_ranges", separated(1.., parse_range, line_ending)).parse_next(input)
}

pub fn parse_range(input: &mut &[u8]) -> ModalResult<RangeInclusive<u64>> {
    trace(
        "parse_range",
        separated_pair(dec_uint, b'-', dec_uint).map(|(s, e)| s..=e),
    )
    .parse_next(input)
}

pub fn parse_ids(input: &mut &[u8]) -> ModalResult<Vec<u64>> {
    trace("parse_ids", separated(1.., parse_id, line_ending)).parse_next(input)
}

pub fn parse_id(input: &mut &[u8]) -> ModalResult<u64> {
    trace("parse_id", dec_uint).parse_next(input)
}

pub fn parse_no_id(content: &[u8]) -> Result<Vec<RangeInclusive<u64>>, AdventError> {
    Ok(trace("parser", terminated(parse_ranges, (newline, rest))).parse(content)?)
}

pub mod custom {
    use std::ops::RangeInclusive;

    use common::error::AdventError;

    pub fn parse(content: &[u8]) -> Result<Vec<RangeInclusive<u64>>, AdventError> {
        let mut ranges = vec![];
        let lines = content.split(|&c| c == b'\n');
        for line in lines {
            // Empty line separator between ranges and IDs
            if line.is_empty() {
                break;
            }

            // Start number
            let mut iter = line.iter().peekable();
            if let Some(c) = iter.peek()
                && !c.is_ascii_digit()
            {
                Err("Invalid format (expected digits)")?;
            }

            let mut start = 0_u64;
            while let Some(c) = iter.peek()
                && c.is_ascii_digit()
            {
                start = start * 10 + (*c - b'0') as u64;
                let _ = iter.next();
            }

            // Dash
            if iter.peek().is_none_or(|c| **c != b'-') {
                Err("Invalid format (expected '-')")?;
            }
            let _ = iter.next();

            // End number
            if let Some(c) = iter.peek()
                && !c.is_ascii_digit()
            {
                Err("Invalid format (expected digits)")?;
            }

            let mut end = 0_u64;
            while let Some(c) = iter.peek()
                && c.is_ascii_digit()
            {
                end = end * 10 + (*c - b'0') as u64;
                let _ = iter.next();
            }

            if iter.peek().is_some() {
                Err("Invalid format (expected EOL)")?;
            };

            ranges.push(start..=end);
        }

        // This parser is only for part2, so we don't care about the IDs

        Ok(ranges)
    }
}
