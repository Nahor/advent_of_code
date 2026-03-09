use std::collections::HashSet;

use aocerror::{AocError, AocSourceChunk, aoc_error_span};

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
        _src: AocSourceChunk::new(line.to_owned(), lineno),
    })?;
    let (winning, have) = numbers.split_once('|').ok_or(AocError::NoWinningHave {
        _src: AocSourceChunk::new(line.to_owned(), lineno),
        _span: aoc_error_span(line, numbers),
    })?;

    let id = header
        .strip_prefix("Card")
        .ok_or(AocError::NoHeader {
            _src: AocSourceChunk::new(line.to_owned(), lineno),
            _span: aoc_error_span(line, header),
        })?
        .trim();
    let id: usize = id.parse().map_err(|err| AocError::InvalidGameId {
        //id_str: id.to_string(),
        _src: AocSourceChunk::new(line.to_owned(), lineno),
        _span: aoc_error_span(line, id),
        _inner: Some(Box::new(err)),
    })?;

    let winning = winning
        .trim()
        .split(' ')
        .filter(|num| !num.is_empty())
        .map(|num| {
            num.parse::<u32>().map_err(|err| AocError::InvalidNumber {
                _src: AocSourceChunk::new(line.to_owned(), lineno),
                _span: aoc_error_span(line, num),
                _inner: Some(Box::new(err)),
            })
        })
        .collect::<Result<HashSet<_>, _>>()?;

    let have = have
        .trim()
        .split(' ')
        .filter(|num| !num.is_empty())
        .map(|num| {
            num.parse::<u32>().map_err(|err| AocError::InvalidNumber {
                _src: AocSourceChunk::new(line.to_owned(), lineno),
                _span: aoc_error_span(line, num),
                _inner: Some(Box::new(err)),
            })
        })
        .collect::<Result<HashSet<_>, _>>()?;

    Ok(Card { id, winning, have })
}
