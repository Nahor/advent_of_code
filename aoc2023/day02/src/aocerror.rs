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
            self.chunk.as_bytes(),
            (0, self.chunk.len()).into(),
            self.start_line,
            span.offset(),
            1,
        );
        Ok(Box::new(content))
    }
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
        _src: AocSourceChunk,
        // Snippets and highlights can be included in the diagnostic!
        #[label("This bit here")]
        _bad_bit: SourceSpan,
        #[source]
        _inner: Option<Box<dyn Error + Send + Sync>>,
    },
    #[error("missing prefix 'Game '")]
    InvalidPrefix,
    #[error("no data")]
    NoData,
    #[error("invalid id")]
    InvalidGameId {
        #[label("Expect u32")]
        _span: SourceSpan,
        #[source]
        _inner: Option<Box<dyn Error + Send + Sync>>,
    },
    #[error("invalid color format in '{comp}'")]
    InvalidComponent { comp: String },
    #[error("invalid color value in '{comp}'")]
    InvalidColorCount {
        comp: String,
        #[source]
        _inner: Option<Box<dyn Error + Send + Sync>>,
    },
    #[error("invalid color name in '{comp}'")]
    InvalidColorName { comp: String },
}
