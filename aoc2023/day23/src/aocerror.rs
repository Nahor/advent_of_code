use std::{
    error::Error,
    num::{ParseFloatError, ParseIntError},
};

use miette::{Diagnostic, MietteSpanContents, SourceCode, SourceSpan};
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub struct AocSourceChunk<'a> {
    chunk: &'a str,
    start_line: usize,
}
impl<'a> AocSourceChunk<'a> {
    pub fn new(chunk: &'a str, start_line: usize) -> Self {
        Self { chunk, start_line }
    }
}

impl SourceCode for AocSourceChunk<'_> {
    fn read_span<'a>(
        &'a self,
        span: &SourceSpan,
        _context_lines_before: usize,
        _context_lines_after: usize,
    ) -> Result<Box<dyn miette::SpanContents<'a> + 'a>, miette::MietteError> {
        let content = MietteSpanContents::new_named(
            "<input>".to_owned(),
            self.chunk.as_bytes(),
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
pub enum AocError<'a> {
    #[error("Invalid document: {desc}")]
    #[diagnostic(
        code(input::bad_document),
        //url(docsrs),
        //help("check the input data")
    )]
    InvalidDocumentError { desc: String },
    #[error("Invalid line: {desc}")]
    #[diagnostic(
        code(input::bad_line),
        //url(docsrs),
        //help("check the input data")
    )]
    GenericParseError {
        desc: String,
        #[source_code]
        src: AocSourceChunk<'a>,
        #[label("here")]
        span: SourceSpan,
        #[source]
        inner: Option<Box<dyn Error + Send + Sync>>,
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
pub struct AocParseError {
    pub input: String,
    pub line: usize,
    pub col: usize,
    pub len: usize,
    pub label: Option<&'static str>,
    pub help: Option<&'static str>,
    pub context: Option<&'static str>,
    pub kind: Option<AocErrorKind>,
}
