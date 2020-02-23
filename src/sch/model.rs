pub struct SCHScreen {
    pub m_version: u8,
}
impl SCHScreen {

    pub fn New() -> SCHScreen {
        SCHScreen { m_version: 0 }
    }
}

pub trait SCHItem {}

//Meta info of schematic
pub struct PageInfo {}
//lower right block
pub struct TitleBlock {}
pub struct SCHSheet {}
