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

    // Proceed right-to-left, so when we see an operator on `operator_line`,
    // we are done with that set of numbers
    //
    // Two accumulators:
    // - a temporary vec of numbers, to store numbers until we get an operator
    // - a final number to store the sum of the operations
    let (_, result) = (0..width).rev().fold(
        (Vec::<u64>::default(), 0_u64),
        |(mut numbers, mut sum), idx| {
            let number =
                number_lines
                    .iter()
                    .enumerate()
                    .fold(None::<u64>, |acc, (lineno, line)| {
                        let digit = line[idx];
                        if digit == b' ' {
                            acc
                        } else if digit.is_ascii_digit() {
                            Some(acc.unwrap_or_default() * 10 + (digit - b'0') as u64)
                        } else {
                            panic!(
                                "expected digit or space, got '{}' (0x{:x}) at line:column {}:{}",
                                digit as char,
                                digit,
                                lineno + 1,
                                idx + 1
                            );
                        }
                    });
            assert!(number.is_some() || numbers.is_empty());

            if let Some(number) = number {
                numbers.push(number)
            }

            match ops_line[idx] {
                b'+' => {
                    sum += numbers.iter().sum::<u64>();
                    numbers.clear();
                }
                b'*' => {
                    sum += numbers.iter().product::<u64>();
                    numbers.clear();
                }
                b' ' => {}
                c => {
                    panic!(
                        "expected operator or space, got '{}' (0x{:x}) at column {}",
                        c as char,
                        c,
                        idx + 1
                    );
                }
            }

            (numbers, sum)
        },
    );
    // .filter_map(|i| {
    //     number_lines.iter().fold(None::<u64>, |acc, line| {
    //         let digit = line[i];
    //         if digit == b' ' {
    //             assert!(acc.is_none());
    //             None
    //         } else if digit.is_ascii_digit() {
    //             Some(acc.unwrap_or_default() * 10 + (digit - b'0') as u64)
    //         } else {
    //             panic!("expected digit or space, got '{digit}' (0x{digit:x})");
    //         }
    //     })
    // })
    // .collect::<Vec<_>>();

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
