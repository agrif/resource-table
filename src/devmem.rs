use crate::types::{DevAddr, DevArea};
use crate::{constants, types, util};

#[repr(C)]
#[derive(Clone, Debug)]
pub struct DevMem {
    pub da: DevAddr,
    pub pa: u32,
    pub len: u32,
    pub flags: u32,
    pub reserved: u32,
    pub name: [u8; constants::RPROC_MAX_NAME_LEN],
}

impl types::ResourceType for DevMem {
    const RESOURCE_TYPE: u32 = 1;
}

impl DevMem {
    pub const fn new(buf: DevArea, pa: u32, flags: u32, name: &str) -> Self {
        Self {
            da: buf.addr,
            pa,
            len: buf.len as u32,
            flags,
            reserved: 0,
            name: util::str_to_array(name).expect("name too long"),
        }
    }
}
