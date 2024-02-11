use std::{cmp::Ordering, collections::HashMap};

pub mod aocerror;
pub mod progress;
pub use aocerror::*;
use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::{
    error::Rich,
    extra,
    primitive::*,
    text::{self},
    IterParser, Parser,
};

#[derive(Debug, Clone, Copy)]
pub enum WorkflowRuleField {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone, Copy)]
pub enum WorkflowRuleTarget<'a> {
    Accepted,
    Rejected,
    Next(&'a str),
}

#[derive(Debug, Clone, Copy)]
pub struct WorkflowRuleCmp {
    pub field: WorkflowRuleField,
    pub cmp: Ordering,
    pub value: i64,
}

#[derive(Debug, Clone, Copy)]
pub struct WorkflowRule<'a> {
    pub cmp: Option<WorkflowRuleCmp>,
    pub target: WorkflowRuleTarget<'a>,
}

pub struct Workflow<'a> {
    pub name: &'a str,
    pub rules: Vec<WorkflowRule<'a>>,
}

#[derive(Debug, Clone, Copy)]
pub struct Part {
    pub x: i64,
    pub m: i64,
    pub a: i64,
    pub s: i64,
}

pub fn parse<'a>(
    input: &'a str,
) -> Result<(HashMap<&'a str, Vec<WorkflowRule<'a>>>, Vec<Part>), AocError> {
    let document = parser().padded().then_ignore(end());

    match document.parse(input).into_result() {
        Ok(data) => Ok(data),
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

fn parser<'a>() -> impl Parser<
    'a,
    &'a str,
    (HashMap<&'a str, Vec<WorkflowRule<'a>>>, Vec<Part>),
    extra::Err<Rich<'a, char>>,
> {
    workflow_list()
        .then_ignore(text::newline().repeated())
        .then(part_list())
}

fn workflow_list<'a>(
) -> impl Parser<'a, &'a str, HashMap<&'a str, Vec<WorkflowRule<'a>>>, extra::Err<Rich<'a, char>>> {
    workflow().separated_by(text::newline()).collect()
}

fn workflow<'a>(
) -> impl Parser<'a, &'a str, (&'a str, Vec<WorkflowRule<'a>>), extra::Err<Rich<'a, char>>> {
    let rules = rule().separated_by(just(',')).collect();

    ident().then(rules.delimited_by(just('{'), just('}')))
}

fn rule<'a>() -> impl Parser<'a, &'a str, WorkflowRule<'a>, extra::Err<Rich<'a, char>>> {
    rule_cmp()
        .then_ignore(just(':'))
        .or_not()
        .then(target())
        //.then_ignore(text::newline)
        .map(|(cmp, target)| WorkflowRule { cmp, target })
}

fn rule_cmp<'a>() -> impl Parser<'a, &'a str, WorkflowRuleCmp, extra::Err<Rich<'a, char>>> {
    let field = choice((
        just('x').to(WorkflowRuleField::X),
        just('m').to(WorkflowRuleField::M),
        just('a').to(WorkflowRuleField::A),
        just('s').to(WorkflowRuleField::S),
    ));
    let cmp_op = choice((
        just('<').to(Ordering::Less),
        just('>').to(Ordering::Greater),
    ));

    field
        .then(cmp_op)
        .then(int_i64())
        .map(|((field, cmp), value)| WorkflowRuleCmp { field, cmp, value })
}

fn part_list<'a>() -> impl Parser<'a, &'a str, Vec<Part>, extra::Err<Rich<'a, char>>> {
    single_part().separated_by(text::newline()).collect()
}
fn single_part<'a>() -> impl Parser<'a, &'a str, Part, extra::Err<Rich<'a, char>>> {
    // {x=577,m=823,a=2129,s=23}
    field_part('x')
        .then_ignore(just(','))
        .then(field_part('m'))
        .then_ignore(just(','))
        .then(field_part('a'))
        .then_ignore(just(','))
        .then(field_part('s'))
        .delimited_by(just('{'), just('}'))
        .map(|(((x, m), a), s)| Part { x, m, a, s })
}

fn field_part<'a>(field_id: char) -> impl Parser<'a, &'a str, i64, extra::Err<Rich<'a, char>>> {
    // x=577
    just(field_id).ignore_then(just('=')).ignore_then(int_i64())
}

fn target<'a>() -> impl Parser<'a, &'a str, WorkflowRuleTarget<'a>, extra::Err<Rich<'a, char>>> {
    choice((
        just('A').to(WorkflowRuleTarget::Accepted),
        just('R').to(WorkflowRuleTarget::Rejected),
        ident().map(|target| WorkflowRuleTarget::Next(target)),
    ))
}

fn int_i64<'a>() -> impl Parser<'a, &'a str, i64, extra::Err<Rich<'a, char>>> {
    text::int(10).map(|s: &str| s.parse::<i64>().unwrap())
}

fn ident<'a>() -> impl Parser<'a, &'a str, &'a str, extra::Err<Rich<'a, char>>> {
    any()
        .filter(|c: &char| c.is_alphanumeric())
        .repeated()
        .at_least(1)
        .to_slice()
}
