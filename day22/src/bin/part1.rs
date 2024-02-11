use day22::*;
use miette;

fn main() -> miette::Result<()> {
    let input = include_str!("input.txt");
    let output = process(input)?;
    dbg!(output);
    Ok(())
}

fn process(input: &str) -> Result<usize, AocError> {
    let bricks = parse(input)?;
    let _graph = build_graph(&bricks);
    // println!("{bricks:#?}");

    // A brick is not required as support if all its children have at least
    // one parent whose max height is the same as the brick
    let output = bricks
        .iter()
        .filter(|brick_cell| {
            let brick = brick_cell.borrow();
            let all = brick
                .supports
                .iter()
                .all(|child| child.borrow().supported_by.len() > 1);
            if all {
                println!("brick {} can be dropped", brick.get_name());
            }
            all
        })
        .count();

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
        assert_eq!(process(input)?, 5);

        Ok(())
    }
}
