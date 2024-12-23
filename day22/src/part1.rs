use miette::Result;

use crate::parse::parse;

pub fn run(content: &[u8]) -> Result<u64> {
    let secrets = parse(content)?;

    let result: u64 = secrets
        .iter()
        .map(|&secret| next_secret_n::<2000>(secret) as u64)
        .sum();

    Ok(result)
}

fn next_secret_n<const N: usize>(secret: u32) -> u32 {
    (0..N).fold(secret, |secret, _| next_secret(secret))
}

fn next_secret(mut secret: u32) -> u32 {
    secret ^= secret * 64;
    secret %= 16777216;
    secret ^= secret / 32;
    secret %= 16777216;
    secret ^= secret * 2048;
    secret %= 16777216;
    secret
}

#[cfg(test)]
mod test {
    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"
1
10
100
2024
"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 37327623);
    }

    #[test]
    fn single() {
        let list = (0..10)
            .scan(123, |secret, _| {
                *secret = next_secret(*secret);
                Some(*secret)
            })
            .collect::<Vec<_>>();
        assert_eq!(
            list,
            [
                15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
                5908254
            ]
        );
    }

    #[test]
    fn list() {
        assert_eq!(next_secret_n::<2000>(1), 8685429);
        assert_eq!(next_secret_n::<2000>(10), 4700978);
        assert_eq!(next_secret_n::<2000>(100), 15273692);
        assert_eq!(next_secret_n::<2000>(2024), 8667524);
    }

    // #[test]
    // fn compare_base() {
    //     assert_eq!(
    //         run(&INPUT_SAMPLE[1..]).unwrap(),
    //         crate::part1::run(&INPUT_SAMPLE[1..]).unwrap()
    //     );

    //     let input = common::read_input_u8!(None).unwrap();
    //     assert_eq!(run(&input).unwrap(), crate::part1::run(&input).unwrap());
    // }
}
