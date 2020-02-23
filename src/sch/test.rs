use super::model::SCHScreen;
use super::parser::SCHParseLegacy;
use super::reader::SCHBufReader;
use nom::{
    bytes::streaming::{tag, take},
    error::ErrorKind::Tag,
    number::streaming::be_u16,
    sequence::tuple,
    Needed,
};
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

#[test]
pub fn testParsePageSetting() {
    let mut reader = SCHBufReader::New(BufReader::new(
        r#"
 
#foo 
EESchema Schematic File Version 4
EELAYER 30 0
EELAYER END
"#
        .as_bytes(),
    ));
    let mut screen = SCHScreen::New();
    screen.parse(&mut reader);
}

#[test]
pub fn testNom() {
    let tpl = tuple((be_u16, take(3u8), tag("fg")));
    assert_eq!(
        tpl(&b"abcdefgh"[..]),
        Ok((&b"h"[..], (0x6162u16, &b"cde"[..], &b"fg"[..])))
    );
    assert_eq!(
        tpl(&b"abcde"[..]),
        Err(nom::Err::Incomplete(Needed::Size(2)))
    );
    let input = &b"abcdejk"[..];
    assert_eq!(tpl(input), Err(nom::Err::Error((&input[5..], Tag))));
}
