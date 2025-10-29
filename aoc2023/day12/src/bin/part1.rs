use day12::{aocerror::AocError, parse};
//use owo_colors::{OwoColorize, Style};

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

fn process(input: &str) -> Result<usize, AocError> {
    let output = parse(input, 1, true)?;

    Ok(output)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_full() -> miette::Result<()> {
        let input = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
        assert_eq!(process(input).unwrap(), 21);
        Ok(())
    }

    #[test]
    fn test_individual() -> miette::Result<()> {
        assert_eq!(process("???.### 1,1,3").unwrap(), 1);
        assert_eq!(process(".??..??...?##. 1,1,3").unwrap(), 4);
        assert_eq!(process("?#?#?#?#?#?#?#? 1,3,1,6").unwrap(), 1);
        assert_eq!(process("????.#...#... 4,1,1").unwrap(), 1);
        assert_eq!(process("????.######..#####. 1,6,5").unwrap(), 4);
        assert_eq!(process("?###???????? 3,2,1").unwrap(), 10);

        Ok(())
    }

    #[test]
    fn test_debug() -> miette::Result<()> {
        assert_eq!(process("??????? 2,1").unwrap(), 10);
        assert_eq!(process("?###???????? 3,2,1").unwrap(), 10);

        Ok(())
    }

    #[test]
    fn simple() -> miette::Result<()> {
        assert_eq!(process("# 1").unwrap(), 1);
        assert_eq!(process("? 1").unwrap(), 1);

        assert_eq!(process("?. 1").unwrap(), 1);
        assert_eq!(process("?# 1").unwrap(), 1);
        assert_eq!(process("?? 1").unwrap(), 2);
        assert_eq!(process("?# 2").unwrap(), 1);

        Ok(())
    }

    #[test]
    fn chunks2() -> miette::Result<()> {
        assert_eq!(process("??#.#???#? 2,1,1").unwrap(), 1);

        Ok(())
    }
}
