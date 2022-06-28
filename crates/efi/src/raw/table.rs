use crate::{Handle, UInt32, UInt64, UIntN, WString};

use super::{
    boot::BootServices,
    config::ConfigurationTable,
    runtime::RuntimeServices,
    text::{TextInput, TextOutput},
};

pub const EFI_SYSTEM_TABLE_SIGNATURE: UInt64 = 0x5453595320494249;
pub const EFI_2_80_SYSTEM_TABLE_REVISION: UInt64 = (2 << 16) | (80);
pub const EFI_2_70_SYSTEM_TABLE_REVISION: UInt64 = (2 << 16) | (70);
pub const EFI_2_60_SYSTEM_TABLE_REVISION: UInt64 = (2 << 16) | (60);
pub const EFI_2_50_SYSTEM_TABLE_REVISION: UInt64 = (2 << 16) | (50);
pub const EFI_2_40_SYSTEM_TABLE_REVISION: UInt64 = (2 << 16) | (40);
pub const EFI_2_31_SYSTEM_TABLE_REVISION: UInt64 = (2 << 16) | (31);
pub const EFI_2_30_SYSTEM_TABLE_REVISION: UInt64 = (2 << 16) | (30);
pub const EFI_2_20_SYSTEM_TABLE_REVISION: UInt64 = (2 << 16) | (20);
pub const EFI_2_10_SYSTEM_TABLE_REVISION: UInt64 = (2 << 16) | (10);
pub const EFI_2_00_SYSTEM_TABLE_REVISION: UInt64 = (2 << 16) | (00);
pub const EFI_1_10_SYSTEM_TABLE_REVISION: UInt64 = (1 << 16) | (10);
pub const EFI_1_02_SYSTEM_TABLE_REVISION: UInt64 = (1 << 16) | (02);
pub const EFI_SPECIFICATION_VERSION: UInt64 = EFI_SYSTEM_TABLE_REVISION;
pub const EFI_SYSTEM_TABLE_REVISION: UInt64 = EFI_2_80_SYSTEM_TABLE_REVISION;

#[repr(C)]
pub struct TableHeader {
    signature: UInt64,
    revision: UInt32,
    size: UInt32,
    crc32: UInt32,
    reserved: UInt32,
}

#[repr(C)]
pub struct Table {
    pub header: TableHeader,
    pub firmware_vendor: WString,
    pub revision: UInt32,
    pub con_in_handle: Handle,
    pub con_in: &'static TextInput,
    pub con_out_handle: Handle,
    pub con_out: &'static TextOutput,
    pub stderr_handle: Handle,
    pub stderr: &'static TextOutput,
    pub runtime: &'static RuntimeServices,
    pub boot: &'static BootServices,
    pub number_of_table_entries: UIntN,
    pub configuration: *const ConfigurationTable,
}
