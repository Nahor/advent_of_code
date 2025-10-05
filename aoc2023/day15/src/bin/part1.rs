use day15::*;

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
    let vec = parse(input)?;
    let output = vec
        .into_iter()
        .map(|step| aoc_hash(format!("{}{}", step.label, step.op).as_str()))
        .sum();

    Ok(output)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() -> miette::Result<()> {
        let input = "\
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(process(input).unwrap(), 1320);

        Ok(())
    }

    #[test]
    fn example() -> miette::Result<()> {
        let input = "HASH";
        assert_eq!(aoc_hash(input), 52);

        Ok(())
    }
}
