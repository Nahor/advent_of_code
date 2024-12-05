use miette::Result;
use std::collections::HashMap;

use crate::parse::{parse, parse_map, parse_vec, parse_vec2, parse_vecmap};

pub fn run(content: &str) -> Result<u64> {
    let lines = parse(content)?;
    let (mut first, mut second): (Vec<_>, Vec<_>) = lines.into_iter().unzip();
    first.sort();
    second.sort();

    let first = count(first);
    let second = count(second);

    let result: u64 = first
        .into_iter()
        .map(|(first_v, first_c)| {
            first_c * first_v * second.get(&first_v).cloned().unwrap_or_default()
        })
        .sum();

    Ok(result)
}

fn count(vec: Vec<u64>) -> HashMap<u64, u64> {
    let mut count = HashMap::new();
    for v in vec {
        *count.entry(v).or_default() += 1;
    }
    count
}

pub fn run_map(content: &str) -> Result<u64> {
    let (first, second) = parse_map(content)?;

    let result: u64 = first
        .into_iter()
        .map(|(first_v, first_c)| {
            first_c * first_v * second.get(&first_v).cloned().unwrap_or_default()
        })
        .sum();

    Ok(result)
}

// This is faster than run_map, probably because:
// - the number of hashmap lookups are the same: either to find the entry in the
// first hashmap during parsing, or to find the entry in the second hashmap when
// doing the sum
// - the number of addition is the same: either to add 1 in the first hashmap's
// count, or to sum the distances
// - run_map must make an extra multiplication to count first_c
// - run_map must handle a second hashmap which is likely more costly than
// handling a vector
pub fn run_vecmap(content: &str) -> Result<u64> {
    let (first, second) = parse_vecmap(content)?;

    let result: u64 = first
        .into_iter()
        .map(|v| v * second.get(&v).cloned().unwrap_or_default())
        .sum();

    Ok(result)
}

pub fn run_sorted(content: &str) -> Result<u64> {
    let lines = parse(content)?;
    let (mut first, mut second): (Vec<_>, Vec<_>) = lines.into_iter().unzip();
    first.sort();
    second.sort();

    let mut iter1 = first.iter().peekable();
    let mut iter2 = second.iter().peekable();
    let mut result = 0;
    loop {
        let Some(v1) = iter1.next() else {
            break;
        };

        // Skip v2s that are not in first
        while iter2.next_if(|v| *v < v1).is_some() {}
        // Count the number of v2 that match v1
        let mut c2 = 0;
        while iter2.next_if(|v| *v == v1).is_some() {
            c2 += 1;
        }

        let dist = v1 * c2;
        result += dist; // for v1
        while iter1.next_if(|v| *v == v1).is_some() {
            result += dist; // subsequent v1
        }
    }

    Ok(result)
}

pub fn run_vec_sorted(content: &str) -> Result<u64> {
    let (mut first, mut second) = parse_vec(content)?;
    first.sort();
    second.sort();

    let mut iter1 = first.iter().peekable();
    let mut iter2 = second.iter().peekable();
    let mut result = 0;
    loop {
        let Some(v1) = iter1.next() else {
            break;
        };

        // Skip v2s that are not in first
        while iter2.next_if(|v| *v < v1).is_some() {}
        // Count the number of v2 that match v1
        let mut c2 = 0;
        while iter2.next_if(|v| *v == v1).is_some() {
            c2 += 1;
        }

        let dist = v1 * c2;
        result += dist; // for v1
        while iter1.next_if(|v| *v == v1).is_some() {
            result += dist; // subsequent v1
        }
    }

    Ok(result)
}

pub fn run_vec_sorted2(content: &str) -> Result<u64> {
    let (mut first, mut second) = parse_vec2(content)?;
    first.sort();
    second.sort();

    let mut iter1 = first.iter().peekable();
    let mut iter2 = second.iter().peekable();
    let mut result = 0;
    loop {
        let Some(v1) = iter1.next() else {
            break;
        };

        // Skip v2s that are not in first
        while iter2.next_if(|v| *v < v1).is_some() {}
        // Count the number of v2 that match v1
        let mut c2 = 0;
        while iter2.next_if(|v| *v == v1).is_some() {
            c2 += 1;
        }

        let dist = v1 * c2;
        result += dist; // for v1
        while iter1.next_if(|v| *v == v1).is_some() {
            result += dist; // subsequent v1
        }
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use common::input::read_input_str;

    use super::*;

    const INPUT_SAMPLE: &str = r#"
3   4
4   3
2   5
1   3
3   9
3   3
"#;

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 31);
    }

    #[test]
    fn sample_map() {
        assert_eq!(
            run_map(&INPUT_SAMPLE[1..]).unwrap(),
            run(&INPUT_SAMPLE[1..]).unwrap()
        );

        let input = read_input_str(None).unwrap();
        assert_eq!(run_map(&input).unwrap(), run(&input).unwrap());
    }

    #[test]
    fn sample_vecmap() {
        assert_eq!(
            run_vecmap(&INPUT_SAMPLE[1..]).unwrap(),
            run(&INPUT_SAMPLE[1..]).unwrap()
        );

        let input = read_input_str(None).unwrap();
        assert_eq!(run_vecmap(&input).unwrap(), run(&input).unwrap());
    }

    #[test]
    fn sample_sorted() {
        assert_eq!(
            run_sorted(&INPUT_SAMPLE[1..]).unwrap(),
            run(&INPUT_SAMPLE[1..]).unwrap()
        );

        let input = read_input_str(None).unwrap();
        assert_eq!(run_sorted(&input).unwrap(), run(&input).unwrap());
    }

    #[test]
    fn sample_vec_sorted() {
        assert_eq!(
            run_vec_sorted(&INPUT_SAMPLE[1..]).unwrap(),
            run(&INPUT_SAMPLE[1..]).unwrap()
        );

        let input = read_input_str(None).unwrap();
        assert_eq!(run_vec_sorted(&input).unwrap(), run(&input).unwrap());
    }
}
