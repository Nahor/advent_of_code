use std::{collections::BTreeMap, iter::repeat};

use aocerror::AocError;
use itertools::Itertools;

pub mod aocerror;

pub fn parse(input: &str, fold: usize, with_validation: bool) -> Result<usize, AocError> {
    let _ = input
        .lines()
        .map(|line| {
            line.as_bytes()
                .into_iter()
                .copied()
                .filter(|&b| b == b'?')
                .count()
        })
        .max();

    Ok(input
        .lines()
        .map(|line| process_line(line, fold, with_validation))
        .sum())
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub len: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum SpringTypes {
    Unknown,
    Working,
    Broken,
}
impl Into<char> for SpringTypes {
    fn into(self) -> char {
        match self {
            SpringTypes::Unknown => '?',
            SpringTypes::Working => '.',
            SpringTypes::Broken => '#',
        }
    }
}

type Cache = BTreeMap<(usize, usize), usize>;

fn process_line(line: &str, fold: usize, with_validation: bool) -> usize {
    let (mapping, count) = line.split_once(' ').unwrap();

    let mapping = repeat(mapping).take(fold).join("?");
    let count = repeat(count).take(fold).join(",");

    // Create sequence of broken springs. Add a single working spring as
    // separator to simplify the code afterwards (i.e. there will be between
    // 0 and x working springs between chunks)
    let chunks = count
        .split(',')
        .map(|num_str| num_str.parse::<usize>().unwrap() + 1)
        .collect::<Vec<_>>();

    // Because chunks ends with a working spring, add a working spring to the
    // mapping in case the real solution ends with a broken spring.
    let mapping = mapping
        .chars()
        .map(|c| match c {
            '?' => SpringTypes::Unknown,
            '.' => SpringTypes::Working,
            '#' => SpringTypes::Broken,
            c => panic!("invalid char '{c}'"),
        })
        .chain([SpringTypes::Working].into_iter())
        .collect::<Vec<_>>();

    // K = (chunk id, position)
    // V = count
    let mut cache = Cache::new();
    let count = get_arrangement_count(&mut cache, &mapping, &chunks, 0, 0);

    // println!("{line}: {count:03}");
    if with_validation {
        assert_eq!(count, validate(&mapping, &chunks));
    }

    count
}

fn get_arrangement_count(
    cache: &mut Cache,
    mapping: &Vec<SpringTypes>,
    chunks: &Vec<usize>,
    chunk_id: usize,
    position: usize,
) -> usize {
    if chunk_id >= chunks.len() {
        // All the chunks have fit, make sure there are no known broken springs
        // left in the mapping
        return mapping[position..]
            .iter()
            .all(|&c| c == SpringTypes::Unknown || c == SpringTypes::Working)
            as usize;
    }

    let value = cache.get(&(chunk_id, position));
    if let Some(count) = value {
        return *count;
    }

    let mut count = 0;

    let (fit, next_pos) = verify(mapping, chunks[chunk_id], position);

    // This chunk fits, check the rest
    if fit {
        count += get_arrangement_count(
            cache,
            mapping,
            chunks,
            chunk_id + 1,
            position + chunks[chunk_id],
        )
    };

    // Add the other position for this chunk
    if let Some(next_pos) = next_pos {
        count += get_arrangement_count(cache, mapping, chunks, chunk_id, next_pos);
    }

    cache.insert((chunk_id, position), count);

    count
}

// Return a `bool`` to indicate if `chunk`` fits at position `pos` and
// an `Option(usize)` for the next position to try (`None` meaning it can never
// match)
fn verify(mapping: &Vec<SpringTypes>, chunk_len: usize, pos: usize) -> (bool, Option<usize>) {
    if mapping.len() < pos + chunk_len {
        return (false, None);
    }
    let (broken, working) = mapping.iter().enumerate().skip(pos).take(chunk_len).fold(
        (None, None),
        |(mut broken, mut working), (charno, mapping_c)| {
            match mapping_c {
                SpringTypes::Unknown => {}
                SpringTypes::Working => {
                    working = match working {
                        Some((first_working, _)) => Some((first_working, charno)),
                        None => Some((charno, charno)),
                    };
                }
                SpringTypes::Broken => {
                    broken = match broken {
                        Some((first_broken, _)) => Some((first_broken, charno)),
                        None => Some((charno, charno)),
                    };
                }
            };
            (broken, working)
        },
    );
    let res = match (broken, working) {
        // Chunk fits
        (None, None) => {
            // Just unknown springs
            (true, Some(pos + 1))
        }
        (Some((first_broken, last_broken)), None) if last_broken < (pos + chunk_len - 1) => {
            // Cannot end with a broken pipe (end must be either working or unknown)
            let next = if first_broken == pos {
                None
            } else {
                Some(pos + 1)
            };
            (true, next)
        }
        (None, Some((first_working, _))) if first_working == (pos + chunk_len - 1) => {
            // First (and only) working pipe must be at then end
            // Since there are no known broken pipe, next option will be after the working pipe
            (true, Some(first_working + 1))
        }
        (Some(_), Some((first_working, _))) if first_working == (pos + chunk_len - 1) => {
            // First (and only) working pipe must be at then end
            // There some broken pipe (necessarily before the working one) so
            // there are no other option
            (true, None)
        }

        (None, Some((first_working, _))) if first_working == (pos + chunk_len - 1) => {
            // No broken spring and ends with the only working spring
            // => fits, but next option can only be after the working spring
            (true, Some(first_working + 1))
        }
        (Some(_), Some((first_working, _))) if first_working == (pos + chunk_len - 1) => {
            // Has some broken springs, ends with the only working spring
            // => fits, but nothing else can
            (true, None)
        }

        // Chunk does NOT fit
        (Some((first_broken, _)), None) => {
            let next = if first_broken == pos {
                None
            } else {
                Some(pos + 1)
            };
            (false, next)
        }
        (None, Some((_, last_working))) => {
            // If there are any broken spring, it will be after the last working
            // one
            (false, Some(last_working + 1))
        }
        (Some((first_broken, _)), Some((_, last_working))) if first_broken < last_working => {
            (false, None)
        }
        (Some(_), Some((_, last_working))) => (false, Some(last_working + 1)),
    };

    // println!(
    //     "Fitting chunk of len {chunk_len} into {}: {res:?}",
    //     mapping[pos..pos + chunk_len]
    //         .iter()
    //         .map(|s| Into::<char>::into(*s))
    //         .collect::<String>()
    // );

    res
}

fn validate(mapping: &Vec<SpringTypes>, chunks: &Vec<usize>) -> usize {
    let mut pos_list = Vec::new();
    let chunk_total_len = chunks.iter().fold(0, |pos, chunk_len| {
        pos_list.push(pos);
        pos + chunk_len
    });
    let leeway = mapping.len() - chunk_total_len;
    // println!("Base: tot: {chunk_total_len}, leeway: {leeway}, pos_list: {pos_list:?}");
    let ranges = pos_list.iter().map(|&pos| pos..(pos + leeway + 1));

    let mut mapped = vec![SpringTypes::Unknown; mapping.len()];
    let arrangements = ranges.multi_cartesian_product();
    println!(
        "Number of arrangements to test: {:?}",
        arrangements.try_len()
    );
    let count = arrangements
        // .inspect(|v| println!("Arrangement: {v:?}"))
        .filter(|arrangement| {
            assert_eq!(arrangement.len(), chunks.len());
            arrangement
                .iter()
                .tuple_windows()
                .enumerate()
                .all(|(i, (&a, &b))| a + chunks[i] <= b)
        })
        .filter(|arrangement| {
            mapped.fill(SpringTypes::Working);
            arrangement
                .iter()
                .zip(chunks.iter())
                .for_each(|(&pos, &chunk_len)| {
                    mapped[pos..(pos + chunk_len - 1)].fill(SpringTypes::Broken);
                });
            compare(&mapping, &mapped)
        })
        .count();

    println!("Validation count: {count}");
    count
}

fn compare(mapping: &Vec<SpringTypes>, mapped: &Vec<SpringTypes>) -> bool {
    assert_eq!(mapping.len(), mapped.len());
    let r = mapping
        .iter()
        .try_fold(mapped.iter(), |mut mapped_iter, &mapping_char| {
            let &mapped_char = mapped_iter.next().unwrap();
            if mapping_char != SpringTypes::Unknown && mapping_char != mapped_char {
                None
            } else {
                Some(mapped_iter)
            }
        })
        .is_some();

    if r {
        println!(
            "Compare ({r}): {} <-> {}",
            mapping
                .iter()
                .map(|&c| Into::<char>::into(c))
                .collect::<String>(),
            mapped
                .iter()
                .map(|&c| Into::<char>::into(c))
                .collect::<String>()
        );
    }

    r
}

#[cfg(test)]
mod test {
    //use super::*;

    // #[test]
    // fn test1() {
    //     assert!(compare(&vec!['?'], &vec!['.']));
    //     assert!(compare(&vec!['?'], &vec!['#']));
    //     assert!(compare(&vec!['.'], &vec!['.']));
    //     assert!(compare(&vec!['#'], &vec!['#']));
    //     assert!(!compare(&vec!['.'], &vec!['#']));
    //     assert!(!compare(&vec!['#'], &vec!['.']));
    // }

    // #[test]
    // fn test2() {
    //     assert!(compare(&vec!['#', '?', '.'], &vec!['#', '.', '.']));
    //     assert!(compare(&vec!['#', '?', '#'], &vec!['#', '.', '#']));

    //     assert!(!compare(&vec!['#', '?', '.'], &vec!['#', '.', '#']));
    //     assert!(!compare(&vec!['.', '?', '.'], &vec!['#', '.', '.']));

    //     assert!(!compare(&vec!['?', '#'], &vec!['.', '#', '.']));

    //     assert!(!compare(&vec!['?', '#', '.'], &vec!['.', '#']));
    // }
}
