use day02::{
    aocerror::AocError,
    parse::{Set, parse},
};

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

fn process(input: &str) -> Result<u32, AocError> {
    let games = parse(input)?;
    let sum = games
        .into_iter()
        .map(|game| {
            let min = game.rounds.iter().fold(Set::default(), |acc, round| Set {
                red: acc.red.max(round.red),
                green: acc.green.max(round.green),
                blue: acc.blue.max(round.blue),
            });
            min.red * min.green * min.blue
        })
        .sum();
    Ok(sum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        assert_eq!(process(input).unwrap(), 2286);
    }
}
