use miette::{Diagnostic, MietteDiagnostic, SourceSpan};
use thiserror::Error;
use winnow::stream::{AsBStr, Location};

#[derive(Debug, Error, Diagnostic)]
#[error("error")]
pub enum AdventError {
    #[error("error")]
    #[diagnostic(code(aoc::generic))]
    GenericStr(#[help] &'static str),

    #[error("error")]
    #[diagnostic(code(aoc::generic))]
    GenericString(#[help] String),

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

    // Using `_xxx` otherwise Rust 1.92 warns about assigned-but-unread fields
    // and even using `#[allow(<unused_assignments>)]` does not fix it
    #[error("parsing error")]
    #[diagnostic(code(aoc::parse_error))]
    ParseError {
        //#[help] (if we want a separate error message, but for now, use it for the span's label instead)
        message: String,
        #[label("{message}")]
        span: SourceSpan,
        #[source_code]
        input: String,
    },

    #[error("parsing int error")]
    #[diagnostic(code(aoc::parse_int_error))]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("error")]
    #[diagnostic(code(aoc::unknown_error))]
    Error(#[from] Box<MietteDiagnostic>),
}

impl AdventError {
    pub fn parse_error_u8(input: &[u8], index: usize, message: String) -> Self {
        let input = String::from_utf8_lossy(input).into_owned();
        let start = input
            .char_indices()
            .enumerate()
            .find(|(_, (byte_idx, _))| *byte_idx > index)
            .map(|(char_idx, _)| char_idx - 1)
            .unwrap_or_else(|| {
                // Distinguish between an error on the last char and on eof
                // (both would fail to find an index since we search with '>')
                if index < input.len() {
                    input.len() - 1
                } else {
                    input.len()
                }
            });
        let end = (start + 1).min(input.len());

        Self::ParseError {
            message: message,
            span: (start..end).into(),
            input: input,
        }
    }
}

impl<I> From<winnow::error::ParseError<I, winnow::error::ContextError>> for AdventError
where
    I: AsBStr,
{
    fn from(value: winnow::error::ParseError<I, winnow::error::ContextError>) -> Self {
        Self::parse_error_u8(
            value.input().as_bstr(),
            value.offset(),
            value.inner().to_string(),
        )
    }
}

impl<I> From<winnow::error::ParseError<I, AdventError>> for AdventError
where
    I: AsBStr,
{
    fn from(value: winnow::error::ParseError<I, AdventError>) -> Self {
        value.into_inner()
    }
}

impl From<&'static str> for AdventError {
    fn from(err: &'static str) -> Self {
        Self::GenericStr(err)
    }
}

impl From<String> for AdventError {
    fn from(err: String) -> Self {
        Self::GenericString(err)
    }
}

impl<I> winnow::error::ParserError<winnow::LocatingSlice<I>> for AdventError
where
    I: AsBStr + winnow::stream::Stream + Clone + winnow::stream::Offset,
{
    type Inner = Self;

    fn from_input(input: &winnow::LocatingSlice<I>) -> Self {
        let offset = input.current_token_start();
        let mut input = input.clone();
        input.reset_to_start();
        Self::ParseError {
            message: "here".to_string(),
            span: (offset..=offset).into(),
            input: String::from_utf8_lossy(input.as_bstr()).into_owned(),
        }
    }

    fn into_inner(self) -> Result<Self::Inner, Self> {
        Ok(self)
    }
}

impl<I> winnow::error::AddContext<winnow::LocatingSlice<I>, &'static str> for AdventError
where
    I: AsBStr + winnow::stream::Stream + Clone + winnow::stream::Offset,
{
    fn add_context(
        self,
        _input: &winnow::LocatingSlice<I>,
        _token_start: &<winnow::LocatingSlice<I> as winnow::stream::Stream>::Checkpoint,
        _context: &'static str,
    ) -> Self {
        match self {
            AdventError::ParseError {
                message: _message,
                span,
                input,
            } => Self::ParseError {
                message: _context.to_string(),
                span,
                input,
            },
            AdventError::GenericStr(_)
            | AdventError::GenericString(_)
            | AdventError::FileError(_)
            | AdventError::ParseIntError(_)
            | AdventError::Error(_) => unreachable!(),
        }
    }
}
