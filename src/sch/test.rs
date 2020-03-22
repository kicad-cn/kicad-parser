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
use std::str::FromStr;
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
    assert_eq!(screen.m_page_info.page_type, "A4");
    assert_eq!(screen.m_page_info.width,11693);
    assert_eq!(screen.m_page_info.height,8268);
    assert_eq!(screen.m_page_info.title_block["Rev"],"V0.0");
    assert_eq!(screen.m_page_info.sheet,(1,7));
    assert_eq!(screen.m_page_info.encoding,"utf-8");
    assert_eq!(screen.m_page_info.title_block["Title"],"STM32H750VB Core Board");
}
