use day05::{aocerror::AocError, part1::parse};
use miette;
//use owo_colors::{OwoColorize, Style};

fn main() -> miette::Result<()> {
    let input = include_str!("input.txt");
    let output = process(input)?;
    dbg!(output);
    Ok(())
}

fn process(input: &str) -> Result<u64, AocError> {
    let data = parse(input)?;

    let mut low = u64::MAX;
    for seed in data.seeds {
        let mut next = seed as u64;
        print!("Mapping {next}");
        for map in data.maps.values() {
            let (range, next_start) = map
                .get_key_value(&next)
                .and_then(|(range, dst)| Some((range.clone(), *dst)))
                .or_else(|| Some((next..(next + 1), next)))
                .unwrap();
            next = (next - range.start) + next_start;
            print!(" -> {next}");
        }
        low = low.min(next);
        println!(" => low: {low}");
    }

    Ok(low)
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // #[should_panic]
    // fn no_header() {
    //     let input = "Game 1: foo\nbar";
    //     process(input).unwrap();
    // }

    #[test]
    fn test1() -> miette::Result<()> {
        let input = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
        assert_eq!(process(input).unwrap(), 35);

        Ok(())
    }
}
