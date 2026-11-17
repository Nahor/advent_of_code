use common::error::AdventError;
use winnow::{
    LocatingSlice,
    ascii::{dec_uint, line_ending},
    combinator::{alt, delimited, eof, repeat_till, terminated, trace},
    error::ContextError,
    prelude::*,
    token::take_while,
};

type ParseInput<'a> = LocatingSlice<&'a [u8]>;
//type ParseResult<T> = ModalResult<T, AdventError>;
type ParseResult<T> = ModalResult<T, ContextError>;

pub fn parse(content: &[u8]) -> Result<Vec<(u64, u64, u64, u64)>, AdventError> {
    let input = LocatingSlice::new(content);
    Ok(trace(
        "parser",
        // `repeat_till(... eof)`, because if there is an error, `repeat` will just assume it was the end of
        // the repetition and backtrack to the beginning of the line, and let the next parser continue from there (or
        // fail there is no more parsers)
        // Alternative: use `cut_err` to force any error in `parse_line` to abort parsing
        repeat_till(1.., terminated(parse_line, alt((line_ending, eof))), eof).map(|(v, _eof)| v),
    )
    .parse(input)?)
}

fn parse_line(input: &mut ParseInput) -> ParseResult<(u64, u64, u64, u64)> {
    trace(
        "parse_line",
        (parse_number, parse_number, parse_number, parse_number),
    )
    .parse_next(input)
}

fn parse_number(input: &mut ParseInput) -> ParseResult<u64> {
    trace(
        "parse_number",
        delimited(
            whitespace0,
            dec_uint.context(winnow::error::StrContext::Expected(
                winnow::error::StrContextValue::Description("number"),
            )),
            whitespace0,
        ),
    )
    .parse_next(input)
}

fn whitespace0(input: &mut ParseInput) -> ParseResult<()> {
    trace("whitespace", take_while(0.., [' ', '\t']).void()).parse_next(input)
}
