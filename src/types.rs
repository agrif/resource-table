#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Header<const N: usize> {
    pub ver: u32,
    pub num: u32,
    pub reserved: [u32; 2],
    pub offset: [u32; N],
}

impl<const N: usize> Header<N> {
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
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
