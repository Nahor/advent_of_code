pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter(|&s| !s.is_empty())
        .map(|line| {
            let first = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .next()
                .unwrap_or_else(|| panic!("No digit in line '{line}'"));
            let second = line
                .chars()
                .rev()
                .filter_map(|c| c.to_digit(10))
                .next()
                .unwrap_or_else(|| panic!("No digit in line '{line}'"));
            first * 10 + second
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let output = part1(input);
        assert_eq!(output, 142);
    }
}
