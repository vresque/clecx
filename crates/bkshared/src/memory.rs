use core::ops::{Deref, DerefMut};

pub type Addr = usize;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct PhysicalAddress(pub u64);

impl Deref for PhysicalAddress {
    fn deref(&self) -> &Self::Target {
        &self.0
    }
    type Target = u64;
}

impl DerefMut for PhysicalAddress {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct VirtualAddress(pub u64);

impl Deref for VirtualAddress {
    fn deref(&self) -> &Self::Target {
        &self.0
    }
    type Target = u64;
}

impl DerefMut for VirtualAddress {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
