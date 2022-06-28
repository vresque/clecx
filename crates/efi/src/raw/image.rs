use bkshared::mmap::ComplexMemoryType;

use crate::{status::Status, Handle, UInt32, UInt64, UIntN, WString};

use super::table::Table;

#[repr(C)]
pub struct RawLoadedImage {
    pub revision: UInt32,
    pub parent: Handle,
    pub table: &'static mut Table,
    pub device: Handle,
    pub path: UIntN,
    pub reserved: UIntN,
    pub options_size: UInt32,
    pub options: WString,
    pub base: UIntN,
    pub size: UInt64,
    pub code: ComplexMemoryType,
    pub data: ComplexMemoryType,
    pub unload: extern "win64" fn(handle: Handle) -> Status,
}
