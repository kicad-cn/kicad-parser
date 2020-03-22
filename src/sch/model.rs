use std::collections::{HashMap};
#[derive(Default)]
pub struct SCHPageInfo {
    pub page_type: String,
    pub width: u64,
    pub height: u64,
    pub encoding: String,
    pub sheet:(u64,u64),
    pub is_portrait: bool,
    pub title_block:HashMap<String,String>
}

#[derive(Default)]
pub struct SCHTitleBlocks {
    pub date: String,
    pub title: String,
    pub revision: String,
    pub company: String,
    pub comments: Vec<String>,
}

#[derive(Default)]
pub struct SCHScreen {
    pub m_version: u8,
    pub m_title_blocks: SCHTitleBlocks,
    pub m_page_info: SCHPageInfo,
}
impl SCHScreen {
    pub fn New() -> SCHScreen {
        SCHScreen {
            ..Default::default()
        }
    }
}

pub trait SCHItem {}

//Meta info of schematic
pub struct PageInfo {}
//lower right block
pub struct TitleBlock {}
pub struct SCHSheet {}
