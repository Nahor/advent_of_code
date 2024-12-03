use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("error")]
pub enum AdventError {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    FileError(#[from] std::io::Error),
    // #[error("the data for key `{0}` is not available")]
    // Redaction(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader {
    //     expected: String,
    //     found: String,
    // },
    #[error("parsing error")]
    #[diagnostic(code(aoc::parse_error))]
    ParseError {
        message: String,
        #[label("here")]
        span: SourceSpan,
        #[source_code]
        input: String,
    },
    #[error("parsing int error")]
    #[diagnostic(code(aoc::parse_error))]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("unknown error")]
    #[diagnostic(code(aoc::unknown_error))]
    Unknown,
}

impl From<winnow::error::ParseError<&str, winnow::error::ContextError>> for AdventError {
    fn from(value: winnow::error::ParseError<&str, winnow::error::ContextError>) -> Self {
        let message = value.inner().to_string();
        let input = value.input().to_owned();
        let start = value.offset();
        // Assume the error span is only for the first `char`.
        // Semantic errors are free to choose the entire span returned by `Parser::with_span`.
        let end = (start + 1..)
            .find(|e| input.is_char_boundary(*e))
            .unwrap_or(start);
        Self::ParseError {
            message,
            span: (start..end).into(),
            input: input.to_owned(),
        }
    }
}
