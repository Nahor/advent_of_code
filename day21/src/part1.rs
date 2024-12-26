use std::{
    cmp::Ordering::{Equal, Greater, Less},
    fmt::Display,
};

use itertools::Itertools;
use miette::Result;
use rustc_hash::FxHashMap;

use crate::parse::parse;

pub fn run(content: &[u8]) -> Result<u64> {
    let codes = parse(content)?;

    // This is not a simple "manhattan distance" because the directional keys
    // dont all have the same cost. If robot N must move left, robot N+1
    // will need to execute 4 moves to go from 'A' to '<' and push ("v<<A").
    // But if N moves right, N+1 only moves 2 ("vA"). Also, repeatedly moving
    // left only cost 1 extra move for each repeat (just "push"). So it is
    // better to move left once as much as possible than doing it in series.
    // So if robot N-1 needs to move in a diagonal left, there are two options:
    // "<^A" or "^<A", which translates to "v<<A >^A >A" and "<A v<A >>A" for
    // robot N, which have the same number of moves. But for robot N+1, the "<<"
    // vs "< + <" makes a big difference because the latter has multiple costly
    // moves to go to "<" then return to "A", while the former only does it once.

    // Also, every single action of robot N-1 translates to robot N moving to
    // another button (if necessary) and pushing it, which in turns translates
    // to a series of action for robot N+1 finishing with a move to 'A' and
    // pushing that. This means that after every action of N-1, N+1 is reset
    // to its starting position. This then means that robot N+2 does not matter
    // no do the actions taken by N-1 before or after.
    // All that matters is where N is at the start, where it ends at the end,
    // and how it does it. And that "how" is directly and only linked to the
    // number of moves needed by N+1.
    //
    // Only the human operator escapes those rules. For them, since they don't
    // have a "N+1", what matters is only the number of buttons to push, not
    // the order, i.e. only the manhattan distance of between the buttons
    // that the previous robot pushes.
    //
    // And there is only 25 different moves that a robot can do (except for the
    // one with the numeric keypad), so we can easily precompute
    // the complexity/cost of those moves for the last robot/ (`init_dir_cost_cache`),
    // and we'll call that depth 0.
    // This then allows us to compute the cost for the robot before that (depth 1),
    // and so on and so forth, down to the first robot (depth N).
    //
    // If the stack of robots is deep, the total complexity will grow
    // exponentially, however, each depth is capped at 25 different moves. So
    // caching the result at each depth will be very efficient, especially for
    // the low ones.
    let mut depth_cache = init_dir_cost_cache();

    let result: usize = codes
        .iter()
        .map(|code| get_code_length(code, 1, &mut depth_cache))
        .sum();

    Ok(result as u64)
}

pub fn get_code_length(code: &[u8], depth: usize, depth_cache: &mut DepthCache) -> usize {
    let val: usize = code[0..(code.len() - 1)]
        .iter()
        .fold(0, |acc, b| acc * 10 + (b - b'0') as usize);

    let sequence = code
        .iter()
        .scan(b'A', |prev, &cur| {
            let tmp = *prev;
            *prev = cur;
            Some((tmp, cur))
        })
        .map(|(from, to)| get_min_moves(from, to, depth, depth_cache))
        .collect::<Vec<_>>();
    let cost = sequence.iter().map(|(cost, _)| cost).sum::<usize>();
    cost * val
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Activate,
}
impl Action {
    const LIST: [Self; 5] = [
        Self::Up,
        Self::Down,
        Self::Left,
        Self::Right,
        Self::Activate,
    ];

    #[inline]
    fn to_char(self) -> char {
        match self {
            Self::Up => '^',
            Self::Down => 'v',
            Self::Left => '<',
            Self::Right => '>',
            Self::Activate => 'A',
        }
    }
}
impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

fn get_base_moves<T>(from: T, to: T) -> Vec<Action>
where
    T: MapPos,
{
    let from_pos = from.map_pos();
    let to_pos = to.map_pos();

    // Because of the invalid position, we need to move down before moving left
    // and move right before moving up.
    let mut moves = Vec::new();
    let vertical_moves = match to_pos.1.cmp(&from_pos.1) {
        Less => vec![Action::Up; to_pos.1.abs_diff(from_pos.1) as usize],
        Equal => vec![],
        Greater => vec![Action::Down; to_pos.1.abs_diff(from_pos.1) as usize],
    };
    match to_pos.0.cmp(&from_pos.0) {
        Less => {
            moves.extend(vertical_moves);
            moves.extend(vec![Action::Left; to_pos.0.abs_diff(from_pos.0) as usize]);
        }
        Equal => {
            // no horizontal move
            moves.extend(vertical_moves);
        }
        Greater => {
            moves.extend(vec![Action::Right; to_pos.0.abs_diff(from_pos.0) as usize]);
            moves.extend(vertical_moves);
        }
    }
    moves
}

// key: from, to, depth
// value: total cost/complexity, moves (for depth-1)
type DepthCache = FxHashMap<(Action, Action, usize), (usize, Vec<Action>)>;
pub fn init_dir_cost_cache() -> DepthCache {
    // For the final depth (=0), the moves order doesn't matter (other than
    // being valid)

    Action::LIST
        .iter()
        .copied()
        .cartesian_product(Action::LIST.iter().copied())
        .map(|(from, to)| {
            let mut moves = get_base_moves(from, to);
            moves.push(Action::Activate);
            ((from, to, 0), (moves.len(), moves))
        })
        .collect()
}

