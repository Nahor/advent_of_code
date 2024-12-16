use std::ops::RangeInclusive;

use day24::*;

#[cfg(feature = "nalgebra")]
use nalgebra::*;

fn main() -> miette::Result<()> {
    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../advent_of_code_input/2023/",
        env!("CARGO_PKG_NAME"),
        "/input.txt"
    ));
    let output = process(input, 200_000_000_000_000f64..=400_000_000_000_000f64)?;
    dbg!(output);
    Ok(())
}

fn process(input: &str, range: RangeInclusive<f64>) -> Result<i64, AocError> {
    let hail = parse(input)?;

    let mut count = 0;
    for i in 0..hail.len() - 1 {
        for j in (i + 1)..hail.len() {
            if let Some((t, u)) = segment_intersection(hail[i], hail[j]) {
                let cross = hail[i].position + hail[i].velocity * t;

                if range.contains(&cross.x) && range.contains(&cross.y) && t >= 0.0 && u >= 0.0 {
                    //println!("{:?}", cross);
                    count += 1;
                }
            }
        }
    }

    Ok(count)
}

#[cfg(not(feature = "nalgebra"))]
fn segment_intersection(s1: Stone, s2: Stone) -> Option<(f64, f64)> {
    // https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection
    let (x1, x2) = (s1.position.x, s1.position.x + s1.velocity.x);
    let (y1, y2) = (s1.position.y, s1.position.y + s1.velocity.y);
    let (x3, x4) = (s2.position.x, s2.position.x + s2.velocity.x);
    let (y3, y4) = (s2.position.y, s2.position.y + s2.velocity.y);

    let div = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
    if div == 0.0 {
        // Parallel segment
        return None;
    }
    let num1 = (x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4);
    let num2 = (x1 - x3) * (y1 - y2) - (y1 - y3) * (x1 - x2);

    let t = num1 / div;
    let u = num2 / div;

    // let x = x1 + (t * (x2 - x1));
    // let y = y1 + (t * (y2 - y1));

    return Some((t, u));
}

#[cfg(feature = "nalgebra")]
fn segment_intersection(s1: Stone, s2: Stone) -> Option<(f64, f64)> {
    // Cramer's rule (https://en.wikipedia.org/wiki/Cramer%27s_rule)
    //    A x = b
    //
    // Using the following set of equations:
    //    X  =  s1.v.x * t + s1.p.x  =  s2.v.x * u + s2.p.x
    //    Y  =  s1.v.y * t + s1.p.y  =  s2.v.y * u + s2.p.y
    // where (X,Y) are the intersection coordinates
    //
    // so we get:
    //    s1.v.x * t - s2.v.x * u  =  s2.p.x - s1.p.x
    //    s1.v.y * t - s2.v.y * u  =  s2.p.y - s1.p.y
    // i.e.:
    //    | s1.v.x, -s2.v.x | * | t |  =  | s2.p.x - s1.p.x |
    //    | s1.v.y, -s2.v.y |   | u |     | s2.p.y - s1.p.y |

    let b = Vector2::from([s2.position.x - s1.position.x, s2.position.y - s1.position.y]);
    let a = Matrix2::from_rows(&[
        RowVector2::new(s1.velocity.x, -s2.velocity.x),
        RowVector2::new(s1.velocity.y, -s2.velocity.y),
    ]);
    let x = a.qr().solve(&b);

    x.map(|x| (x[0], x[1]))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() -> miette::Result<()> {
        let input = "\
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
";
        assert_eq!(process(input, 7.0..=28.0,)?, 2);

        Ok(())
    }
}
