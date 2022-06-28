use crate::{status::Status, Char16, Guid, UInt128, UInt64, UInt8, UIntN, WString};

use super::time::Time;

#[repr(u64)]
pub enum FileMode {
    Read = 0x0000000000000001,
    Write = 0x0000000000000002,
    Create = 0x8000000000000000,
}

#[repr(u64)]
pub enum FileAttribute {
    ReadOnly = 0x01,
    Hidden = 0x02,
    System = 0x04,
    Reserved = 0x08,
    Directory = 0x10,
    Archive = 0x20,
}

#[repr(C)]
pub struct SimpleFileSystemProtocol {
    pub revision: UInt64,
    pub open_volume:
        extern "win64" fn(&mut SimpleFileSystemProtocol, root: &mut *mut RawFile) -> Status,
}

#[repr(C)]
pub struct FileInfo {
    pub size: UInt64,
    pub file_size: UInt64,
    pub phys_size: UInt64,
    pub create_time: Time,
    pub last_access: Time,
    pub last_modification: Time,
    pub attribute: UInt64,
    pub name: [Char16; 256],
}

impl Default for FileInfo {
    fn default() -> Self {
        Self {
            size: 0,
            file_size: 0,
            phys_size: 0,
            create_time: Time::default(),
            last_access: Time::default(),
            last_modification: Time::default(),
            attribute: 0,
            name: [0; 256],
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RawFile {
    pub revision: u64,
    pub open: extern "win64" fn(
        &mut RawFile,
        new: &mut *mut RawFile,
        filename: WString,
        mode: u64,
        attrs: u64,
    ) -> Status,
    pub close: extern "win64" fn(&mut RawFile) -> Status,
    pub delete: extern "win64" fn(&mut RawFile) -> Status,
    pub read: extern "win64" fn(&mut RawFile, size: &mut usize, buffer: *mut UInt8) -> Status,
    pub write: extern "win64" fn(&mut RawFile, size: &mut usize, buffer: *const UInt8) -> Status,
    pub set_position: extern "win64" fn(&mut RawFile, pos: UInt64) -> Status,
    pub get_position: extern "win64" fn(&mut RawFile, position: &mut UIntN) -> Status,
    pub get_info:
        extern "win64" fn(&mut RawFile, ty: &Guid, size: &mut UIntN, buffer: *mut UInt8) -> Status,
    pub set_info: extern "win64" fn(
        &mut RawFile,
        ty: &Guid,
        size: &mut UIntN,
        buffer: *const UInt8,
    ) -> Status,
    pub flush: extern "win64" fn(&mut RawFile) -> Status,
}
