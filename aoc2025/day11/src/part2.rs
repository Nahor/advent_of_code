use common::error::AdventError;
use miette::Result;
use rustc_hash::FxHashMap;

use crate::parse::parse;

fn get_count<'a>(
    node: &'a [u8],
    target: &'a [u8],
    list: &FxHashMap<&'a [u8], Vec<&'a [u8]>>,
    cache: &mut FxHashMap<(&'a [u8], &'a [u8]), usize>,
) -> usize {
    if node == target {
        return 1;
    }
    let Some(children) = list.get(node) else {
        return 0;
    };

    children
        .iter()
        .map(|child| {
            if let Some(&c) = cache.get(&(*child, target)) {
                c
            } else {
                let c = get_count(child, target, list, cache);
                cache.insert((*child, target), c);
                c
            }
        })
        .sum::<usize>()
}

pub fn run(content: &[u8]) -> Result<usize, AdventError> {
    let list = parse(content)?;

    let mut cache = Default::default();
    let result = (
        //svr -> dac -> fft -> out
        get_count(b"svr", b"dac", &list, &mut cache)
            * get_count(b"dac", b"fft", &list, &mut cache)
            * get_count(b"fft", b"out", &list, &mut cache)
    ) + (
        //svr -> fft -> dac -> out
        get_count(b"svr", b"fft", &list, &mut cache)
            * get_count(b"fft", b"dac", &list, &mut cache)
            * get_count(b"dac", b"out", &list, &mut cache)
    );

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 2);
    }

    // #[test]
    // fn compare_base() {
    //     assert_eq!(
    //         run(&INPUT_SAMPLE[1..]).unwrap(),
    //         crate::part2::run(&INPUT_SAMPLE[1..]).unwrap()
    //     );

    //     let input = common::read_input_u8!(None).unwrap();
    //     assert_eq!(run(&input).unwrap(), crate::part2::run(&input).unwrap());
    // }
}
