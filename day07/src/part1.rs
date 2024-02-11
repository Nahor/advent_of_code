use std::{collections::BTreeMap, fmt::Display};

use crate::aocerror::{AocError, AocSourceChunk};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Default)]
pub struct Data {
    pub hand: Vec<u32>,
    pub bid: usize,
}
impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut cards = String::new();
        self.hand.iter().for_each(|&card| match card {
            card if card < 10 => cards.push_str(&format!("{card}")),
            10 => cards.push_str("T"),
            11 => cards.push_str("J"),
            12 => cards.push_str("Q"),
            13 => cards.push_str("K"),
            14 => cards.push_str("A"),
            _ => panic!("Unexpected card value"),
        });

        let hand_type = match self.get_hand_type() {
            HandType::HighCard => "High",
            HandType::OnePair => "1Pair",
            HandType::TwoPairs => "2Pairs",
            HandType::ThreeOfAKind => "Three",
            HandType::FullHouse => "House",
            HandType::FourOfAKind => "Four",
            HandType::FiveOfAKind => "Five",
        };
        write!(f, "{}: {:>3} {}", cards, self.bid, hand_type)
    }
}
impl Data {
    fn get_hand_type(&self) -> HandType {
        let mut card_count: BTreeMap<u32, u32> = BTreeMap::new();
        self.hand.iter().for_each(|card| {
            card_count
                .entry(*card)
                .and_modify(|count| (*count) = (*count) + 1)
                .or_insert(1);
        });

        card_count
            .iter()
            .map(|(_, count)| count)
            .fold(HandType::HighCard, |acc, count| match count {
                1 => acc,
                2 if acc == HandType::HighCard => HandType::OnePair,
                2 if acc == HandType::OnePair => HandType::TwoPairs,
                2 if acc == HandType::ThreeOfAKind => HandType::FullHouse,
                2 => acc,
                3 if acc == HandType::HighCard => HandType::ThreeOfAKind,
                3 if acc <= HandType::OnePair => HandType::FullHouse,
                4 => HandType::FourOfAKind,
                5 => HandType::FiveOfAKind,
                _ => panic!("Unexpected card count"),
            })
    }
}
impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand && self.bid == other.bid
    }
}
impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.get_hand_type().partial_cmp(&other.get_hand_type()) {
            Some(std::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        for i in 0..5 {
            match self.hand[i].partial_cmp(&other.hand[i]) {
                Some(std::cmp::Ordering::Equal) => {}
                ord => return ord,
            }
        }
        Some(std::cmp::Ordering::Equal)
    }
}
impl Ord for Data {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).expect("Data should be orderable")
    }
}
impl Eq for Data {}

pub fn parse(input: &str) -> Result<Vec<Data>, AocError> {
    input
        .lines()
        .enumerate()
        .map(|(lineno, line)| parse_line(lineno, line))
        .collect()
}

fn parse_line(lineno: usize, line: &str) -> Result<Data, AocError> {
    let (hand, bid) = line.split_once(' ').ok_or_else(|| AocError::InputError {
        src: AocSourceChunk::new(line.to_owned(), lineno),
        bad_bit: (0, line.len()).into(),
        inner: None,
    })?;

    let hand = hand
        .chars()
        .map(|c| {
            c.to_digit(10)
                .or_else(|| match c {
                    'T' => Some(10),
                    'J' => Some(11),
                    'Q' => Some(12),
                    'K' => Some(13),
                    'A' => Some(14),
                    _ => None,
                })
                .ok_or_else(|| AocError::InvalidNumber {
                    src: AocSourceChunk::new(line.to_owned(), lineno),
                    span: (0, hand.len()).into(),
                    inner: None,
                })
        })
        .collect::<Result<Vec<u32>, AocError>>()?;

    let bid = bid
        .parse::<usize>()
        .map_err(|err| AocError::InvalidNumber {
            src: AocSourceChunk::new(line.to_owned(), lineno),
            span: (hand.len() + 1, line.len()).into(),
            inner: Some(Box::new(err)),
        })?;

    Ok(Data { hand, bid })
}
