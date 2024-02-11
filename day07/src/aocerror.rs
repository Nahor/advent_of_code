use std::{
    error::Error,
    num::{ParseFloatError, ParseIntError},
};

use miette::{Diagnostic, MietteSpanContents, SourceCode, SourceSpan};
use nom::error::{ContextError, ErrorKind, FromExternalError, ParseError};
use thiserror::Error;

#[derive(Debug)]
pub struct AocSourceChunk {
    chunk: String,
    start_line: usize,
}
impl AocSourceChunk {
    pub fn new(chunk: String, start_line: usize) -> Self {
        Self { chunk, start_line }
    }
}

impl SourceCode for AocSourceChunk {
    fn read_span<'a>(
        &'a self,
        span: &SourceSpan,
        _context_lines_before: usize,
        _context_lines_after: usize,
    ) -> Result<Box<dyn miette::SpanContents<'a> + 'a>, miette::MietteError> {
        let content = MietteSpanContents::new_named(
            "<input>".to_owned(),
            &self.chunk.as_bytes(),
            (0, self.chunk.len()).into(),
            self.start_line,
            span.offset(),
            1,
        );
        Ok(Box::new(content))
    }
}

pub fn aoc_error_span(line: &str, chunk: &str) -> SourceSpan {
    (
        unsafe { chunk.as_ptr().offset_from(line.as_ptr()) } as usize,
        chunk.len(),
    )
        .into()
}

#[derive(Error, Debug, Diagnostic)]
pub enum AocError {
    #[error("Invalid line")]
    #[diagnostic(
        code(input::bad_line),
        //url(docsrs),
        help("check the input data")
    )]
    InputError {
        #[source_code]
        src: AocSourceChunk,
        // Snippets and highlights can be included in the diagnostic!
        //#[label("This bit here")]
        bad_bit: SourceSpan,
        #[source]
        inner: Option<Box<dyn Error + Send + Sync>>,
    },
    #[error("missing prefix 'Game '")]
    InvalidPrefix,
    #[error("no data")]
    NoData,
    // #[error("invalid id")]
    // InvalidGameId {
    //     #[label("Expect u32")]
    //     span: SourceSpan,
    //     #[source]
    //     inner: Option<Box<dyn Error + Send + Sync>>,
    // },
    #[error("invalid color value in '{comp}'")]
    InvalidColorCount {
        comp: String,
        #[source]
        inner: Option<Box<dyn Error + Send + Sync>>,
    },
    #[error("invalid game id")]
    InvalidGameId {
        #[source_code]
        src: AocSourceChunk,
        #[label("expected u32 here")]
        span: SourceSpan,
        #[source]
        inner: Option<Box<dyn Error + Send + Sync>>,
    },
    #[error("duplicate part number '{entry}'")]
    DuplicateEntry { entry: u32 },
    #[error("missing ':' to split header from numbers")]
    NoHeaderNumbers {
        #[source_code]
        src: AocSourceChunk,
    },
    #[error(r#"missing '|' to split winning numbers from "have" numbers"#)]
    NoWinningHave {
        #[source_code]
        src: AocSourceChunk,
        #[label("missing '|' here")]
        span: SourceSpan,
    },
    #[error("missing header prefix ('Card')")]
    NoHeader {
        #[source_code]
        src: AocSourceChunk,
        #[label("missing 'Card' prefix")]
        span: SourceSpan,
    },
    #[error("invalid number")]
    InvalidNumber {
        #[source_code]
        src: AocSourceChunk,
        #[label("expected u32 here")]
        span: SourceSpan,
        #[source]
        inner: Option<Box<dyn Error + Send + Sync>>,
    },
    #[error("invalid input")]
    ParseError {
        //#[source]
        //inner: VerboseError<&str>,
        #[source_code]
        input: String,

        /// Offset in chars of the error.
        #[label("{}", label.unwrap_or("here"))]
        span: SourceSpan,

        /// Label text for this span. Defaults to `"here"`.
        label: Option<&'static str>,

        /// Suggestion for fixing the parser error.
        #[help]
        help: Option<&'static str>,

        /// Specific error kind for this parser error.
        kind: AocErrorKind,
    },
}

/// A type reprenting additional information specific to the type of error being returned.
#[derive(Debug, Diagnostic, Clone, Eq, PartialEq, Error)]
pub enum AocErrorKind {
    /// An error occurred while parsing an integer.
    #[error(transparent)]
    #[diagnostic(code(kdl::parse_int))]
    ParseIntError(ParseIntError),

    /// An error occurred while parsing a floating point number.
    #[error(transparent)]
    #[diagnostic(code(kdl::parse_float))]
    ParseFloatError(ParseFloatError),

    /// Generic parsing error. The given context string denotes the component
    /// that failed to parse.
    #[error("Expected {0}.")]
    #[diagnostic(code(kdl::parse_component))]
    Context(&'static str),

    /// Generic unspecified error. If this is returned, the call site should
    /// be annotated with context, if possible.
    #[error("An unspecified error occurred.")]
    #[diagnostic(code(kdl::other))]
    Other,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AocParseError<I> {
    pub input: I,
    pub context: Option<&'static str>,
    pub len: usize,
    pub label: Option<&'static str>,
    pub help: Option<&'static str>,
    pub kind: Option<AocErrorKind>,
    pub touched: bool,
}

impl<I> ParseError<I> for AocParseError<I> {
    fn from_error_kind(input: I, _kind: nom::error::ErrorKind) -> Self {
        Self {
            input,
            len: 0,
            label: None,
            help: None,
            context: None,
            kind: None,
            touched: false,
        }
    }

    fn append(_input: I, _kind: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

impl<I> ContextError<I> for AocParseError<I> {
    fn add_context(_input: I, ctx: &'static str, mut other: Self) -> Self {
        other.context = other.context.or(Some(ctx));
        other
    }
}

impl<'a> FromExternalError<&'a str, ParseIntError> for AocParseError<&'a str> {
    fn from_external_error(input: &'a str, _kind: ErrorKind, e: ParseIntError) -> Self {
        AocParseError {
            input,
            len: 0,
            label: None,
            help: None,
            context: None,
            kind: Some(AocErrorKind::ParseIntError(e)),
            touched: false,
        }
    }
}

impl<'a> FromExternalError<&'a str, ParseFloatError> for AocParseError<&'a str> {
    fn from_external_error(input: &'a str, _kind: ErrorKind, e: ParseFloatError) -> Self {
        AocParseError {
            input,
            len: 0,
            label: None,
            help: None,
            context: None,
            kind: Some(AocErrorKind::ParseFloatError(e)),
            touched: false,
        }
    }
}

/// Creates a span for an item using a substring of self.full_input
///
/// Note that substr must be a literal substring, as in it must be
/// a pointer into the same string!
pub fn span_from_substr(input: &str, substr: &str) -> SourceSpan {
    let base_addr = input.as_ptr() as usize;
    let substr_addr = substr.as_ptr() as usize;
    assert!(
        substr_addr >= base_addr,
        "tried to get the span of a non-substring!"
    );
    let start = substr_addr - base_addr;
    let end = start + substr.len();
    SourceSpan::from(start..end)
}
