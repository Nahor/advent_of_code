use std::ops::Range;

use miette::{miette, Result};

use crate::parse::{parse, Coord};

#[derive(Debug, Clone)]
struct Group {
    pub bytes: Vec<Coord>,
    // Boundary of the group including a padding of 1, i.e. the "reach" of the
    // group, that way we can just call `contains` to check if a byte is within
    // the boundaries.
    // Note:
    // - if x_range is [-1..x] or y_range is [y..max_coord+1], the group touches
    // one of the bottom/left edges
    // - similarly, [x..max_coord+1] or [-1..y] means touching one of the
    // top/right edges
    // - touching both means blocking the path
    pub x_range: Range<isize>,
    pub y_range: Range<isize>,
}

pub fn run(content: &[u8], max_coord: isize, _min_amount: usize) -> Result<String> {
    let bytes = parse(content)?;

    let mut groups = Vec::<Group>::new();
    for b in bytes {
        // Find all the groups that are touching the byte...
        let mut matching_groups: Vec<usize> = groups
            .iter()
            .enumerate()
            .filter_map(|(i, group)| {
                if !group.x_range.contains(&b.x) || !group.y_range.contains(&b.y) {
                    // The byte is not in reach of the group, i.e. it can't
                    // belong to the group.
                    return None;
                }

                // The byte is within the bounds, not to check if the byte
                // actually touch one from the group
                if group
                    .bytes
                    .iter()
                    .any(|other| (other.x.abs_diff(b.x) <= 1) && (other.y.abs_diff(b.y) <= 1))
                {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();

        // ... the create/modify/merge the groups as needed
        if matching_groups.is_empty() {
            // The byte doesn't touch anything, it's its own group for now
            let group = Group {
                bytes: vec![b],
                x_range: (b.x - 1)..(b.x + 2), // +1 for the border, +1 for the exclusive range,
                y_range: (b.y - 1)..(b.y + 2), // +1 for the border, +1 for the exclusive range,
            };
            groups.push(group);
        } else {
            // Update/merge one or more groups
            // => add the byte to the last group
            // => merge the other groups into that one

            // The must be sorted because we'll need to remove the groups from
            // `groups` and we don't want to mess with the indices (i.e. we
            // need to process them from the highest index to the lowest)
            //
            // Thankfully, because we search the groups in order, the list
            // should already be sorted
            debug_assert!(matching_groups.is_sorted());

            // We need to (temporarily) remove instead of borrow because we may
            // need to remove more groups afterwards while still modifying this
            // one, which would require two mutable borrows.
            let mut group = groups.remove(matching_groups.pop().unwrap());
            group.bytes.push(b);
            group.x_range = group.x_range.start.min(b.x - 1)..group.x_range.end.max(b.x + 2);
            group.y_range = group.y_range.start.min(b.y - 1)..group.y_range.end.max(b.y + 2);

            // Merge all the other matching groups
            while let Some(index) = matching_groups.pop() {
                let other_group = groups.remove(index);
                group.bytes.extend(other_group.bytes.iter());
                group.x_range = group.x_range.start.min(other_group.x_range.start)
                    ..group.x_range.end.max(other_group.x_range.end);
                group.y_range = group.y_range.start.min(other_group.y_range.start)
                    ..group.y_range.end.max(other_group.y_range.end);
            }

            // +2 because it's an exclusive range
            if ((group.x_range.start == -1) || (group.y_range.end == max_coord + 2))
                && ((group.x_range.end == max_coord + 2) || (group.y_range.start == -1))
            {
                // This byte created a group blocking the path
                return Ok(format!("{},{}", b.x, b.y));
            }

            groups.push(group);
        }
    }

    Err(miette!("no byte blocks the path"))
}

#[cfg(test)]
mod test {
    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input, 6, 1024).unwrap(), "6,1");
    }

    // #[test]
    // fn compare_base() {
    //     assert_eq!(
    //         run(&INPUT_SAMPLE[1..], 6, 12).unwrap(),
    //         crate::part2::run(&INPUT_SAMPLE[1..], 6, 12).unwrap()
    //     );

    //     let input = common::read_input_u8!(None).unwrap();
    //     assert_eq!(
    //         run(&input, 70, 1024).unwrap(),
    //         crate::part2::run(&input, 70, 1024).unwrap()
    //     );
    // }
}
