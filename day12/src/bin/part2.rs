use day12::{aocerror::AocError, parse};
use miette;
//use owo_colors::{OwoColorize, Style};

fn main() -> miette::Result<()> {
    let input = include_str!("input.txt");
    let output = process(input)?;
    dbg!(output);
    Ok(())
}

fn process(input: &str) -> Result<usize, AocError> {
    let output = parse(input, 5, false)?;

    Ok(output)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() -> miette::Result<()> {
        let input = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
        assert_eq!(process(input).unwrap(), 525152);

        Ok(())
    }

    #[test]
    fn test2() -> miette::Result<()> {
        let input = "\
???.### 1,1,3
";
        assert_eq!(process(input).unwrap(), 1);

        Ok(())
    }

    #[test]
    fn test3() -> miette::Result<()> {
        let input = "\
.??..??...?##. 1,1,3
";
        assert_eq!(process(input).unwrap(), 16384);

        Ok(())
    }
}
