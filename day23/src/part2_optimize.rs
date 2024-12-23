// Part 2 but with a few optimization
// - use rayon to parallelize the tests
// - use a BTreeSet to avoid the constant back and forth between `Vec`` and
//   `Hashset`
// - use a single hashset to check for existing connections instead of a
//   hashmap + binary search
use std::{collections::BTreeSet, sync::mpsc};

use itertools::Itertools;
use miette::Result;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::parse::{parse, Machine};

pub fn run(content: &[u8]) -> Result<String> {
    let connections = parse(content)?;

    let mut conn_hash = FxHashSet::default();
    conn_hash.extend(
        connections
            .iter()
            .flat_map(|(m1, v)| v.iter().map(|m2| (*m1, *m2))),
    );

    let mut groups = get_groups(&connections);

    // We have all the possible groups of 3.
    // If there exists a group of 4, we should find two groups of 3 with a
    // common pair of machines, and where the 2 remaining machines (one in
    // each group) are connected together.
    // More generally, if we find two groups of size N sharing N-1 machines,
    // then they form a group of N+1 if the two extras are connected.
    //
    // Given the exercise, we should expect to eventually end with only a single
    // group => the biggest group
    #[cfg(test)]
    let mut i = 3;
    #[cfg(test)]
    println!(
        "processing {} groups of {}: {:?}...",
        groups.len(),
        i,
        &groups.first()
    );
    while groups.len() > 1 {
        let mut new_groups = BTreeSet::new();
        rayon::scope(|scope| {
            let (tx, rx) = mpsc::channel();
            for chunk in &groups.iter().tuple_combinations::<(_, _)>().chunks(2000) {
                let tx = tx.clone();
                let conn_hash = &conn_hash;
                let chunk = chunk.collect::<Vec<_>>();
                scope.spawn(move |_| {
                    chunk.into_iter().for_each(|(g1, g2)| {
                        let mut other = g1.symmetric_difference(g2).copied().collect::<Vec<_>>();
                        if other.len() != 2 {
                            return;
                        }
                        other.sort();
                        if !conn_hash.contains(&(other[0], other[1])) {
                            return;
                        }

                        let mut new_g = g1.clone();
                        new_g.extend(other);
                        let _ = tx.send(new_g);
                    });
                })
            }
            drop(tx);
            while let Ok(group) = rx.recv() {
                new_groups.insert(group);
            }
        });
        groups = new_groups;

        #[cfg(test)]
        {
            i += 1;
            println!(
                "=> {} groups of {}: {:?}...",
                groups.len(),
                i,
                &groups.first()
            );
        }
    }
    assert_eq!(groups.len(), 1);

    // Generate the secret code
    let result_str = groups.first().unwrap().iter().join(",");

    Ok(result_str)
}

fn get_groups(connections: &FxHashMap<Machine, Vec<Machine>>) -> BTreeSet<BTreeSet<Machine>> {
    connections
        .iter()
        .flat_map(|(m1, v1)| {
            v1.iter()
                .filter_map(|m2| {
                    let r = connections.get(m2).map(|v2| {
                        let r = v2.iter().filter_map(|m3| {
                            v1.binary_search(m3)
                                .map(|_| BTreeSet::from([*m1, *m2, *m3]))
                                .ok()
                        });
                        r
                    });
                    r
                })
                .flatten()
        })
        .collect::<BTreeSet<_>>()
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
