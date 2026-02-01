#[repr(C)]
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
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

/// Device address representation.
///
/// Device address fields in the resource table are commonly filled
/// with pointer values that aren't known until link time. In Rust,
/// this is only possible using actual pointer types, but the resource
/// table pointer fields are a fixed 32 bits.
///
/// In order for this crate to be useful on 64-bit platforms, device
/// addresses are represented using this union. It always contains a
/// `raw` integer field, and on systems with 32-bit pointers, it also
/// contains a `ptr` field.
#[repr(C)]
#[derive(Clone, Copy)]
pub union DevAddr {
    /// Raw address.
    pub raw: u32,
    /// Pointer address (only on 32-bit platforms).
    #[cfg(target_pointer_width = "32")]
    pub ptr: *mut u8,
}

/// Pointers are not sync, but we necessarily need pointers stored in
/// statics.
unsafe impl Sync for DevAddr {}

impl core::fmt::Debug for DevAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        unsafe { write!(f, "{:x?}", self.raw) }
    }
}

impl DevAddr {
    /// Create a device address from a raw integer.
    pub const fn from_u32(raw: u32) -> Self {
        Self { raw }
    }

    /// Create a device address from a pointer (only on 32-bit platforms).
    #[cfg(target_pointer_width = "32")]
    pub const fn const_from_ptr(ptr: *mut u8) -> Self {
        Self { ptr }
    }

    /// Create a device address from a pointer.
    pub fn from_ptr(ptr: *mut u8) -> Self {
        Self { raw: ptr as u32 }
    }

    /// Get the device address as a raw integer.
    pub fn as_u32(&self) -> u32 {
        unsafe { self.raw }
    }

    /// Get the device address as a pointer.
    pub fn as_ptr(&self) -> *mut u8 {
        #[cfg(target_pointer_width = "32")]
        unsafe {
            self.ptr
        }

        #[cfg(not(target_pointer_width = "32"))]
        unsafe {
            self.raw as *mut u8
        }
    }
}

/// Device buffer representation.
///
/// This is a [DevAddr] paired with a length, representing a buffer of
/// bytes in device memory.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DevBuf {
    /// Base address of buffer.
    pub addr: DevAddr,
    /// Length of buffer, in bytes.
    pub len: usize,
}

impl DevBuf {
    /// Create a device buffer from a raw integer and length.
    pub const fn from_u32_len(raw: u32, len: usize) -> Self {
        Self {
            addr: DevAddr::from_u32(raw),
            len,
        }
    }

    /// Create a device buffer from a slice pointer (only on 32-bit
    /// platforms).
    #[cfg(target_pointer_width = "32")]
    pub const fn const_from_slice(ptr: *mut [u8]) -> Self {
        Self {
            addr: DevAddr::const_from_ptr(ptr as *mut u8),
            len: ptr.len(),
        }
    }

    /// Create a device address from a slice pointer.
    pub fn from_slice(ptr: *mut [u8]) -> Self {
        Self {
            addr: DevAddr::from_ptr(ptr as *mut u8),
            len: ptr.len(),
        }
    }

    /// Create a device buffer from a base pointer and length (only on
    /// 32-bit platforms).
    #[cfg(target_pointer_width = "32")]
    pub const fn const_from_ptr_len(ptr: *mut u8, len: usize) -> Self {
        Self {
            addr: DevAddr::const_from_ptr(ptr),
            len,
        }
    }

    /// Create a device address from a base pointer and length.
    pub fn from_ptr_len(ptr: *mut u8, len: usize) -> Self {
        Self {
            addr: DevAddr::from_ptr(ptr),
            len,
        }
    }
}
