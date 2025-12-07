use common::error::AdventError;
use num::BigUint;

pub fn parse(content: &[u8]) -> Result<(BigUint, BigUint, Vec<BigUint>), AdventError> {
    let mut lines = content
        .split(|b| *b == b'\n')
        .filter(|line| !line.is_empty());

    let start = lines.next().ok_or("Empty data")?;

    let len = start.len();
    let mut mask = BigUint::ZERO;
    (0..len).for_each(|idx| mask.set_bit(idx as u64, true));

    let start_idx = start
        .iter()
        .enumerate()
        .find(|(_, b)| **b == b'S')
        .map(|(i, _)| i)
        .ok_or("No starting point")?;
    let mut start = BigUint::ZERO;
    start.set_bit(start_idx as u64, true);

    let splitters = lines
        .map(|line| parse_line(len, line))
        .filter(|vec| !vec.as_ref().is_ok_and(|v| *v == BigUint::ZERO))
        .collect::<Result<Vec<_>, _>>()?;

    Ok((start, mask, splitters))
}

pub(crate) fn parse_line(len: usize, input: &[u8]) -> Result<BigUint, AdventError> {
    if input.len() != len {
        Err(format!(
            "Invalid splitter line len. Expected {}, got {}",
            len,
            input.len()
        ))?
    }

    let mut v = BigUint::ZERO;
    input.iter().enumerate().try_for_each(|(bit, &b)| match b {
        b'.' => {
            v.set_bit(bit as u64, false);
            Ok(())
        }
        b'^' => {
            v.set_bit(bit as u64, true);
            Ok(())
        }
        _ => Err(format!("Invalid char '{}' (0x{:x})", b as char, b).to_string()),
    })?;

    Ok(v)
}
