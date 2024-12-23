use itertools::Itertools;
use miette::Result;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::parse::{parse, Machine};

pub fn run(content: &[u8]) -> Result<String> {
    let connections = parse(content)?;

    let mut groups = get_groups(&connections);
    groups.sort();

    // We have all the possible groups of 3.
    // If there exists a group of 4, we should find two groups of 3 with a
    // common pair of machines, and where the 2 remaining machines (one in
    // each group) are connected together.
    // More generally, if we find two groups of size N sharing N-1 machines,
    // then they form a group of N+1 if the two extras are connected.
    //
    // Given the exercise, we should expect to eventually end with only a single
    // group => the biggest group
    let mut i = 3;
    println!(
        "processing {} groups of {}: {:?}",
        groups.len(),
        i,
        &groups[0..groups.len().min(4)]
    );
    while groups.len() > 1 {
        let hash_group = groups
            .into_iter()
            .map(FxHashSet::from_iter)
            .tuple_combinations()
            .filter_map(|(g1, g2)| {
                let mut other = g1.symmetric_difference(&g2).collect::<Vec<_>>();
                if other.len() != 2 {
                    return None;
                }
                other.sort();
                if !connections
                    .get(other[0])
                    .is_some_and(|v| v.contains(other[1]))
                {
                    return None;
                }

                // Store in a sorted vector so we can remove duplicates via
                // a hashset.
                let mut new_group = g1.union(&g2).copied().collect::<Vec<_>>();
                new_group.sort();

                Some(new_group)
            })
            .collect::<FxHashSet<_>>();
        groups = Vec::from_iter(hash_group);
        groups.sort();

        i += 1;
        println!(
            "=> {} groups of {}: {:?}",
            groups.len(),
            i,
            &groups[0..groups.len().min(4)]
        );
    }
    assert_eq!(groups.len(), 1);

    // Generate the secret code
    let mut group = Vec::from_iter(groups[0].iter().copied());
    group.sort();
    let result_str = group.iter().join(",");

    Ok(result_str)
}

fn get_groups(connections: &FxHashMap<Machine, Vec<Machine>>) -> Vec<Vec<Machine>> {
    connections
        .iter()
        .flat_map(|(m1, v1)| {
            v1.iter()
                .filter_map(|m2| {
                    let r = connections.get(m2).map(|v2| {
                        let r = v2.iter().filter_map(|m3| {
                            v1.binary_search(m3).map(|_| vec![*m1, *m2, *m3]).ok()
                        });
                        r
                    });
                    r
                })
                .flatten()
        })
        .collect::<Vec<_>>()
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
