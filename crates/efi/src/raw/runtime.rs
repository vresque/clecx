use bkshared::memory::{PhysicalAddress, VirtualAddress};
use bkshared::mmap::{ComplexMemoryType, MemoryDescriptor};

use super::capsule::CapsuleHeader;
use super::table::TableHeader;
use super::time::{Time, TimeCapabilities};
use crate::UInt64;
use crate::{status::Status, Bool, Guid, UInt32, UInt8, UIntN, WString};

#[repr(C)]
pub struct EfiMemoryDescriptor {
    pub ty: ComplexMemoryType,
    pub phys_start: PhysicalAddress,
    pub virt_start: VirtualAddress,
    pub count: u64,
    pub attribute: u64,
}

pub enum ResetType {
    Cold,
    Warm,
    Shutdown,
}

#[repr(C)]
pub struct RuntimeServices {
    pub header: TableHeader,

    pub get_time: extern "win64" fn(time: &mut Time, capabilities: *mut TimeCapabilities) -> Status,

    pub set_time: extern "win64" fn(time: &Time) -> Status,

    pub get_wakeup_time:
        extern "win64" fn(enabled: &mut Bool, pending: &mut Bool, time: &mut Time) -> Status,

    pub set_wakeup_time: extern "win64" fn(enable: Bool, time: *const Time) -> Status,

    pub set_virt_addr_map: extern "win64" fn(
        mmap_size: UIntN,
        descr_size: UIntN,
        descr_version: UInt32,
        virt_map: *const EfiMemoryDescriptor,
    ) -> Status,

    pub convert_ptr: extern "win64" fn(debug_dispos: UIntN, addr: &mut UIntN) -> Status,

    pub get_variable: extern "win64" fn(
        name: WString,
        vendor_guid: &Guid,
        attrs: *mut UInt32,
        data_sz: &mut UIntN,
        daza: *mut u8,
    ) -> Status,

    pub get_next_var_name:
        extern "win64" fn(name_sz: &mut UIntN, name: WString, vendor_guid: &mut Guid) -> Status,

    pub set_variable: extern "win64" fn(
        name: WString,
        vendor_guid: &Guid,
        attrs: UInt32,
        size: UIntN,
        data: *const UInt8,
    ) -> Status,

    pub get_next_high_mono_count: extern "win64" fn(high_count: &mut UInt32) -> Status,

    pub reset_system: extern "win64" fn(
        ResetType: ResetType,
        ResetStatus: Status,
        DataSize: UIntN,
        ResetData: *const u8,
    ) -> !,

    pub update_capsule: extern "win64" fn(
        capsule_headers: *const *const CapsuleHeader,
        count: UIntN,
        scatter_gather_list: PhysicalAddress,
    ) -> Status,

    pub query_capsule_capabilities: extern "win64" fn(
        capsule_headers: *const *const CapsuleHeader,
        count: UIntN,
        max_capsule_size: &mut UInt64,
        reset_ty: &mut ResetType,
    ) -> Status,

    pub query_var_info: extern "win64" fn(
        attr: UInt32,
        max_storage: &mut u64,
        remaining_storage: &mut u64,
        max_var_size: &mut UInt64,
    ) -> Status,
}
