use miette::{miette, Result};

use crate::parse::parse;

pub fn run(content: &[u8]) -> Result<usize> {
    let machine = parse(content)?;

    // I can't think of a practical generic solution. So let's ensure we are
    // working with the same program as mine. I suspect that all the programs
    // are the same, with only some swapped instructions.
    assert_eq!(
        machine.program(),
        &vec![2, 4, 1, 1, 7, 5, 4, 6, 0, 3, 1, 4, 5, 5, 3, 0]
    );

    // Code:
    //    b = a % 8; // B = bits 2..0 of A
    //    b ^= 1;
    //    c = a >> b; // C = bits ..8 of A
    //    b ^= c; // so the 3 LSB of B combine bits 2..0 + 9..7 of A at most
    //    a >>= 3;
    //    b ^= 4;
    //    ... b % 8; // so output depends on bits 9..0 of A
    //    loop if A!=0
    //
    // => so we only need the least 10 bits of A to know the next output
    // (then A is r-shifted by 3 and we loop until A==0)
    // => we should be able to build a lookup table to match the expected
    // output to the values of a 10-bit A
    //
    // Note: strictly speaking, we only need 6-bits of A, that including the
    // in-between bits simplifies the code.

    // Build a lookup table for all the values of A leading to the next output
    let lookup = get_lookup_table();

    compute_reg_value(machine.program(), 0, &lookup).ok_or_else(|| miette!("no solution found"))
}

fn get_lookup_table() -> Vec<Vec<usize>> {
    let mut lookup = vec![vec![]; 8];
    // 0 is not a valid input since the program exits in that case (and we
    // never start with A=0)
    for a in 1..(1 << 10) {
        let mut b = a % 8;
        b ^= 1;
        let c = a >> b;
        b ^= c;
        b ^= 4;
        b %= 8;
        lookup[b].push(a);
    }
    lookup
}

fn compute_reg_value(prog: &[usize], a: usize, lookup: &Vec<Vec<usize>>) -> Option<usize> {
    let Some(&out) = prog.last() else {
        // no more program to match
        return Some(a);
    };

    // Reverse the ">>3" of the program
    let next_a = a << 3;
    // Remove the MSB that are not used in the lookup
    let masked_a = next_a & ((1 << 10) - 1);
    // Only the last 3 LSB are unknown, so this is the range of viable values
    let match_range = masked_a..(masked_a + 0x8);

    let possibles = &lookup[out];
    for possible in possibles {
        if !match_range.contains(possible) {
            // Not a viable value for the masked A
            continue;
        }

        // Add the missing bits (no need to mask since bits 3..9 are the same
        // by construction)
        let new_a = next_a | possible;

        // It matches, try to find the remaining bits
        let r = compute_reg_value(&prog[0..prog.len() - 1], new_a, lookup);
        if r.is_some() {
            return r;
        }
    }

    // We couldn't find a valid value with A as MSB
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lookup_table() {
        let lookup = get_lookup_table();

        // Verify that the lookup table is correct, i.e. that we can find the
        // pieces from part1
        assert!(lookup[7].binary_search_by(|&v| v.cmp(&0x37f)).is_ok());
        assert!(lookup[3].binary_search_by(|&v| v.cmp(&0x6f)).is_ok());
        assert!(lookup[3].binary_search_by(|&v| v.cmp(&0x6e)).is_ok());
        assert!(lookup[0].binary_search_by(|&v| v.cmp(&0x10d)).is_ok());
        assert!(lookup[5].binary_search_by(|&v| v.cmp(&0x221)).is_ok());
        assert!(lookup[7].binary_search_by(|&v| v.cmp(&0x2c4)).is_ok());
        assert!(lookup[1].binary_search_by(|&v| v.cmp(&0x358)).is_ok());
        assert!(lookup[4].binary_search_by(|&v| v.cmp(&0x6b)).is_ok());
        assert!(lookup[0].binary_search_by(|&v| v.cmp(&0xd)).is_ok());
        assert!(lookup[5].binary_search_by(|&v| v.cmp(&0x1)).is_ok());
    }

    #[test]
    fn from_part1() {
        // Output of part1
        let prog = vec![7, 3, 0, 5, 7, 1, 4, 0, 5];
        let expected = 28057973; // The A value from part 1 (28066687) is not the lowest

        let lookup = get_lookup_table();
        assert_eq!(compute_reg_value(&prog, 0, &lookup), Some(expected));
    }
}
