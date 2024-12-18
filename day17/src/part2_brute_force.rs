use std::ops::Range;

pub fn run() {
    // Brute force version of part 2 trying to match the output/input of part 1
    const OUTPUT: usize = 730571405;
    const RANGE_CHECK: Range<usize> = 0..30_000_000;
    const PART1: usize = 28_066_687;

    let solutions = RANGE_CHECK
        .filter(|a| {
            let mut a = *a;
            let mut b;
            let mut c;
            let mut result = 0;
            loop {
                b = a % 8;
                b ^= 1;
                c = a >> b;
                b ^= c;
                a >>= 3;
                b ^= 4;
                result = result * 10 + b % 8;
                if (a == 0) || result > OUTPUT {
                    break;
                }
            }
            result == OUTPUT
        })
        .collect::<Vec<_>>();
    println!("Found {} solutions in {RANGE_CHECK:?}", solutions.len());
    if let Ok(idx) = solutions.binary_search(&PART1) {
        println!("Contains part 1 value at index {idx}");
    } else {
        println!("Part 1 NOT FOUND");
    }
    println!("Least solution: {:?}", solutions.first());
}
