use miette::Result;

pub fn run(content: &[u8]) -> Result<u64> {
    let mut content = content.split(|&b| b == b'\n').collect::<Vec<_>>();

    // Remove last line if it's empty
    let _ = content.pop_if(|l| l.is_empty());

    assert!(content.len() >= 3);

    let width = content[0].len();
    assert!(content.iter().all(|line| { line.len() == width }));

    let number_lines = &content[0..content.len() - 1];
    let ops_line = content[content.len() - 1];

    // Parse the numbers, leaving a `None` between sets of numbers
    let mut numbers = vec![None::<u64>; width];
    number_lines.iter().for_each(|line| {
        line.iter().enumerate().for_each(|(idx, b)| {
            if !b.is_ascii_digit() {
                return;
            }

            numbers[idx] = Some(numbers[idx].unwrap_or_default() * 10 + (b - b'0') as u64);
        });
    });

    // Iterate over the operators, zipping them with the sets of numbers
    let result = ops_line
        .iter()
        .filter(|b| **b == b'+' || **b == b'*')
        .zip(numbers.split(|num| num.is_none()))
        .map(|(op, numbers)| match op {
            b'+' => numbers.iter().fold(0_u64, |acc, num| acc + num.unwrap()),
            b'*' => numbers.iter().fold(1_u64, |acc, num| acc * num.unwrap()),
            _ => unreachable!(),
        })
        .sum::<u64>();

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

        assert_eq!(run(input).unwrap(), 3263827);
    }
}
