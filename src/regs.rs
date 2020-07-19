/// Seesaw HW ID code.
pub const SEESAW_HW_ID_CODE: u8 = 0x55;

pub mod base {
    //! Module Base Addreses
    //! The module base addresses for different seesaw modules.
    pub const SEESAW_STATUS_BASE: u8 = 0x00;
    pub const SEESAW_TOUCH_BASE: u8 = 0x0F;
}

pub mod func {
    //! status module function addres registers
    pub const SEESAW_STATUS_HW_ID: u8 = 0x01;
    pub const SEESAW_STATUS_TEMP: u8 = 0x04;
}

pub mod touch {
    //! Touch module function addres registers
    pub const SEESAW_TOUCH_CHANNEL_OFFSET: u8 = 0x10;
}
