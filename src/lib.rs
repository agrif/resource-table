#![no_std]

// useful references:
// https://github.com/torvalds/linux/blob/master/include/linux/remoteproc.h
// https://github.com/OpenAMP/open-amp/blob/main/lib/include/openamp/remoteproc.h

mod util;

pub const RSC_NOTIFY_ID_ANY: u32 = 0xffffffff;
pub const RPROC_MAX_NAME_LEN: usize = 32;
pub const FW_RSC_ADDR_ANY: u32 = 0xffffffff;

#[repr(C)]
#[derive(Debug)]
pub struct ResourceTableHeader<const N: usize> {
    pub ver: u32,
    pub num: u32,
    pub reserved: [u32; 2],
    pub offset: [u32; N],
}

impl<const N: usize> ResourceTableHeader<N> {
    pub const fn new(offset: [u32; N]) -> Self {
        Self {
            ver: 1,
            num: N as u32,
            reserved: [0; _],
            offset,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Resource<T> {
    pub type_: u32,
    pub data: T,
}

pub trait ResourceType {
    const RESOURCE_TYPE: u32;
}

impl<T> Resource<T>
where
    T: ResourceType,
{
    pub const fn new(data: T) -> Self {
        Self {
            type_: T::RESOURCE_TYPE,
            data,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Carveout {
    da: u32,
    pa: u32,
    len: u32,
    flags: u32,
    reserved: u32,
    name: [u8; RPROC_MAX_NAME_LEN],
}

impl ResourceType for Carveout {
    const RESOURCE_TYPE: u32 = 0;
}

impl Carveout {
    pub const fn new(da: Option<u32>, len: usize, flags: u32, name: &str) -> Self {
        Self {
            da: if let Some(addr) = da {
                addr
            } else {
                FW_RSC_ADDR_ANY
            },
            pa: 0,
            len: len as u32,
            flags,
            reserved: 0,
            name: util::str_to_array(name).expect("name too long"),
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ResourceTable {
    header: ResourceTableHeader<1>,
    carveout: Resource<Carveout>,
}

impl ResourceTable {
    pub const fn new() -> Self {
        Self {
            header: ResourceTableHeader::new([core::mem::offset_of!(Self, carveout) as u32]),
            carveout: Resource::new(Carveout::new(None, 0x8000, 0, "carveout")),
        }
    }
}

#[unsafe(link_section = ".resource_table")]
#[unsafe(no_mangle)]
#[allow(dead_code)]
pub static _RESOURCE_TABLE: ResourceTable = ResourceTable::new();
