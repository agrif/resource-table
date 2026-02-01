use crate::types::{DevAddr, DevArea};
use crate::{constants, types, util};

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Trace {
    pub da: DevAddr,
    pub len: u32,
    pub reserved: u32,
    pub name: [u8; constants::RPROC_MAX_NAME_LEN],
}

impl types::ResourceType for Trace {
    const RESOURCE_TYPE: u32 = 2;
}

impl Trace {
    pub const fn new(buf: DevArea, name: &str) -> Self {
        Self {
            da: buf.addr,
            len: buf.len as u32,
            reserved: 0,
            name: util::str_to_array(name).expect("name too long"),
        }
    }
}
