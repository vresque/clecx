use crate::{Guid, UInt32};

pub const EFI_CAPSULE_FLAGS_PERSIST_ACROSS_RESET: UInt32 = 0x00010000;
pub const EFI_CAPSULE_FLAGS_POPULATE_SYSTEM_TABLE: UInt32 = 0x00020000;
pub const EFI_CAPSULE_FLAGS_INITIATE_RESET: UInt32 = 0x00040000;

#[repr(C)]
pub struct CapsuleHeader {
    guid: Guid,
    size: UInt32,
    flags: UInt32,
    image_size: UInt32,
}
