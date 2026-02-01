#![no_std]

// useful references:
// https://github.com/torvalds/linux/blob/master/include/linux/remoteproc.h
// https://github.com/OpenAMP/open-amp/blob/main/lib/include/openamp/remoteproc.h

mod r#macro;
mod util;

pub mod constants;
pub mod types;

#[repr(C)]
#[derive(Debug)]
pub struct Carveout {
    da: u32,
    pa: u32,
    len: u32,
    flags: u32,
    reserved: u32,
    name: [u8; constants::RPROC_MAX_NAME_LEN],
}

impl types::ResourceType for Carveout {
    const RESOURCE_TYPE: u32 = 0;
}

impl Carveout {
    pub const fn new(da: Option<u32>, len: usize, flags: u32, name: &str) -> Self {
        Self {
            da: if let Some(addr) = da {
                addr
            } else {
                constants::FW_RSC_ADDR_ANY
            },
            pa: 0,
            len: len as u32,
            flags,
            reserved: 0,
            name: util::str_to_array(name).expect("name too long"),
        }
    }
}

crate::resource_table! {
    /// Documentation comment.
    pub static CARVEOUT: Carveout = Carveout::new(None, 0x8000, 0, "carveout");
    pub static CARVEOUT2: Carveout = Carveout::new(None, 0x4000, 1, "haha");
}
