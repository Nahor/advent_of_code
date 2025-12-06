use miette::Result;

use crate::parse::parse;

pub fn run(content: &[u8]) -> Result<u64> {
    let (operands, operator) = parse(content)?;
    let len = operands[0].len();

    let result: u64 = (0..len)
        .map(|i| match operator[i] {
            crate::Ops::Add => operands.iter().fold(0_u64, |acc, operand| acc + operand[i]),
            crate::Ops::Mul => operands.iter().fold(1_u64, |acc, operand| acc * operand[i]),
        })
        .sum();

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 4277556);
    }
}
