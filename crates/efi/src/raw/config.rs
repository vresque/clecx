use crate::{Guid, UIntN};

#[derive(Debug)]
#[repr(C)]
pub struct ConfigurationTable {
    pub vendor_guid: Guid,
    pub vendor_table: UIntN,
}
