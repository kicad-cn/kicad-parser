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
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::path::Path;

#[test]
pub fn test_parse_page_settings() {
    let mut reader = SCHBufReader::new(
        File::open(
            Path::new(env::var("CARGO_MANIFEST_DIR").unwrap().as_str())
                .join("case")
                .join("STM32H750VB-Core")
                .join("H750_Core_v0.0")
                .join("H750_Core.sch"),
        )
        .unwrap(),
    );
    let mut screen = SCHScreen::New();
    screen.parse(&mut reader);
    assert_eq!(screen.m_version, 4);
}
