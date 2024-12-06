use miette::{Diagnostic, SourceSpan};
use thiserror::Error;
use winnow::stream::AsBStr;

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

impl<I> From<winnow::error::ParseError<I, winnow::error::ContextError>> for AdventError
where
    I: AsBStr,
{
    fn from(value: winnow::error::ParseError<I, winnow::error::ContextError>) -> Self {
        let message = value.inner().to_string();
        let input = String::from_utf8_lossy(value.input().as_bstr()).into_owned();
        // Map the byte index into a char index
        // This assumes the offset is a byte-offset regardless of the input
        // type (u8, chars, tokens, ...). This is what the Display impl for
        // ParserError in winnow does.
        let start = input
            .char_indices()
            .find(|(i, _)| *i > value.offset())
            .map(|(i, _)| if i == value.offset() { i } else { i - 1 })
            .unwrap_or_else(|| input.len());
        value.offset();
        let end = (start + 1).min(input.len());

        Self::ParseError {
            message,
            span: (start..end).into(),
            input,
        }
    }
}
