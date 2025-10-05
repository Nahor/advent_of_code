use std::collections::{HashMap, HashSet};

use common::error::AdventError;
use winnow::{
    ascii::{dec_uint, line_ending, newline},
    combinator::{repeat, separated, separated_pair, terminated, trace},
    prelude::*,
};

use crate::document::{Document, Update};

pub fn parse(content: &[u8]) -> Result<Document, AdventError> {
    Ok(trace(
        "parser",
        separated_pair(parse_rules, newline, parse_pages)
            .map(|(rules, updates)| Document { rules, updates }),
    )
    .parse(content)?)
}

fn parse_rules(input: &mut &[u8]) -> PResult<HashMap<u64, HashSet<u64>>> {
    trace(
        "parse_rules",
        repeat(
            1..,
            terminated(separated_pair(dec_uint, '|', dec_uint), line_ending),
        )
        .fold(
            HashMap::<u64, HashSet<u64>>::new,
            |mut updates, (page, other)| {
                updates.entry(page).or_default().insert(other);
                updates
            },
        ),
    )
    .parse_next(input)
}

fn parse_pages(input: &mut &[u8]) -> PResult<Vec<Update>> {
    trace(
        "parse_pages",
        repeat(1.., terminated(parse_single_page, line_ending)),
    )
    .parse_next(input)
}

fn parse_single_page(input: &mut &[u8]) -> PResult<Update> {
    trace(
        "parse_single_page",
        separated(1.., dec_uint::<_, u64, _>, ',').map(Update),
    )
    .parse_next(input)
}
