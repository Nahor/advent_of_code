use day02::{
    aocerror::AocError,
    parse::{parse, Set},
};

fn main() -> miette::Result<()> {
    let input = include_str!("input.txt");
    let output = process(
        input,
        &Set {
            red: 12,
            green: 13,
            blue: 14,
        },
    )?;
    dbg!(output);
    Ok(())
}

fn process(input: &str, bag: &Set) -> Result<u32, AocError> {
    let games = parse(input)?;
    let sum = games
        .into_iter()
        .filter(|game| game.rounds.iter().all(|round| round.valid(bag)))
        .map(|game| game.id)
        .sum();
    Ok(sum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn no_header() {
        let input = "Game 1: foo\nbar";
        process(
            input,
            &Set {
                red: 0,
                green: 0,
                blue: 0,
            },
        )
        .unwrap();
    }

    #[test]
    fn test1() {
        let input = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        assert_eq!(
            process(
                input,
                &Set {
                    red: 12,
                    green: 13,
                    blue: 14
                }
            )
            .unwrap(),
            8
        );
    }
}
