use std::collections::BinaryHeap;

use common::error::AdventError;
use glam::I64Vec2;
use itertools::Itertools;
use miette::Result;

use crate::parse::i64::parse;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pair((I64Vec2, I64Vec2), i64);
impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1)
    }
}
impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// (r1,r2) are opposite corners of a rectangle
// (s1,s2) are segment extremities
pub fn intersect((r1, r2): (I64Vec2, I64Vec2), (s1, s2): (I64Vec2, I64Vec2)) -> bool {
    let r_min_x = r1.x.min(r2.x);
    let r_max_x = r1.x.max(r2.x);
    let r_min_y = r1.y.min(r2.y);
    let r_max_y = r1.y.max(r2.y);

    let s_min_x = s1.x.min(s2.x);
    let s_max_x = s1.x.max(s2.x);
    let s_min_y = s1.y.min(s2.y);
    let s_max_y = s1.y.max(s2.y);

    // No intersection if segments in on the left/right/above/below the rectangle.
    // And since we know the segment is horizontal or vertical, we don't need
    // to worry about the case where the segment cuts (or not) a corner
    !((s_max_x <= r_min_x) || (r_max_x <= s_min_x) || (s_max_y <= r_min_y) || (r_max_y <= s_min_y))
}

pub fn run(content: &[u8]) -> Result<i64, AdventError> {
    let points = parse(content)?;

    let segments = points
        .iter()
        .copied()
        .circular_tuple_windows()
        .map(|(p1, p2)| Pair((p1, p2), (p2 - p1).length_squared()))
        .collect::<BinaryHeap<_>>()
        .into_sorted_vec();

    // Creating and sorting by biggest rectangle does not help
    let result = points
        .iter()
        .copied()
        .tuple_combinations()
        .filter(|r| segments.iter().rev().all(|Pair(s, _)| !intersect(*r, *s)))
        .map(|(p1, p2)| {
            let d = (p2 - p1).abs();
            (d.x + 1) * (d.y + 1)
            // ((p2.x.abs_diff(p1.x) + 1) * (p2.y.abs_diff(p1.y) + 1)) as i64
        })
        .max()
        .ok_or("No box found")?;
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 24);
    }

    #[test]
    fn simple() {
        let input = &br#"
2,0
3,0
3,3
0,3
0,1
2,1
"#[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 12);
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
