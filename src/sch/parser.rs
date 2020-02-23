use super::model::SCHScreen;
use super::reader::SCHRead;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric0, digit1, line_ending, space0},
    combinator::map_res,
    multi::many0,
    sequence::{preceded, terminated},
    IResult,
};
use std::str;

pub trait SCHParseLegacy {
    fn parse<R: SCHRead + Sized>(&mut self, reader: &mut R) {
        let all = reader.as_str().unwrap();
        Self::parse_header(all);
    }
    fn parse_empty_lines(input: &str) -> IResult<&str, Vec<&str>> {
        many0(alt((
            preceded(space0, line_ending),
            preceded(tag("#"), terminated(alphanumeric0, line_ending)),
        )))(input)
    }
    fn parse_lines(input: &str) -> IResult<&str, Vec<&str>> {
        many0(terminated(alphanumeric0, line_ending))(input)
    }

    fn parse_header(input: &str) -> IResult<&str, Vec<&str>> {
        let (res, version) = terminated(
            preceded(
                preceded(
                    Self::parse_empty_lines,
                    tag("EESchema Schematic File version"),
                ),
                map_res(digit1, |e: &str| u16::from_str_radix(e, 10)),
            ),
            line_ending,
        )(input)?;
        terminated(
            Self::parse_lines,
            terminated(tag("EELAYER END"), line_ending),
        )(res)
    }

    // fn parse_page_settings(&mut self) {}
    // fn ParseComponent(&mut self);
    // fn ParseSheet(&mut self);
    // fn ParseBitmap(&mut self);
    // fn ParseJunction(&mut self);
    // fn ParseNoConnect(&mut self);
    // fn ParseWire(&mut self);
    // fn ParseBusEntry(&mut self);
    // fn ParseText(&mut self);
    // fn ParseBusAlias(&mut self);
}

impl SCHParseLegacy for SCHScreen {}
