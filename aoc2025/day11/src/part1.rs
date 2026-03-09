use common::error::AdventError;
use miette::Result;
use rustc_hash::FxHashMap;

use crate::parse::parse;

fn get_count<'a>(
    node: &'a [u8],
    list: &FxHashMap<&'a [u8], Vec<&'a [u8]>>,
    cache: &mut FxHashMap<&'a [u8], usize>,
) -> usize {
    if node == b"out" {
        return 1;
    }
    let Some(children) = list.get(node) else {
        return 0;
    };

    children
        .iter()
        .map(|child| {
            if let Some(&c) = cache.get(child) {
                c
            } else {
                let c = get_count(child, list, cache);
                cache.insert(child, c);
                c
            }
        })
        .sum::<usize>()
}

pub fn run(content: &[u8]) -> Result<usize, AdventError> {
    let list = parse(content)?;

    let mut cache = Default::default();
    let result = get_count(b"you", &list, &mut cache);

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 5);
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
