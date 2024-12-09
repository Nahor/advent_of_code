use miette::Result;

use crate::parse::parse;

pub fn run(content: &[u8]) -> Result<u64> {
    let equations = parse(content)?;
    let result: i64 = equations
        .into_iter()
        .filter_map(|equation| {
            if count_solutions("".into(), equation.result, &equation.terms) > 0 {
                Some(equation.result)
            } else {
                None
            }
        })
        .sum();

    Ok(result as u64)
}

fn count_solutions(ops: String, result: i64, terms: &[i64]) -> u64 {
    if terms.len() == 1 {
        if terms[0] == result {
            //println!("OK: {} {ops}", terms[0]);
            1
        } else {
            //println!("NOK: {} {ops}", terms[0]);
            0
        }
    } else {
        let mut count = 0;
        let (rest, term) = terms.split_at(terms.len() - 1);
        let term = term[0];
        let tmp_result = result - term;
        if tmp_result >= 0 {
            count += count_solutions(format!("+ {term} {ops}"), tmp_result, rest);
        }

        let tmp_result = result / term;
        if (tmp_result * term) == result {
            count += count_solutions(format!("* {term} {ops}"), tmp_result, rest);
        }

        let result_str = format!("{result}").to_string();
        let term_str = format!("{term}").to_string();
        if let Some(trimmed) = result_str.strip_suffix(&term_str) {
            if trimmed.is_empty() {
                // there is no more result to match the remaining terms => no match
            } else {
                let new_result: i64 = trimmed.parse().unwrap();
                count += count_solutions(format!("|| {term} {ops}"), new_result, rest);
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 11387);
    }

    // #[test]
    // fn sample_sorted() {
    //     assert_eq!(
    //         run_sorted(&INPUT_SAMPLE[1..]).unwrap(),
    //         run(&INPUT_SAMPLE[1..]).unwrap()
    //     );

    //     let input = read_input_u8(None).unwrap();
    //     assert_eq!(run_sorted(&input).unwrap(), run(&input).unwrap());
    // }
}
