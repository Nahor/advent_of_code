use glam::I64Vec3;
use itertools::Itertools;
use miette::Result;
use rustc_hash::FxHashMap;

use crate::parse::parse;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct DistanceVec {
    v: i64,
}
impl DistanceVec {
    pub fn new(point1: I64Vec3, point2: I64Vec3, idx1: u16, idx2: u16) -> Self {
        let len = (point1 - point2).length_squared();
        let v = len << 20 | ((idx1 as i64) << 10) | (idx2 as i64);
        DistanceVec { v }
    }
    pub fn idx1(&self) -> usize {
        ((self.v >> 10) & 0x3ff) as usize
    }
    pub fn idx2(&self) -> usize {
        (self.v & 0x3ff) as usize
    }
}
impl Ord for DistanceVec {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // distance (reversed, since we want the smallest distance first)
        self.v.cmp(&other.v).reverse()
    }
}
impl PartialOrd for DistanceVec {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn run(content: &[u8]) -> Result<i64> {
    let points = parse(content)?;
    let mut distances = Vec::with_capacity((points.len() * points.len() - points.len()) / 2);

    // let s = std::time::Instant::now();
    points
        .as_slice()
        .iter()
        .enumerate()
        .tuple_combinations()
        .for_each(|((idx1, point1), (idx2, point2))| {
            distances.push(DistanceVec::new(*point1, *point2, idx1 as u16, idx2 as u16));
        });
    // println!("ins: {:?}", s.elapsed());

    distances.sort_unstable();
    // println!("sort: {:?}", s.elapsed());

    let mut circuit_from_points = FxHashMap::default();
    let mut points_from_circuit = vec![];

    let last = loop {
        let line = distances.pop().expect("Not enough lines");
        let idx1 = line.idx1();
        let idx2 = line.idx2();

        match (
            circuit_from_points.get(&idx1),
            circuit_from_points.get(&idx2),
        ) {
            (None, None) => {
                let circuit = points_from_circuit.len();
                circuit_from_points.insert(idx1, circuit);
                circuit_from_points.insert(idx2, circuit);
                points_from_circuit.push(vec![idx1, idx2]);
            }
            (None, Some(&circuit)) => {
                circuit_from_points.insert(idx1, circuit);
                points_from_circuit[circuit].push(idx1);

                if points_from_circuit[circuit].len() == points.len() {
                    break line;
                }
            }
            (Some(&circuit), None) => {
                circuit_from_points.insert(idx2, circuit);
                points_from_circuit[circuit].push(idx2);

                if points_from_circuit[circuit].len() == points.len() {
                    break line;
                }
            }
            (Some(&circuit1), Some(&circuit2)) if circuit1 == circuit2 => {
                // same circuit => ignore
            }
            (Some(&circuit1), Some(&circuit2)) => {
                points_from_circuit[circuit2].iter().for_each(|point| {
                    *circuit_from_points.get_mut(point).unwrap() = circuit1;
                });
                let mut temp = vec![];
                std::mem::swap(&mut points_from_circuit[circuit2], &mut temp);
                points_from_circuit[circuit1].extend(temp);

                if points_from_circuit[circuit1].len() == points.len() {
                    break line;
                }
            }
        }
    };

    Ok(points[last.idx1()].x * points[last.idx2()].x)
}

#[cfg(test)]
mod test {
    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 25272);
    }

    #[test]
    fn compare_base() {
        assert_eq!(
            run(&INPUT_SAMPLE[1..]).unwrap(),
            crate::part2::run(&INPUT_SAMPLE[1..]).unwrap()
        );

        let input = common::read_input_u8!(None).unwrap();
        assert_eq!(run(&input).unwrap(), crate::part2::run(&input).unwrap());
    }
}
