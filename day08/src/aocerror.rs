use std::error::Error;

use miette::{Diagnostic, MietteSpanContents, SourceCode, SourceSpan};
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
    InvalidLineError {
        desc: String,
        #[source_code]
        src: AocSourceChunk,
        #[label("here")]
        span: SourceSpan,
        #[source]
        inner: Option<Box<dyn Error + Send + Sync>>,
    },
}
