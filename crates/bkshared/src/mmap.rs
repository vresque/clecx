#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum ComplexMemoryType {
    Reserved,
    LoaderCode,
    LoaderData,
    BootServicesCode,
    BootServicesData,
    RuntimeServicesCode,
    RuntimeServicesData,
    ConventionalMemory,
    UnusableMemory,
    ACPIReclaimMemory,
    ACPIMemoryNVS,
    MemoryMappedIO,
    MemoryMappedIOPortSpace,
    PalCode,
    PersistentMemory,
    MaxMemoryType,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum MemoryType {
    Null,
    Free,
    Reclaimable,
    Reserved = 3,
}

impl From<ComplexMemoryType> for MemoryType {
    fn from(com: ComplexMemoryType) -> Self {
        match com {
            ComplexMemoryType::LoaderCode
            | ComplexMemoryType::LoaderData
            | ComplexMemoryType::BootServicesCode
            | ComplexMemoryType::BootServicesData
            | ComplexMemoryType::ConventionalMemory => MemoryType::Free,
            ComplexMemoryType::ACPIReclaimMemory => MemoryType::Reclaimable,
            _ => MemoryType::Reserved,
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct MemoryDescriptor {
    pub complex: ComplexMemoryType,
    pub ty: MemoryType,
    pub base: u64,
    pub size: u64,
}

#[repr(packed)]
#[derive(Copy, Clone, Debug)]
pub struct MemoryMap {
    pub members: [MemoryDescriptor; 512],
}
