use miette::Result;

use crate::parse::parse;

pub fn run(content: &[u8]) -> Result<u64> {
    let document = parse(content)?;

    let result: u64 = document
        .updates
        .iter()
        .filter(|update| !update.need_remap(&document.rules))
        .map(|update| update.get_mid())
        .sum();

    Ok(result)
}

pub fn run_sort(content: &[u8]) -> Result<u64> {
    let document = parse(content)?;

    let result: u64 = document
        .updates
        .iter()
        .filter(|update| update.is_sorted(&document.rules))
        .map(|update| update.get_mid())
        .sum();

    Ok(result)
}

#[cfg(test)]
mod test {
    use common::read_input_u8;

    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 143);
    }

    #[test]
    fn sample_sorted() {
        assert_eq!(
            run_sort(&INPUT_SAMPLE[1..]).unwrap(),
            run(&INPUT_SAMPLE[1..]).unwrap()
        );

        let input = read_input_u8!(None).unwrap();
        assert_eq!(run_sort(&input).unwrap(), run(&input).unwrap());
    }
}
