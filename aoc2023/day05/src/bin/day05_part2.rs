use day05::{aocerror::AocError, part2::parse};
use rayon::prelude::*;

fn main() -> miette::Result<()> {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../input/2023/",
        env!("CARGO_PKG_NAME"),
        "/input.txt"
    ));
    let output = process(input)?;
    dbg!(output);
    Ok(())
}

fn process(input: &str) -> Result<u64, AocError> {
    let data = parse(input)?;

    static CHUNK_SIZE: u64 = 1_000_000;
    let small_seed_ranges = data
        .seeds
        .par_iter()
        .map(|seed_range| {
            seed_range
                .clone()
                .step_by(CHUNK_SIZE as usize)
                .map(|start| {
                    let end = (start + CHUNK_SIZE).min(seed_range.end);
                    start..end
                })
                .collect::<Vec<_>>()
            // range.take
            // while range.peek().is_some() {
            //     let chunk: Vec<_> = range.by_ref().take(5).collect();

            //     println!("{:?}", chunk);
            // }
        })
        .flatten()
        .collect::<Vec<_>>();
    println!(
        "Processing {} chunks, {} seeds",
        small_seed_ranges.len(),
        small_seed_ranges
            .iter()
            .map(|range| range.end - range.start)
            .sum::<u64>()
    );

    let low = small_seed_ranges
        .par_iter()
        .map(|seed_range| {
            let mut low = u64::MAX;
            for seed in seed_range.clone() {
                let mut next = seed;
                // print!("Mapping {next}");
                for map in data.maps.values() {
                    let (range, next_start) = map
                        .get_key_value(&next)
                        .map(|(range, dst)| (range.clone(), *dst))
                        .or_else(|| Some((next..(next + 1), next)))
                        .unwrap();
                    next = (next - range.start) + next_start;
                    // print!(" -> {next}");
                }
                low = low.min(next);
                // println!(" => low: {low}");
            }
            low
        })
        .min()
        .expect("should have a value");

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
        assert_eq!(process(input).unwrap(), 46);

        Ok(())
    }
}
