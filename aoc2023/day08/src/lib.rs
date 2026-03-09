use std::collections::HashMap;

use aocerror::{AocError, AocSourceChunk};

pub mod aocerror;

#[derive(Debug, Clone, Copy)]
pub enum Step {
    Left,
    Right,
}

#[derive(Debug, Default)]
pub struct Node {
    pub left: String,
    pub right: String,
}

#[derive(Debug, Default)]
pub struct Data {
    pub steps: Vec<Step>,
    pub nodes: HashMap<String, Node>,
}

pub fn parse(input: &str) -> Result<Data, AocError> {
    let (steps_str, nodes_str) =
        input
            .split_once('\n')
            .ok_or_else(|| AocError::InvalidDocumentError {
                desc: "expected two or more lines (steps + graphs)".to_owned(),
            })?;

    let steps = steps_str
        .chars()
        .enumerate()
        .map(|(charno, c)| {
            Ok(match c {
                'L' => Step::Left,
                'R' => Step::Right,
                _ => {
                    return Err(AocError::InvalidLineError {
                        _desc: "invalid step char".to_owned(),
                        _src: AocSourceChunk::new(steps_str.to_owned(), 0),
                        _span: (charno, 1).into(),
                        _inner: None,
                    });
                }
            })
        })
        .collect::<Result<Vec<Step>, _>>()?;
    if steps.is_empty() {
        return Err(AocError::InvalidLineError {
            _desc: "no steps".to_owned(),
            _src: AocSourceChunk::new(steps_str.to_owned(), 0),
            _span: (0, steps_str.len()).into(),
            _inner: None,
        });
    }

    let mut nodes = HashMap::new();
    nodes_str
        .lines()
        .enumerate()
        .map(|(lineno, line)| (lineno + 1, line)) // +1 to account for the step line
        .filter(|(_, line)| !line.is_empty())
        .try_for_each(|(lineno, line)| {
            let (name, children) =
                line.split_once('=')
                    .ok_or_else(|| AocError::InvalidLineError {
                        _desc: "expected =".to_owned(),
                        _src: AocSourceChunk::new(line.to_owned(), lineno),
                        _span: (0, line.len()).into(),
                        _inner: None,
                    })?;
            let (left, right) =
                children
                    .split_once(',')
                    .ok_or_else(|| AocError::InvalidLineError {
                        _desc: "expected ,".to_owned(),
                        _src: AocSourceChunk::new(line.to_owned(), lineno),
                        _span: (name.len() + 1, children.len()).into(),
                        _inner: None,
                    })?;
            let name = name.trim().to_owned();
            let left = left
                .trim_matches(|c: char| c.is_whitespace() || c == '(')
                .to_owned();
            let right = right
                .trim_matches(|c: char| c.is_whitespace() || c == ')')
                .to_owned();

            //println!("Inserting node {name} with {left},{right}");
            if nodes.insert(name.clone(), Node { left, right }).is_some() {
                return Err(AocError::InvalidLineError {
                    _desc: format!("duplicate node {name}"),
                    _src: AocSourceChunk::new(line.to_owned(), lineno),
                    _span: (name.len() + 1, children.len()).into(),
                    _inner: None,
                });
            };

            Ok::<(), AocError>(())
        })?;
    //println!("Res: {res}");
    Ok(Data { steps, nodes })
}
