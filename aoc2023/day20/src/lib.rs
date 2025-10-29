use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
};

pub mod aocerror;
pub mod progress;
pub use aocerror::*;
use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::{
    error::Rich,
    extra,
    primitive::*,
    text::{self, newline},
    IterParser, Parser,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Pulse {
    High,
    Low,
}

#[derive(Debug, Hash, Clone)]
pub struct FlipFlop {
    pub is_on: bool,
}
impl FlipFlop {
    fn new() -> Self {
        FlipFlop { is_on: false }
    }
    pub fn signal(&mut self, pulse: Pulse) -> Option<Pulse> {
        match pulse {
            Pulse::High => None,
            Pulse::Low => {
                if self.is_on {
                    self.is_on = false;
                    Some(Pulse::Low)
                } else {
                    self.is_on = true;
                    Some(Pulse::High)
                }
            }
        }
    }
}

#[derive(Debug, Hash, Clone)]
pub struct Conjunction<'a> {
    pub inputs: BTreeMap<&'a str, Pulse>,
}
impl<'a> Conjunction<'a> {
    fn new() -> Self {
        Conjunction {
            inputs: BTreeMap::new(),
        }
    }
    pub fn signal(&mut self, from: &'a str, pulse: Pulse) -> Option<Pulse> {
        *self.inputs.get_mut(&from).unwrap() = pulse;
        self.inputs
            .iter()
            .all(|(_, &last_pulse)| last_pulse == Pulse::High)
            .then_some(Pulse::Low)
            .or(Some(Pulse::High))
    }
    fn add_input(&mut self, input: &'a str) {
        if self.inputs.insert(input, Pulse::Low).is_some() {
            panic!("multiple entries for {input}");
        }
    }
}

#[derive(Debug, Hash, Clone)]
pub struct Broadcaster {}
impl Broadcaster {
    fn new() -> Self {
        Broadcaster {}
    }
    pub fn signal(&mut self, pulse: Pulse) -> Option<Pulse> {
        Some(pulse)
    }
}

#[derive(Debug, Hash, Clone)]
pub enum Module<'a> {
    Broadcaster(Broadcaster),
    FlipFlop(FlipFlop),
    Conjunction(Conjunction<'a>),
}

//////////////
//////////////
//////////////
//////////////
//////////////

pub fn parse<'a>(input: &'a str) -> Result<HashMap<&'a str, (Module<'a>, Vec<&'a str>)>, AocError> {
    let document = parser().padded().then_ignore(end());

    match document.parse(input).into_result() {
        Ok(mut data) => {
            // Update conjunctions with their inputs
            // First get the list of input names for each conjunction...
            let mut conjunction_inputs = HashMap::<&'a str, Vec<&'a str>>::new();
            data.iter().for_each(|(k, (_, outs))| {
                outs.iter().for_each(|name| {
                    if let Some((Module::Conjunction(_), _)) = data.get(name) {
                        conjunction_inputs.entry(name).or_default().push(k);
                    }
                });
            });
            // ... Then update the conjunctions
            conjunction_inputs.into_iter().for_each(|(name, vec)| {
                let module = match data.get_mut(name).unwrap().0 {
                    Module::Conjunction(ref mut module) => module,
                    _ => return,
                };
                vec.into_iter().for_each(|name| module.add_input(name));
            });
            Ok(data)
        }
        Err(errs) => {
            for err in errs {
                Report::build(ReportKind::Error, (), err.span().start)
                    .with_code(3)
                    .with_message(err.to_string())
                    .with_label(
                        Label::new(err.span().into_range())
                            .with_message(err.reason().to_string())
                            .with_color(Color::Red),
                    )
                    .finish()
                    .eprint(Source::from(input))
                    .unwrap();
            }
            Err(AocError::InvalidDocumentError {
                desc: "parser error".to_owned(),
            })
        }
    }
}

fn parser<'a>(
) -> impl Parser<'a, &'a str, HashMap<&'a str, (Module<'a>, Vec<&'a str>)>, extra::Err<Rich<'a, char>>>
{
    module().separated_by(newline()).collect()
}

fn module<'a>(
) -> impl Parser<'a, &'a str, (&'a str, (Module<'a>, Vec<&'a str>)), extra::Err<Rich<'a, char>>> {
    module_type()
        .then(module_name())
        .then_ignore(just("->").padded())
        .then(module_output_list())
        .map(|((mod_type, name), list)| (name, (mod_type, list)))
}

fn module_type<'a>() -> impl Parser<'a, &'a str, Module<'a>, extra::Err<Rich<'a, char>>> {
    choice((
        just('%').to(Module::FlipFlop(FlipFlop::new())),
        just('&').to(Module::Conjunction(Conjunction::new())),
        empty().to(Module::Broadcaster(Broadcaster::new())),
    ))
}

fn module_name<'a>() -> impl Parser<'a, &'a str, &'a str, extra::Err<Rich<'a, char>>> {
    text::ident()
}

fn module_output_list<'a>() -> impl Parser<'a, &'a str, Vec<&'a str>, extra::Err<Rich<'a, char>>> {
    text::ident().separated_by(just(',').padded()).collect()
}
