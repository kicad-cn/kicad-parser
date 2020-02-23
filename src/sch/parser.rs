use super::model::SCHScreen;
use super::reader::SCHRead;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::{
        alphanumeric0, alphanumeric1, digit1, line_ending, multispace1, not_line_ending, space0,
    },
    combinator::map_res,
    error::ErrorKind,
    multi::{fold_many0, many0, many_till},
    sequence::{preceded, terminated},
    Err as nomErr, IResult,
};
use std::str;

pub trait SCHParseLegacy {
    fn set_version(&mut self, version: u8);

    fn parse<R: SCHRead + Sized>(&mut self, reader: &mut R) {
        let all = reader.as_str().unwrap();
        let e = self.parse_sch(all);
        println!("{:?}", e);
    }
    fn parse_sch<'a>(&mut self, input: &'a str) -> IResult<&'a str, &'a str> {
        let (res, _) = self.parse_header(input)?;
        Self::parse_items(res)
    }
    fn parse_empty_lines(input: &str) -> IResult<&str, Vec<&str>> {
        many0(alt((
            preceded(space0, line_ending),
            preceded(tag("#"), terminated(alphanumeric0, multispace1)),
        )))(input)
    }
    fn parse_item_tag(input: &str) -> IResult<&str, &str> {
        let (res, _) = Self::parse_empty_lines(input)?;
        preceded(tag("$"), terminated(alphanumeric1, multispace1))(res)
    }

    fn parse_header<'a>(&mut self, input: &'a str) -> IResult<&'a str, (Vec<&'a str>, &'a str)> {
        let (res, _) = preceded(
            Self::parse_empty_lines,
            tag("EESchema Schematic File Version "),
        )(input)?;
        let (res, version) = terminated(
            map_res(digit1, |e: &str| u16::from_str_radix(e, 10)),
            line_ending,
        )(res)?;
        self.set_version(version as u8);
        many_till(
            terminated(take_till(|e| e == '\n'), line_ending),
            terminated(tag("EELAYER END"), line_ending),
        )(res)
    }
    fn parse_items(input: &str) -> IResult<&str, &str> {
        let (res, item_tag) = Self::parse_item_tag(input)?;
        match item_tag {
            "Descr" => Self::parse_page_settings(res),
            _ => Err(nomErr::Error(("unexpected eof", ErrorKind::Eof))),
        }
    }
    fn parse_page_settings(input: &str) -> IResult<&str, &str> {
        take_till(|e| e == '1')(input)
    }
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

impl SCHParseLegacy for SCHScreen {
    fn set_version(&mut self, version: u8) {
        self.m_version = version;
    }
}