fn get_min_moves<T>(
    from: T,
    to: T,
    depth: usize,
    depth_cache: &mut DepthCache,
) -> (usize, std::vec::Vec<Action>)
where
    T: MapPos,
{
    let moves = get_base_moves(from, to);
    let cost_moves = moves
        .iter()
        .permutations(moves.len())
        .filter_map(|seq| {
            // Convert from a `Vec<&T>` to a `Vec<T>`
            let mut moves = seq.into_iter().cloned().collect::<Vec<_>>();
            if !T::is_valid_sequence(from, to, &moves) {
                return None;
            }

            // Then add the finishing activation key
            moves.push(Action::ACTIVATE_VAL);
            Some(moves)
        })
        .map(|seq| {
            let cost = seq
                .iter()
                .copied()
                .scan(Action::ACTIVATE_VAL, |prev, cur| {
                    let tmp = *prev;
                    *prev = cur;
                    Some((tmp, cur))
                })
                .map(|(from, to)| {
                    if let Some((cost, _)) = depth_cache.get(&(from, to, depth)) {
                        *cost
                    } else {
                        let cost_moves = get_min_moves(from, to, depth - 1, depth_cache);
                        let cost = cost_moves.0;
                        depth_cache.insert((from, to, depth), cost_moves);
                        cost
                    }
                })
                .sum::<usize>();
            (cost, seq)
        })
        .min_by(|(cost_l, _), (cost_r, _)| cost_l.cmp(cost_r))
        .unwrap_or_else(|| {
            panic!(
                "should have a result for {}->{} at depth {depth}",
                from.to_char(),
                to.to_char()
            )
        });
    cost_moves
}

trait MapPos: Copy {
    const ACTIVATE_VAL: Self;
    fn map_pos(&self) -> (i8, i8);
    fn to_char(self) -> char;

    fn is_valid_sequence(from: Self, to: Self, seq: &[Action]) -> bool;
}
impl MapPos for u8 {
    const ACTIVATE_VAL: Self = b'A';
    fn map_pos(&self) -> (i8, i8) {
        static NUMERIC_MAP: [(i8, i8); 11] = [
            // in order: 0..9, A
            (1, 3),
            (0, 2),
            (1, 2),
            (2, 2),
            (0, 1),
            (1, 1),
            (2, 1),
            (0, 0),
            (1, 0),
            (2, 0),
            (2, 3),
        ];
        match self {
            b'0'..=b'9' => NUMERIC_MAP[(self - b'0') as usize],
            b'A' => NUMERIC_MAP[10],
            _ => unreachable!("invalid numeric character"),
        }
    }

    fn to_char(self) -> char {
        self as char
    }

    fn is_valid_sequence(from: Self, to: Self, seq: &[Action]) -> bool {
        let from_pos = from.map_pos();
        let to_pos = to.map_pos();
        if from_pos.1 == 3 && seq.starts_with(&vec![Action::Left; from_pos.0 as usize]) {
            return false;
        }
        if to_pos.1 == 3 && seq.ends_with(&vec![Action::Right; to_pos.0 as usize]) {
            return false;
        }
        true
    }
}
impl MapPos for Action {
    const ACTIVATE_VAL: Self = Action::Activate;
    fn map_pos(&self) -> (i8, i8) {
        match self {
            Self::Up => (1, 0),
            Self::Down => (1, 1),
            Self::Left => (0, 1),
            Self::Right => (2, 1),
            Self::Activate => (2, 0),
        }
    }

    fn to_char(self) -> char {
        (self as Action).to_char()
    }

    fn is_valid_sequence(from: Self, to: Self, seq: &[Action]) -> bool {
        let from_pos = from.map_pos();
        let to_pos = to.map_pos();
        if from_pos == (0, 1) && matches!(seq.first(), Some(Action::Up)) {
            return false;
        }

        if to_pos == (0, 1) && matches!(seq.last(), Some(Action::Down)) {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // cspell:disable
    const INPUT_SAMPLE: &[u8] = br#"
029A
980A
179A
456A
379A
"#;
    // cspell:enable

    #[test]
    fn sample() {
        let input = &INPUT_SAMPLE[1..]; // remove leading \n

        assert_eq!(run(input).unwrap(), 126384);
    }

    #[test]
    fn singles() {
        assert_eq!(run(b"029A").unwrap(), 68 * 29);
        assert_eq!(run(b"980A").unwrap(), 60 * 980);
        assert_eq!(run(b"179A").unwrap(), 68 * 179);
        assert_eq!(run(b"456A").unwrap(), 64 * 456);
        assert_eq!(run(b"379A").unwrap(), 64 * 379);
    }

    // #[test]
    // fn test_3_9() {
    //     let sequence = optimize_numeric(b'3', b'7');
    //     println!("v1: {}", DisplaySequence(&sequence));
    //     let sequence = sequence
    //         .iter()
    //         .scan(Action::Activate, |prev, &cur| {
    //             let tmp = *prev;
    //             *prev = cur;
    //             Some((tmp, cur))
    //         })
    //         .map(|(from, to)| get_dir_seq(from, to, 2))
    //         .collect::<Vec<_>>();
    //     println!("v2: {}", DisplaySequence(&sequence));
    //     let sequence = sequence
    //         .iter()
    //         .flatten()
    //         .scan(Action::Activate, |prev, &cur| {
    //             let tmp = *prev;
    //             *prev = cur;
    //             Some((tmp, cur))
    //         })
    //         .map(|(from, to)| get_dir_seq(from, to, 1))
    //         .collect::<Vec<_>>();
    //     println!("v3: {}", DisplaySequence(&sequence));
    // }

    // #[test]
    // fn compare_base() {
    //     assert_eq!(
    //         run(&INPUT_SAMPLE[1..]).unwrap(),
    //         crate::part1::run(&INPUT_SAMPLE[1..]).unwrap()
    //     );

    //     let input = common::read_input_u8!(None).unwrap();
    //     assert_eq!(run(&input).unwrap(), crate::part1::run(&input).unwrap());
    // }
}
