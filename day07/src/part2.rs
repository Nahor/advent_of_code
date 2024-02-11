use std::{collections::BTreeMap, fmt::Display};

use crate::aocerror::{AocError, AocSourceChunk};

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub enum HandType {
    None,
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
impl Display for HandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Default)]
pub struct Data {
    pub hand: Vec<u32>,
    pub bid: usize,
}
impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let hand_type = match self.get_hand_type() {
        //     HandType::None => panic!("Unknown hand"),
        //     HandType::HighCard => "High",
        //     HandType::OnePair => "1Pair",
        //     HandType::TwoPairs => "2Pairs",
        //     HandType::ThreeOfAKind => "Three",
        //     HandType::FullHouse => "House",
        //     HandType::FourOfAKind => "Four",
        //     HandType::FiveOfAKind => "Five",
        // };
        write!(
            f,
            "{}: {:>3} {}",
            self.get_hand_str(),
            self.bid,
            self.get_hand_type()
        )
    }
}
impl Data {
    pub fn get_hand_str(&self) -> String {
        self.hand
            .iter()
            .map(|&card| match card {
                1 => "J".to_owned(),
                card if card < 10 => format!("{card}").to_owned(),
                10 => "T".to_owned(),
                12 => "Q".to_owned(),
                13 => "K".to_owned(),
                14 => "A".to_owned(),
                _ => panic!("Unexpected card value"),
            })
            .collect()
    }
    pub fn get_hand_type(&self) -> HandType {
        let mut card_count: BTreeMap<u32, u32> = BTreeMap::new();
        self.hand.iter().for_each(|card| {
            card_count
                .entry(*card)
                .and_modify(|count| (*count) = (*count) + 1)
                .or_insert(1);
        });
        // println!("count: {card_count:?}");
        let (hand_type, jokers) =
            card_count
                .iter()
                .fold((HandType::None, 0), |(acc, jokers), (card, count)| {
                    // println!("state: {acc}, {jokers} before {card}-{count}");
                    if *card == 1 {
                        (acc, *count)
                    } else {
                        (
                            match count {
                                1 if acc == HandType::None => HandType::HighCard,
                                1 => acc,
                                2 if acc == HandType::None => HandType::OnePair,
                                2 if acc == HandType::HighCard => HandType::OnePair,
                                2 if acc == HandType::OnePair => HandType::TwoPairs,
                                2 if acc == HandType::ThreeOfAKind => HandType::FullHouse,
                                2 => acc,
                                3 if acc == HandType::None => HandType::ThreeOfAKind,
                                3 if acc == HandType::HighCard => HandType::ThreeOfAKind,
                                3 if acc <= HandType::OnePair => HandType::FullHouse,
                                4 => HandType::FourOfAKind,
                                5 => HandType::FiveOfAKind,
                                _ => panic!("Unexpected card count"),
                            },
                            jokers,
                        )
                    }
                });
        // println!("{}, {}, {}", self.get_hand_str(), hand_type, jokers);
        let updated_type = match (jokers, hand_type) {
            (0, _) => hand_type,
            (1, HandType::HighCard) => HandType::OnePair,
            (1, HandType::OnePair) => HandType::ThreeOfAKind,
            (1, HandType::TwoPairs) => HandType::FullHouse,
            (1, HandType::ThreeOfAKind) => HandType::FourOfAKind,
            (1, HandType::FourOfAKind) => HandType::FiveOfAKind,
            (1, _) => panic!("Can't have a joker in this configuration"),
            (2, HandType::HighCard) => HandType::ThreeOfAKind,
            (2, HandType::OnePair) => HandType::FourOfAKind,
            (2, HandType::ThreeOfAKind) => HandType::FiveOfAKind,
            (2, _) => panic!("Can't have two jokers in this configuration"),
            (3, HandType::HighCard) => HandType::FourOfAKind,
            (3, HandType::OnePair) => HandType::FiveOfAKind,
            (3, _) => panic!("Can't have three jokers in this configuration"),
            (4, HandType::HighCard) => HandType::FiveOfAKind,
            (4, _) => panic!("Can't have four jokers in this configuration"),
            (5, HandType::None) => HandType::FiveOfAKind,
            (5, _) => panic!("Can't have five jokers in this configuration"),
            (_, _) => panic!("Invalid number of jokers"),
        };
        updated_type
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
                    'J' => Some(1),
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
