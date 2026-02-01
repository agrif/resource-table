use crate::types::{DevAddr, DevBuf};
use crate::{constants, types, util};

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Carveout {
    pub da: DevAddr,
    pub pa: u32,
    pub len: u32,
    pub flags: u32,
    pub reserved: u32,
    pub name: [u8; constants::RPROC_MAX_NAME_LEN],
}

impl types::ResourceType for Carveout {
    const RESOURCE_TYPE: u32 = 0;
}

impl Carveout {
    pub const fn new_dynamic(len: usize, flags: u32, name: &str) -> Self {
        Self {
            da: DevAddr::from_u32(constants::FW_RSC_ADDR_ANY),
            pa: 0,
            len: len as u32,
            flags,
            reserved: 0,
            name: util::str_to_array(name).expect("name too long"),
        }
    }

    pub const fn new_fixed(buf: DevBuf, flags: u32, name: &str) -> Self {
        Self {
            da: buf.addr,
            pa: 0,
            len: buf.len as u32,
            flags,
            reserved: 0,
            name: util::str_to_array(name).expect("name too long"),
        }
    }
}
