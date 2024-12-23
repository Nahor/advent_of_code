use miette::Result;
use rustc_hash::FxHashMap;

use crate::parse::{parse, Machine};

pub fn run(content: &[u8]) -> Result<u64> {
    let connections = parse(content)?;
    let groups = filter_chief(&get_groups(connections));
    Ok(groups.len() as u64)
}

fn filter_chief(groups: &[(Machine, Machine, Machine)]) -> Vec<(Machine, Machine, Machine)> {
    groups
        .iter()
        .filter(|(m1, m2, m3)| m1.0[0] == b't' || m2.0[0] == b't' || m3.0[0] == b't')
        .copied()
        .collect::<Vec<_>>()
}

fn get_groups(connections: FxHashMap<Machine, Vec<Machine>>) -> Vec<(Machine, Machine, Machine)> {
    let mut groups = connections
        .iter()
        .flat_map(|(m1, v1)| {
            v1.iter()
                .filter_map(|m2| {
                    let r = connections.get(m2).map(|v2| {
                        let r = v2
                            .iter()
                            .filter_map(|m3| v1.binary_search(m3).map(|_| (*m1, *m2, *m3)).ok());
                        r
                    });
                    r
                })
                .flatten()
        })
        .collect::<Vec<_>>();
    groups.sort();
    groups
}

#[cfg(test)]
mod test {
    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 7);
    }

    #[test]
    fn groups() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n
        let output = "[(aq, cg, yn), (aq, vc, wq), (co, de, ka), (co, de, ta), (co, ka, ta), (de, ka, ta), (kh, qp, ub), (qp, td, wh), (tb, vc, wq), (tc, td, wh), (td, wh, yn), (ub, vc, wq)]";
        let connections = parse(input).unwrap();
        let groups = get_groups(connections);

        let groups_str = format!("{groups:?}").to_string();

        assert_eq!(groups_str, output);
    }

    #[test]
    fn groups_chief() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n
        let output = "[(co, de, ta), (co, ka, ta), (de, ka, ta), (qp, td, wh), (tb, vc, wq), (tc, td, wh), (td, wh, yn)]";
        let connections = parse(input).unwrap();
        let groups = get_groups(connections);
        let groups = filter_chief(&groups);

        let groups_str = format!("{groups:?}").to_string();

        assert_eq!(groups_str, output);
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
