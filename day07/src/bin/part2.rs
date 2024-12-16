use std::collections::BTreeSet;

use day07::{aocerror::AocError, part2::*};
//use owo_colors::{OwoColorize, Style};

fn main() -> miette::Result<()> {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../advent_of_code_input/2023/",
        env!("CARGO_PKG_NAME"),
        "/input.txt"
    ));
    let output = process(input)?;
    dbg!(output);
    Ok(())
}

fn process(input: &str) -> Result<usize, AocError> {
    let data = parse(input)?;

    let ordered: BTreeSet<_> = data.iter().collect();
    let output = ordered
        .into_iter()
        .enumerate()
        .map(|(rank, data)| (rank + 1) * data.bid)
        .sum();

    Ok(output)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() -> miette::Result<()> {
        let input = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
        assert_eq!(process(input).unwrap(), 5905);

        Ok(())
    }

    #[test]
    fn cmp1() {
        let d1 = Data {
            hand: vec![9, 2, 3, 4, 10],
            bid: 348,
        };
        let d2 = Data {
            hand: vec![5, 2, 3, 8, 4],
            bid: 17,
        };
        let _ = dbg!(d1.partial_cmp(&d2));
    }

    #[test]
    fn jokers() {
        let d1 = Data {
            hand: vec![13, 10, 1, 1, 10],
            bid: 348,
        };
        println!("{} {}", d1.get_hand_str(), d1.get_hand_type());
    }
}
