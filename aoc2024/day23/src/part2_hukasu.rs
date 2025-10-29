// Adaptation of Hukasu's code (https://github.com/hukasu/AoC2024/blob/5a4ca26fa62b54994ca99781d03606b801897a8d/src/days/day23.rs)
// which is similar to `part2_brute_force` but build the combination of machines
// manually instead of using Itertools's `tuple_combination`, which allow to
// check for connections gradually and NOT check the connections between machines
// that are part of the same "group prefix" more than once, i.e. we check
// group "[a,b,c,d]" we don't need to check "a-b, a-c, b-c" again when checking
// "[a,b,c,e]" next.
use miette::Result;
use rustc_hash::FxHashSet;

use crate::parse::{parse, Machine};

pub fn run(content: &[u8]) -> Result<String> {
    let connections = parse(content)?;

    let mut conn_hash = FxHashSet::default();
    connections.iter().for_each(|(m1, v)| {
        v.iter().for_each(|m2| {
            // The parser does not return symmetric connections, only the one
            // where m1 < m2
            conn_hash.insert((*m1, *m2));
            conn_hash.insert((*m2, *m1));
        });
    });

    let mut computer_groups = Vec::with_capacity(500);
    connections.iter().for_each(|(pc, node_connections)| {
        get_fully_connected(
            node_connections,
            &conn_hash,
            vec![*pc],
            &mut computer_groups,
        );
    });

    Ok(computer_groups
        .into_iter()
        .max_by_key(|group| group.len())
        .map(|group| {
            group
                .into_iter()
                .map(|pc| String::from_utf8_lossy(&pc.0).to_string())
                .collect::<Vec<_>>()
                .join(",")
        })
        .unwrap())
}

fn get_fully_connected(
    pc_connections: &[Machine],
    all_connections: &FxHashSet<(Machine, Machine)>,
    mut partial: Vec<Machine>,
    fully_connected: &mut Vec<Vec<Machine>>,
) {
    let [head_node, tail @ ..] = pc_connections else {
        partial.sort();
        fully_connected.push(partial);
        return;
    };

    get_fully_connected(tail, all_connections, partial.clone(), fully_connected);

    if partial
        .iter()
        .skip(1)
        .all(|group_node| all_connections.contains(&(*head_node, *group_node)))
    {
        partial.push(*head_node);
        get_fully_connected(tail, all_connections, partial, fully_connected);
    }
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

        assert_eq!(run(input).unwrap(), "co,de,ka,ta");
    }

    #[test]
    fn compare_base() {
        assert_eq!(
            run(&INPUT_SAMPLE[1..]).unwrap(),
            crate::part2::run(&INPUT_SAMPLE[1..]).unwrap()
        );

        // Baseline is way too slow to compare the full input, so hardcode
        // the result for my input
        const EXPECTED: &str = "cc,dz,ea,hj,if,it,kf,qo,sk,ug,ut,uv,wh";
        let input = common::read_input_u8!(None).unwrap();
        assert_eq!(run(&input).unwrap(), EXPECTED);
    }
}
