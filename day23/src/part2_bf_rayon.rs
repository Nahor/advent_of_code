// Different solution: use a mostly brute-force approach where, for a given
// machine, we search all the possible groups of a given size where all the
// machines are connected to each other.
// This works well because we know all the connections for a given machine (
// which sets an upper-bound for the group size) and that number of connections
// is moderate in practice (which limits the number of combinations to try)
use itertools::Itertools;
use miette::Result;
use rayon::prelude::*;
use rustc_hash::FxHashSet;

use crate::parse::parse;

pub fn run(content: &[u8]) -> Result<String> {
    let connections = parse(content)?;

    let mut conn_hash = FxHashSet::default();
    connections.iter().for_each(|(m1, v)| {
        v.iter().for_each(|m2| {
            conn_hash.insert((*m1, *m2));
        });
    });

    // A group cannot be bigger than the biggest  number of connections any
    // machine has
    let groups_max_size = connections
        .values()
        .map(|v| v.len() + 1)
        .max()
        .expect("should have at least one connection");

    // Faster than using the hashmap when using Rayon (maybe because )
    let connections = connections.into_iter().collect::<Vec<_>>();

    // Binary search to find the group size.
    // The puzzle expects a single "biggest group". From part 1, we already
    // know that there are multiple groups of 3, so the max group is at least
    // of size 4. Start with one below that to ensure the search will check it
    let mut range = 1..(groups_max_size + 1);
    let mut cur_max_group = vec![];
    while range.len() > 1 {
        let mid = (range.end + range.start) / 2;
        #[cfg(test)]
        println!("range: {range:?}, trying {mid}");

        let tmp_group = connections.par_iter().find_map_any(|(m1, v)| {
            // By construction, we already `m1` is connected to the other
            // machines in its connection list. So we only need to find
            // a combination of size `mid-1` in this list where all the machines
            // are connected to each other.
            v.iter()
                .combinations(mid - 1)
                .find(|g| {
                    g.iter().tuple_combinations().all(|(m2, m3)| {
                        // Since the connections are sorted, we know `m2 < m3`, and
                        // that it will be listed in that order in `conn_hash`
                        // i.e. we don't need to check for `(m3,m2)`
                        conn_hash.contains(&(**m2, **m3))
                    })
                })
                .map(|g| {
                    // convert the "m1 + group-of-size-mid-1" into a full group
                    let mut full_group = Vec::with_capacity(g.len() + 1);
                    full_group.push(*m1);
                    full_group.extend(g.into_iter().copied());
                    full_group
                })
        });

        if let Some(tmp_group) = tmp_group {
            range = mid..range.end;
            cur_max_group = tmp_group;
        } else {
            range = range.start..mid;
        }
    }
    #[cfg(test)]
    println!("final range: {range:?}");

    // Generate the secret code
    let result_str = cur_max_group.iter().join(",");

    Ok(result_str)
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
