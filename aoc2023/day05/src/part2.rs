use std::{collections::BTreeMap, ops::Range};

use crate::aocerror::{AocError, AocErrorKind, AocParseError};
use miette::SourceSpan;
use nom::{
    Finish, IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, multispace0, multispace1, newline, space0, space1},
    combinator::{all_consuming, map, value},
    multi::separated_list1,
    sequence::{preceded, terminated, tuple},
};
use rangemap::RangeMap;

pub type SeedList = Vec<Range<u64>>;
pub type DataMap = RangeMap<u64, u64>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DataSeq {
    SeedToSoil = 0,
    SoilToFertilizer = 1,
    FertilizerToWater = 2,
    WaterToLight = 3,
    LightToTemperature = 4,
    TemperatureToHumidity = 5,
    HumidityToLocation = 6,
}

#[derive(Debug, Clone, Default)]
pub struct Data {
    pub seeds: SeedList,
    pub maps: BTreeMap<DataSeq, DataMap>,
}

fn seeds(input: &str) -> IResult<&str, SeedList, AocParseError<&str>> {
    preceded(
        tuple((tag("seeds:"), space0)),
        separated_list1(
            space1,
            map(
                tuple((complete::u64, space1, complete::u64)),
                |(start, _, len)| start..(start + len),
            ),
        ),
    )
    .parse(input)
}

// pub fn length_value() -> impl Fn(Input) -> IResult<Input, Input, Error> {
//     tag("seed-to-soil")
// }
fn map_headers(input: &str) -> IResult<&str, DataSeq, AocParseError<&str>> {
    terminated(
        alt((
            value(DataSeq::SeedToSoil, tag("seed-to-soil")),
            value(DataSeq::SoilToFertilizer, tag("soil-to-fertilizer")),
            value(DataSeq::FertilizerToWater, tag("fertilizer-to-water")),
            value(DataSeq::WaterToLight, tag("water-to-light")),
            value(DataSeq::LightToTemperature, tag("light-to-temperature")),
            value(
                DataSeq::TemperatureToHumidity,
                tag("temperature-to-humidity"),
            ),
            value(DataSeq::HumidityToLocation, tag("humidity-to-location")),
        )),
        tuple((space1, tag("map:"))),
    )
    .parse(input)
}
fn map_range(input: &str) -> IResult<&str, (Range<u64>, u64), AocParseError<&str>> {
    let ranges = tuple((complete::u64, space1, complete::u64, space1, complete::u64));
    map(ranges, |(dst, _, src, _, len)| (src..(src + len), dst)).parse(input)
}
fn map_ranges(input: &str) -> IResult<&str, DataMap, AocParseError<&str>> {
    //let data_map = DataMap::new();
    map(separated_list1(newline, map_range), |vec| {
        vec.into_iter().collect::<DataMap>()
    })
    .parse(input)
}
fn map_kind(input: &str) -> IResult<&str, (DataSeq, DataMap), AocParseError<&str>> {
    tuple((map_headers, preceded(newline, map_ranges))).parse(input)
}
fn maps(input: &str) -> IResult<&str, BTreeMap<DataSeq, DataMap>, AocParseError<&str>> {
    map(separated_list1(multispace1, map_kind), |vec| {
        vec.into_iter().collect()
    })
    .parse(input)
}
fn data_parser(input: &str) -> IResult<&str, Data, AocParseError<&str>> {
    all_consuming(map(
        tuple((multispace0, seeds, multispace1, maps, multispace0)),
        |(_, seeds, _, maps, _)| Data { seeds, maps },
    ))
    .parse(input)
}

// #[test]
// fn header_test() -> Result<(), AocParseError<&str>> {
//     let input = "soil-to-fertilizer map:";
//     let res = all_consuming(delimited(multispace0, map_headers, multispace0))
//         .parse(input)
//         .finish();
//     match res {
//         Ok(data) => println!("Data: {data:?}"),
//         Err(err) => {
//             return Err(err);
//         }
//     };
//     Ok(())
// }

// #[test]
// fn range_test() -> Result<(), String> {
//     let input = "10 20 5\n200 100 42";
//     let res = all_consuming(delimited(multispace0, map_ranges, multispace0))
//         .parse(input)
//         .finish();
//     match res {
//         Ok(data) => println!("Data: {data:#?}"),
//         Err(err) => {
//             return Err(convert_error(input, err));
//         }
//     };
//     Ok(())
// }

// #[test]
// fn range_maps() -> Result<(), String> {
//     let input = "seed-to-soil map:
// 50 98 2
// 52 50 48

// soil-to-fertilizer map:
// 0 15 37
// 37 52 2
// 39 0 15

// fertilizer-to-water map:
// 49 53 8
// 0 11 42
// 42 0 7
// 57 7 4

// water-to-light map:
// 88 18 7
// 18 25 70

// light-to-temperature map:
// 45 77 23
// 81 45 19
// 68 64 13

// temperature-to-humidity map:
// 0 69 1
// 1 0 69

// humidity-to-location map:
// 60 56 37
// 56 93 4
// ";
//     let res = all_consuming(delimited(multispace0, maps, multispace0))
//         .parse(input)
//         .finish();
//     match res {
//         Ok(data) => println!("Data: {data:#?}"),
//         Err(err) => {
//             return Err(convert_error(input, err));
//         }
//     };
//     Ok(())
// }

// #[test]
// fn header_test() -> Result<(), String> {
//     let input = "soil-to-fertilizer map:";
//     let res = all_consuming(delimited(multispace0, map_headers, multispace0))
//         .parse(input)
//         .finish();
//     match res {
//         Ok(data) => println!("Data: {data:?}"),
//         Err(err) => {
//             return Err(err);
//         }
//     };
//     Ok(())
// }

pub fn parse(input: &str) -> Result<Data, AocError> {
    // let mut data = Data::default();
    // let _ = input
    //     .lines()
    //     .enumerate()
    //     .map(|(lineno, line)| parse_line(lineno, line))
    //     .collect::<Result<Vec<_>, _>>()?;
    let (_, data) = data_parser(input).finish().map_err(|err| {
        let span_substr = &err.input[..err.len];

        AocError::ParseError {
            input: input.to_owned(),
            span: span_from_substr(input, span_substr),
            help: err.help,
            label: err.label,
            kind: if let Some(kind) = err.kind {
                kind
            } else if let Some(ctx) = err.context {
                AocErrorKind::Context(ctx)
            } else {
                AocErrorKind::Other
            },
        }
    })?;

    Ok(data)
}

/// Creates a span for an item using a substring of self.full_input
///
/// Note that substr must be a literal substring, as in it must be
/// a pointer into the same string!
pub(crate) fn span_from_substr(input: &str, substr: &str) -> SourceSpan {
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

// pub fn parse_line(lineno: usize, line: &str) -> Result<(), AocError> {
//     let _ = line
//         .trim()
//         .split(' ')
//         .filter(|num| !num.is_empty())
//         .map(|num| {
//             num.parse::<u32>().map_err(|err| AocError::InvalidNumber {
//                 src: AocSourceChunk::new(line.to_owned(), lineno),
//                 span: aoc_error_span(line, num),
//                 inner: Some(Box::new(err)),
//             })
//         })
//         .collect::<Result<Vec<_>, _>>()?;
//     Ok(())
// }
