use std::{collections::BinaryHeap, fmt::Display};

use glam::I64Vec3;
use itertools::Itertools;
use miette::Result;
use rustc_hash::FxHashMap;

use crate::parse::parse;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DistanceVec {
    point1: I64Vec3,
    point2: I64Vec3,
}
impl DistanceVec {
    pub fn length_squared(&self) -> i64 {
        self.coord().length_squared()
    }
    pub fn coord(&self) -> I64Vec3 {
        self.point1 - self.point2
    }
}
impl Ord for DistanceVec {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // distance (reversed, since we want the smallest distance first)
        self.length_squared().cmp(&other.length_squared()).reverse()
    }
}
impl PartialOrd for DistanceVec {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
    // fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    //     match self.length_squared().partial_cmp(&other.length_squared()) {
    //         Some(core::cmp::Ordering::Equal) => {}
    //         Some(ord) => return Some(ord.reverse()),
    //         None => return None,
    //     }
    //     // if self.length_squared()
    //     //     .partial_cmp(&other.length_squared()).
    //     //     //.reverse()
    //     //     // point1
    //     //     .then_with(|| self.point1.x.partial_cmp(&other.point2.x))
    //     //     .then_with(|| self.point1.y.partial_cmp(&other.point1.y))
    //     //     .then_with(|| self.point1.z.partial_cmp(&other.point1.z))
    //     //     //point2
    //     //     .then_with(|| self.point2.x.partial_cmp(&other.point2.x))
    //     //     .then_with(|| self.point2.y.partial_cmp(&other.point2.y))
    //     //     .then_with(|| self.point2.z.partial_cmp(&other.point2.z))

    //     // point1
    //     match self.point1.x.partial_cmp(&other.point2.x) {
    //         Some(core::cmp::Ordering::Equal) => {}
    //         ord => return ord,
    //     }
    //     match self.point1.y.partial_cmp(&other.point2.y) {
    //         Some(core::cmp::Ordering::Equal) => {}
    //         ord => return ord,
    //     }
    //     match self.point1.z.partial_cmp(&other.point2.z) {
    //         Some(core::cmp::Ordering::Equal) => {}
    //         ord => return ord,
    //     }

    //     // point2
    //     match self.point2.x.partial_cmp(&other.point2.x) {
    //         Some(core::cmp::Ordering::Equal) => {}
    //         ord => return ord,
    //     }
    //     match self.point2.y.partial_cmp(&other.point2.y) {
    //         Some(core::cmp::Ordering::Equal) => {}
    //         ord => return ord,
    //     }
    //     self.point2.z.partial_cmp(&other.point2.z)
    // }
}
impl Display for DistanceVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.point1, self.point2)
    }
}

pub fn run(content: &[u8], mut iteration: usize) -> Result<usize> {
    let points = parse(content)?;
    let mut distances = BinaryHeap::new();

    points
        .as_slice()
        .iter()
        .copied()
        .tuple_combinations()
        .for_each(|(point1, point2)| {
            distances.push(DistanceVec { point1, point2 });
        });

    let mut circuit_from_points = FxHashMap::default();
    let mut points_from_circuit = vec![];

    while iteration > 0 {
        let line = distances.pop().expect("Not enough lines");

        match (
            circuit_from_points.get(&line.point1),
            circuit_from_points.get(&line.point2),
        ) {
            (None, None) => {
                let circuit = points_from_circuit.len();
                circuit_from_points.insert(line.point1, circuit);
                circuit_from_points.insert(line.point2, circuit);
                points_from_circuit.push(vec![line.point1, line.point2]);
            }
            (None, Some(&circuit)) => {
                circuit_from_points.insert(line.point1, circuit);
                points_from_circuit[circuit].push(line.point1);
            }
            (Some(&circuit), None) => {
                circuit_from_points.insert(line.point2, circuit);
                points_from_circuit[circuit].push(line.point2);
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
            }
        }

        iteration -= 1;
    }

    assert!(points_from_circuit.len() >= 3, "not enough circuits");

    points_from_circuit.sort_unstable_by(|c1, c2| c1.len().cmp(&c2.len()).reverse());
    while points_from_circuit.pop_if(|c| c.is_empty()).is_some() {}

    let result: usize =
        points_from_circuit[0].len() * points_from_circuit[1].len() * points_from_circuit[2].len();

    Ok(result)
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

        assert_eq!(run(input, 10).unwrap(), 40);
    }
}
