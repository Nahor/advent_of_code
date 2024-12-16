use crate::aocerror::{AocError, AocSourceChunk};

#[derive(Debug, Default, Clone)]
pub struct Game {
    pub id: u32,
    pub rounds: Vec<Set>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Set {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}
impl Set {
    pub fn new(s: &str) -> Result<Self, AocError> {
        Ok(s.split(",")
            .map(|comp| {
                let comp = comp.trim();
                let (val, name) =
                    comp.trim()
                        .split_once(' ')
                        .ok_or(AocError::InvalidComponent {
                            comp: comp.to_owned(),
                        })?;
                let val = val
                    .parse::<u32>()
                    .map_err(|err| AocError::InvalidColorCount {
                        comp: comp.to_owned(),
                        inner: Some(Box::new(err)),
                    })?;
                match name {
                    "red" => Ok(Set {
                        red: val,
                        green: 0,
                        blue: 0,
                    }),
                    "green" => Ok(Set {
                        red: 0,
                        green: val,
                        blue: 0,
                    }),
                    "blue" => Ok(Set {
                        red: 0,
                        green: 0,
                        blue: val,
                    }),
                    _ => Err(AocError::InvalidColorName {
                        comp: comp.to_owned(),
                    }),
                }
            })
            .try_fold(Set::default(), |acc, set| match set {
                Ok(set) => Ok(Set {
                    red: acc.red + set.red,
                    green: acc.green + set.green,
                    blue: acc.blue + set.blue,
                }),
                err => err,
            }))?
    }

    pub fn valid(&self, rhs: &Self) -> bool {
        self.red <= rhs.red && self.green <= rhs.green && self.blue <= rhs.blue
    }
}

pub fn parse(input: &str) -> Result<Vec<Game>, AocError> {
    input
        .lines()
        .enumerate()
        .map(|(lineno, line)| {
            process_game(line).map_err(|err| AocError::InputError {
                src: AocSourceChunk::new(line.to_owned(), lineno),
                bad_bit: (0, line.len()).into(),
                inner: Some(Box::new(err)),
            })
        })
        .collect()
}

fn process_game(line: &str) -> Result<Game, AocError> {
    let line = line.strip_prefix("Game ").ok_or(AocError::InvalidPrefix)?;
    let (id, data) = line.split_once(':').ok_or_else(|| AocError::NoData)?;
    let id = id.parse::<u32>().map_err(|err| AocError::InvalidGameId {
        span: (5, 5 + id.len()).into(),
        inner: Some(Box::new(err)),
    })?;
    let rounds = data
        .split(';')
        .map(Set::new)
        .collect::<Result<Vec<Set>, _>>()?;

    Ok(Game { id, rounds })
}
