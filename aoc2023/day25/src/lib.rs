use std::{
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
    ops::{Add, Mul, Range},
};

pub mod aocerror;
pub mod progress;
pub use aocerror::*;
use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::{
    prelude::*,
    text::{ascii::ident, inline_whitespace, newline},
};
use num::{One, ToPrimitive};

pub trait MinMax<T> {
    type Output;
    fn minmax(&self, other: T) -> Self::Output;
}
impl MinMax<isize> for isize {
    type Output = (Self, Self);
    fn minmax(&self, other: isize) -> Self::Output {
        if *self <= other {
            (*self, other)
        } else {
            (other, *self)
        }
    }
}
impl<T: num::Num + PartialOrd + Clone> MinMax<T> for (T, T) {
    type Output = Self;
    fn minmax(&self, other: T) -> Self::Output {
        if other < self.0 {
            (other, self.1.clone())
        } else if self.1 < other {
            (self.0.clone(), other)
        } else {
            self.clone()
        }
    }
}
impl<T> MinMax<T> for Range<T>
where
    T: Ord + Add<T, Output = T> + One + Clone,
{
    type Output = Self;
    fn minmax(&self, other: T) -> Self::Output {
        if other < self.start {
            other..self.end.clone()
        } else if self.end <= other {
            (self.start.clone())..(other + T::one())
        } else {
            self.clone()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
pub struct Coord<T: num::Num> {
    pub x: T,
    pub y: T,
    pub z: T,
}
impl<T> Coord<T>
where
    T: num::Num + num::NumCast,
{
    pub fn new<U>(x: U, y: U, z: U) -> Self
    where
        U: ToPrimitive,
    {
        Self {
            x: T::from(x).unwrap(),
            y: T::from(y).unwrap(),
            z: T::from(z).unwrap(),
        }
    }
}
impl<T: num::Num> Add<Self> for Coord<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl<T: num::Num + Clone> Mul<T> for Coord<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Coord {
            x: self.x * rhs.clone(),
            y: self.y * rhs.clone(),
            z: self.z * rhs.clone(),
        }
    }
}

pub type CoordType = f64;
pub type F64Coord = Coord<CoordType>;

#[derive(Debug, Clone, Copy, Default)]
pub struct Stone {
    pub position: F64Coord,
    pub velocity: F64Coord,
}

pub fn parse(input: &str) -> Result<HashMap<&'_ str, Vec<&'_ str>>, AocError<'_>> {
    let document = diagram_parser().padded().then_ignore(end());

    let stones = match document.parse(input).into_result() {
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
    }?;

    Ok(stones)
}

fn diagram_parser<'a>()
-> impl Parser<'a, &'a str, HashMap<&'a str, Vec<&'a str>>, extra::Err<Rich<'a, char>>> {
    component_parser()
        .separated_by(newline().padded_inline())
        .collect()
}

fn component_parser<'a>()
-> impl Parser<'a, &'a str, (&'a str, Vec<&'a str>), extra::Err<Rich<'a, char>>> {
    text::ident()
        .then_ignore(just(':').padded_inline())
        .then(ident().separated_by(inline_whitespace()).collect())
}

// Trait to add the function `padded_inline` to any parser
pub trait InlinePadding<'a, C, I, O, E>
where
    Self: Parser<'a, I, O, E>,
    C: text::Char,
    I: chumsky::input::StrInput<'a, C>,
    I::Token: text::Char,
    E: extra::ParserExtra<'a, I>,
{
    fn padded_inline(self) -> impl Parser<'a, I, O, E>
    where
        Self: Sized,
    {
        self.padded_by(inline_whitespace())
    }
}
impl<'a, C, I, O, E, P> InlinePadding<'a, C, I, O, E> for P
where
    P: Parser<'a, I, O, E>,
    C: text::Char,
    I: chumsky::input::StrInput<'a, C>,
    I::Token: text::Char,
    E: extra::ParserExtra<'a, I>,
{
}
