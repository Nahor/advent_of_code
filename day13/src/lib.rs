use aocerror::AocError;

pub mod aocerror;

pub fn parse(input: &str, smudges: usize) -> Result<usize, AocError> {
    let patterns = input
        .split("\n\n")
        .map(|pattern| pattern.lines().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let count: usize = patterns
        .iter()
        .enumerate()
        .filter_map(|(patter_no, pattern)| {
            let v = vert_mirror(patter_no, pattern, smudges);
            let h = horiz_mirror(patter_no, pattern, smudges);
            v.or_else(|| h)
        })
        .sum();

    Ok(count)
}

fn vert_mirror(pattern_no: usize, pattern: &Vec<&str>, smudges: usize) -> Option<usize> {
    let width = pattern[0].len();
    // println!("Width: {width}");
    let col = (1..width)
        .into_iter()
        //.inspect(|c| println!("Checking col {c}"))
        .filter(|split| {
            let left = (0..(*split)).into_iter().rev();
            let right = ((*split)..width).into_iter();
            let combined = left.zip(right);

            let error_count = combined
                .map(|(left, right)| {
                    //println!("{left} - {right}");
                    pattern
                        .iter()
                        .filter(|&line| line.as_bytes()[left] != line.as_bytes()[right])
                        .count()
                })
                .sum::<usize>();
            error_count == smudges
        })
        .inspect(|split| {
            println!("{pattern_no}: Found col {split:?}");
        })
        .collect::<Vec<_>>();
    col.get(0).copied()
}

fn horiz_mirror(pattern_no: usize, pattern: &Vec<&str>, smudges: usize) -> Option<usize> {
    let height = pattern.len();
    // println!("Height: {height}");
    let line = (1..height)
        .into_iter()
        // .inspect(|c| println!("Checking ln {c}"))
        .filter(|split| {
            let top = (0..(*split)).into_iter().rev();
            let bottom = ((*split)..height).into_iter();
            let combined = top.zip(bottom);

            let error_count = combined
                .map(|(top, bottom)| {
                    // println!("{top} - {bottom}");
                    pattern[top]
                        .as_bytes()
                        .iter()
                        .zip(pattern[bottom].as_bytes().iter())
                        .filter(|(a, b)| a != b)
                        .count()
                })
                .sum::<usize>();
            error_count == smudges
        })
        .inspect(|count| {
            println!("{pattern_no}: Found line {count:?}");
        })
        .map(|count| count * 100)
        .collect::<Vec<_>>();

    line.get(0).copied()
}
