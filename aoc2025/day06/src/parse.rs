use common::error::AdventError;
use winnow::{
    ascii::{dec_uint, line_ending, space0, space1},
    combinator::{alt, delimited, eof, opt, repeat, separated, terminated, trace},
    error::ParserError,
    prelude::*,
    stream::{AsBStr, AsChar, Compare, Stream, StreamIsPartial},
};

use crate::Ops;

pub fn parse(content: &[u8]) -> Result<(Vec<Vec<u64>>, Vec<Ops>), AdventError> {
    Ok(trace("parser", terminated(parse_doc, (opt(line_ending), eof))).parse(content)?)
}

fn parse_doc(input: &mut &[u8]) -> ModalResult<(Vec<Vec<u64>>, Vec<Ops>)> {
    let mut first = vec![parse_first_line.parse_next(input)?];
    let len = first[0].len();

    first.extend(parse_next_lines(len).parse_next(input)?);

    let ops = parse_ops(len).parse_next(input)?;

    Ok((first, ops))
}

fn parse_first_line(input: &mut &[u8]) -> ModalResult<Vec<u64>> {
    trace(
        "parse_first_line",
        delimited(
            space0,
            separated(1.., dec_uint::<_, u64, _>, space1),
            line_ending,
        ),
    )
    .parse_next(input)
}

pub fn parse_next_lines<Input, Error>(count: usize) -> impl Parser<Input, Vec<Vec<u64>>, Error>
where
    Input: Stream + StreamIsPartial + Compare<&'static str>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace(
        "parse_next_lines",
        repeat(
            1..,
            delimited(
                space0,
                separated::<_, _, Vec<u64>, _, _, _, _>(
                    count..=count,
                    dec_uint::<_, u64, _>,
                    space1,
                ),
                (space0, line_ending),
            ),
        ),
    )
}

pub fn parse_ops<Input, Error>(count: usize) -> impl Parser<Input, Vec<Ops>, Error>
where
    Input: Stream + StreamIsPartial + Compare<&'static str> + Compare<u8>,
    <Input as Stream>::Slice: AsBStr,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    trace(
        "parse_ops",
        delimited(
            space0,
            separated::<_, _, Vec<Ops>, _, _, _, _>(
                count..=count,
                alt((b'+'.value(Ops::Add), b'*'.value(Ops::Mul))),
                space1,
            ),
            (space0, line_ending),
        ),
    )
}
