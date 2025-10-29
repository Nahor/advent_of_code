#![allow(clippy::mutable_key_type)]
use std::collections::{HashSet, VecDeque};

use day22::*;

fn main() -> miette::Result<()> {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../input/2023/",
        env!("CARGO_PKG_NAME"),
        "/input.txt"
    ));
    let output = process(input)?;
    dbg!(output);
    Ok(())
}

fn process(input: &'_ str) -> Result<usize, AocError<'_>> {
    let bricks = parse(input)?;
    let _graph = build_graph(&bricks);
    // println!("{bricks:#?}");

    // A brick is not required as support if all its children have at least
    // one parent whose max height is the same as the brick
    let output = bricks
        .iter()
        .map(|brick_cell| {
            let brick = brick_cell.borrow();

            let mut falling = HashSet::from([brick_cell.clone()]);
            let mut pending = brick.supports.iter().cloned().collect::<VecDeque<_>>();
            while let Some(brick_cell) = pending.pop_front() {
                let brick = brick_cell.borrow();
                let no_support = brick
                    .supported_by
                    .iter()
                    .all(|parent_cell| falling.contains(parent_cell));
                if no_support {
                    falling.insert(brick_cell.clone());
                    pending.extend(brick.supports.iter().cloned());
                }
            }

            println!(
                "Brick {} brings down {}",
                brick.get_name(),
                falling.len() - 1
            );

            falling.len() - 1
        })
        .sum();

    Ok(output)
}

// fn can_remove(brick: &Brick) -> bool {
//     brick.supported_by.iter().filter(predicate)
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() -> miette::Result<()> {
        let input = "\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
";
        assert_eq!(process(input)?, 7);

        Ok(())
    }
}
