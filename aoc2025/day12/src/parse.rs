use common::error::AdventError;
use winnow::{
    ascii::{dec_uint, digit1, line_ending, space0, space1},
    combinator::{alt, delimited, eof, repeat, separated, separated_pair, terminated, trace},
    prelude::*,
};

type Region = ((usize, usize), Vec<usize>);

pub fn parse(content: &[u8]) -> Result<(Vec<usize>, Vec<Region>), AdventError> {
    Ok(trace("parser", terminated(parse_doc, eof)).parse(content)?)
}

fn parse_doc(input: &mut &[u8]) -> ModalResult<(Vec<usize>, Vec<Region>)> {
    trace("parse_doc", (parse_presents, parse_regions)).parse_next(input)
}

fn parse_presents(input: &mut &[u8]) -> ModalResult<Vec<usize>> {
    trace("parse_presents", repeat(1.., parse_present)).parse_next(input)
}

fn parse_present(input: &mut &[u8]) -> ModalResult<usize> {
    trace(
        "parse_present",
        delimited(parse_present_header, parse_present_body, line_ending),
    )
    .parse_next(input)
}

fn parse_present_header(input: &mut &[u8]) -> ModalResult<()> {
    trace("parse_present_header", (digit1, b':', line_ending).void()).parse_next(input)
}

fn parse_present_body(input: &mut &[u8]) -> ModalResult<usize> {
    trace(
        "parse_present_body",
        (parse_present_line, parse_present_line, parse_present_line)
            .map(|(i1, i2, i3)| i1 + i2 + i3),
    )
    .parse_next(input)
}

fn parse_present_line(input: &mut &[u8]) -> ModalResult<usize> {
    trace(
        "parse_present_line",
        terminated(
            (
                alt((b'.'.value(0_usize), b'#'.value(1_usize))),
                alt((b'.'.value(0_usize), b'#'.value(1_usize))),
                alt((b'.'.value(0_usize), b'#'.value(1_usize))),
            )
                .map(|(i1, i2, i3)| i1 + i2 + i3),
            line_ending,
        ),
    )
    .parse_next(input)
}

fn parse_regions(input: &mut &[u8]) -> ModalResult<Vec<Region>> {
    trace(
        "parse_regions",
        repeat(1.., terminated(parse_region, line_ending)),
    )
    .parse_next(input)
}

fn parse_region(input: &mut &[u8]) -> ModalResult<Region> {
    trace(
        "parse_region",
        separated_pair(
            separated_pair(dec_uint, b'x', dec_uint),
            (b':', space0),
            separated::<_, _, Vec<_>, _, _, _, _>(1.., dec_uint::<_, usize, _>, space1),
        ),
    )
    .parse_next(input)
}
