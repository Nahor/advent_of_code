use day08::{aocerror::AocError, parse, Step};
use miette;
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

fn process(input: &str) -> Result<u32, AocError> {
    let data = parse(input)?;

    let mut step_count = 0;
    let mut step_iter = data.steps.iter();
    let mut node_name = "AAA".to_owned();
    while node_name != "ZZZ" {
        let step = match step_iter.next() {
            Some(step) => step,
            None => {
                step_iter = data.steps.iter();
                continue;
            }
        };
        step_count = step_count + 1;
        let node = match data.nodes.get(&node_name) {
            Some(node) => node,
            None => panic!("node {node_name} not found"),
        };
        node_name = match step {
            Step::Left => node.left.clone(),
            Step::Right => node.right.clone(),
        };
    }

    Ok(step_count)
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
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";
        assert_eq!(process(input).unwrap(), 6);

        Ok(())
    }

    #[test]
    fn test2() -> miette::Result<()> {
        let input = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";
        assert_eq!(process(input).unwrap(), 2);

        Ok(())
    }
}
