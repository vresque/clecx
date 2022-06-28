use bkshared::mmap::ComplexMemoryType;

use super::runtime::EfiMemoryDescriptor;
use crate::{
    status::Status, AllocateType, Bool, Event, Guid, Handle, IntN, StubFunc, StubParam,
    TaskPriorityLvl, UInt32, UInt64, UInt8, UIntN, VoidPtr,
};

use super::table::{TableHeader, EFI_SPECIFICATION_VERSION};

pub const EFI_BOOT_SERVICES_SIGNATURE: UInt64 = 0x56524553544f4f42;
pub const EFI_BOOT_SERVICES_REVISION: UInt64 = EFI_SPECIFICATION_VERSION;

#[repr(C)]
pub enum SearchType {
    /* Retrieve all handles */ AllHandles,
    /* Through Register Notify */ SearchByRegisterNotify,
    SearchByProtocol,
}

#[repr(C)]
pub struct BootServices {
    pub header: TableHeader,
    pub raise_tpl: extern "win64" fn(new: TaskPriorityLvl) -> UIntN,
    pub restore_tpl: extern "win64" fn(old: TaskPriorityLvl) -> UIntN,
    pub allocate_pages: extern "win64" fn(
        alloc_ty: AllocateType,
        mem_ty: ComplexMemoryType,
        pages: UIntN,
        memory: &mut UIntN,
    ) -> Status,
    pub free_pages: extern "win64" fn(memory: UIntN, pages: UIntN) -> Status,
    pub get_memory_map: extern "win64" fn(
        size: &mut UIntN,
        map: *mut EfiMemoryDescriptor,
        key: &mut UIntN,
        descriptor_size: &mut UIntN,
        version: &mut UInt32,
    ) -> Status,
    pub allocate_pool:
        extern "win64" fn(pool_ty: ComplexMemoryType, size: UIntN, buffer: &mut UIntN) -> Status,
    pub free_pool: extern "win64" fn(ptr: UIntN) -> Status,
    pub create_event: extern "win64" fn(
        kind: UInt32,
        notify_t: TaskPriorityLvl,
        notify_func: extern "win64" fn(evt: Event, context: VoidPtr) -> Status,
        event: &mut Event,
    ),
    set_timer: StubFunc,
    pub wait_for_event:
        extern "win64" fn(count: UIntN, evt: *const Event, idx: &mut UIntN) -> Status,
    signal_event: StubFunc,
    close_event: StubFunc,
    check_event: StubFunc,
    pub install_proto_interface:
        extern "win64" fn(handle: Handle, proto: &Guid, interface: UIntN) -> Status,
    pub reinstall_proto_interface: StubFunc,
    pub uinstall_proto_interface:
        extern "win64" fn(handle: Handle, protocol: &Guid, interface: UIntN) -> Status,
    pub handle_protocol:
        extern "win64" fn(handle: Handle, proto: &Guid, interface: &mut UIntN) -> Status,
    _reserved: UIntN,
    register_proto_notify: StubFunc,
    pub locate_handle: extern "win64" fn(
        search_ty: SearchType,
        proto: &Guid,
        key: UIntN,
        size: &mut UIntN,
        buffer: *mut Handle,
    ) -> Status,
    locate_device_path: StubFunc,
    install_config_table: StubFunc,
    pub load_image: extern "win64" fn(
        policy: Bool,
        parent: Handle,
        path: StubParam,
        source: *const UInt8,
        size: UIntN,
        handle: &mut Handle,
    ) -> Status,
    pub start_image: extern "win64" fn(
        handle: Handle,
        exit_data_size: &mut UIntN,
        exit_data: &mut *mut UInt32,
    ) -> Status,
    pub exit: extern "win64" fn(
        handle: Handle,
        status: IntN,
        data_size: usize,
        data: *const u16,
    ) -> Status,
    unload_image: StubFunc,
    pub exit_boot_services: extern "win64" fn(handle: Handle, map_key: UIntN) -> Status,
    get_next_mono_count: StubFunc,
    pub stall: extern "win64" fn(micros: UIntN) -> Status,
    pub set_watchdog_timer:
        extern "win64" fn(timeout: UIntN, code: UInt64, size: UIntN, data: *const u16) -> Status,
    connect_controller: StubFunc,
    disconnect_controller: StubFunc,
    open_protocol: StubFunc,
    close_protocol: StubFunc,
    open_protocol_information: StubFunc,
    pub protocols_per_handle:
        extern "win64" fn(handle: Handle, proto_buf: *mut Guid, count: UIntN) -> Status,
    pub locate_handle_buffer: extern "win64" fn(
        ty: SearchType,
        proto: &Guid,
        key: UIntN,
        no_handles: &mut UIntN,
        buffer: &mut *mut Handle,
    ),
    pub locate_protocol:
        extern "win64" fn(protocol: &Guid, registration: UIntN, interface: &mut usize) -> Status,
    install_multiple_proto_interfaces: StubFunc,
    uninstall_multiple_proto_interfaces: StubFunc,
    calculate_crc32: StubFunc,
    copy_mem: StubFunc,
    set_mem: StubFunc,
    pub create_event_ex: extern "win64" fn(
        kind: UInt32,
        notify_t: TaskPriorityLvl,
        notify_fn: extern "win64" fn(evt: Event, ctx: VoidPtr),
        context: VoidPtr,
        event: &mut Event,
    ),
}
