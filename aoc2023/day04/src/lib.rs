use std::collections::HashSet;

use aocerror::{aoc_error_span, AocError, AocSourceChunk};

pub mod aocerror;

#[derive(Debug)]
pub struct Card {
    pub id: usize,
    pub winning: HashSet<u32>,
    pub have: HashSet<u32>,
}

pub fn parse(input: &str) -> Result<Vec<Card>, AocError> {
    let r = input
        .lines()
        .enumerate()
        .map(|(lineno, line)| parse_line(lineno, line))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(r)
}

pub fn parse_line(lineno: usize, line: &str) -> Result<Card, AocError> {
    let (header, numbers) = line.split_once(':').ok_or(AocError::NoHeaderNumbers {
        src: AocSourceChunk::new(line.to_owned(), lineno),
    })?;
    let (winning, have) = numbers.split_once('|').ok_or(AocError::NoWinningHave {
        src: AocSourceChunk::new(line.to_owned(), lineno),
        span: aoc_error_span(line, numbers),
    })?;

    let id = header
        .strip_prefix("Card")
        .ok_or(AocError::NoHeader {
            src: AocSourceChunk::new(line.to_owned(), lineno),
            span: aoc_error_span(line, header),
        })?
        .trim();
    let id: usize = id.parse().map_err(|err| AocError::InvalidGameId {
        //id_str: id.to_string(),
        src: AocSourceChunk::new(line.to_owned(), lineno),
        span: aoc_error_span(line, id),
        inner: Some(Box::new(err)),
    })?;

    let winning = winning
        .trim()
        .split(' ')
        .filter(|num| !num.is_empty())
        .map(|num| {
            num.parse::<u32>().map_err(|err| AocError::InvalidNumber {
                src: AocSourceChunk::new(line.to_owned(), lineno),
                span: aoc_error_span(line, num),
                inner: Some(Box::new(err)),
            })
        })
        .collect::<Result<HashSet<_>, _>>()?;

    let have = have
        .trim()
        .split(' ')
        .filter(|num| !num.is_empty())
        .map(|num| {
            num.parse::<u32>().map_err(|err| AocError::InvalidNumber {
                src: AocSourceChunk::new(line.to_owned(), lineno),
                span: aoc_error_span(line, num),
                inner: Some(Box::new(err)),
            })
        })
        .collect::<Result<HashSet<_>, _>>()?;

    Ok(Card { id, winning, have })
}
