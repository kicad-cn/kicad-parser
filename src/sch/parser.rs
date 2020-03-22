use super::model::{SCHPageInfo, SCHScreen};
use super::reader::SCHRead;
use nom::character::is_alphabetic;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until, take_while},
    character::complete::{
        alphanumeric0, alphanumeric1, char as nchar, digit1, line_ending, multispace1,
        not_line_ending, space0,
    },
    combinator::map_res,
    error::ErrorKind,
    multi::{fold_many0, many0, many_till},
    sequence::{preceded, separated_pair, terminated, tuple},
    Err as nomErr, IResult,
};
use std::collections::HashMap;
use std::iter::FromIterator;
use std::str;
use std::str::FromStr;

pub trait SCHParseLegacy {
    fn set_version(&mut self, version: u8);
    fn set_pageinfo(&mut self, pageinfo: SCHPageInfo);

    fn parse<R: SCHRead + Sized>(&mut self, reader: &mut R) {
        let all = reader.as_str().unwrap();
        let e = self.parse_sch(all);

        println!("{:?}", e);
    }
    fn parse_sch<'a>(&mut self, input: &'a str) -> IResult<&'a str, ()> {
        let (res, _) = self.parse_header(input)?;
        self.parse_items(res)
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
    fn parse_items<'a>(&mut self, input: &'a str) -> IResult<&'a str, ()> {
        let (res, item_tag) = Self::parse_item_tag(input)?;
        match item_tag {
            "Descr" => self.parse_page_settings(res),
            _ => Err(nomErr::Error(("unexpected eof", ErrorKind::Eof))),
        }
    }
    fn parse_page_settings<'a>(&mut self, input: &'a str) -> IResult<&'a str, ()> {
        let mut is_portrait: bool;
        let mut encoding = String::new();
        let mut sheet: (u64, u64) = (0, 0);
        let mut title_block = HashMap::new();

        let (res, (page_type, (width, height))) = terminated(
            separated_pair(
                alphanumeric1,
                tag(" "),
                separated_pair(
                    map_res(digit1, |e: &str| u64::from_str_radix(e, 10)),
                    tag(" "),
                    map_res(digit1, |e: &str| u64::from_str_radix(e, 10)),
                ),
            ),
            line_ending,
        )(input)?;

        let (res, s) = alt((
            terminated(tag("portrait"), line_ending),
            take_till(|e| is_alphabetic(e as u8)),
        ))(res)?;

        is_portrait = s == "portrait";

        let mut res: &str = res;
        let mut key: &str;

        while true {
            let e = alt((
                terminated(alphanumeric1, multispace1),
                terminated(tag("$EndDescr"), multispace1),
            ))(res)?;
            res = e.0;
            key = e.1;

            if key == "encoding" {
                let e = terminated(not_line_ending, line_ending)(res)?;
                res = e.0;
                encoding = String::from_str(e.1).unwrap();
            } else if key == "Sheet" {
                let e = terminated(
                    separated_pair(
                        map_res(digit1, |e: &str| u64::from_str_radix(e, 10)),
                        multispace1,
                        map_res(digit1, |e: &str| u64::from_str_radix(e, 10)),
                    ),
                    line_ending,
                )(res)?;
                res = e.0;
                sheet = e.1;
            } else if key == "$EndDescr" {
                break;
            } else {
                let e = terminated(not_line_ending, line_ending)(res)?;
                res = e.0;
                title_block.insert(
                    String::from_str(key).unwrap(),
                    String::from_str(e.1).unwrap().replace("\"", ""),
                );
            }
        }

        self.set_pageinfo(SCHPageInfo {
            width,
            height,
            is_portrait,
            sheet,
            title_block,
            encoding,
            page_type: String::from_str(page_type).unwrap(),
        });
        println!("{:?}", res);

        return Ok((res, ()));
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
    fn set_pageinfo(&mut self, pageinfo: SCHPageInfo) {
        self.m_page_info = pageinfo;
    }
}
