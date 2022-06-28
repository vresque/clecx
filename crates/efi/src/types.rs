pub type Bool = bool;
pub type IntN = isize; // u32 on 32-bit systems
pub type UIntN = usize; // --^

pub type Int8 = i8;
pub type UInt8 = u8;

pub type Int16 = i16;
pub type UInt16 = u16;

pub type Int32 = i32;
pub type UInt32 = u32;

pub type Int64 = i64;
pub type UInt64 = u64;

pub type Int128 = i128;
pub type UInt128 = u128;

pub type Char8 = u8;
pub type Char16 = u16;

pub type Void = ();
pub type Event = usize;
pub type Handle = usize;
pub type LogicalBlockAddr = u64;
pub type TaskPriorityLvl = UIntN;

pub type VoidPtr = *mut ();
pub type StubFunc = extern "win64" fn();
pub type StubParam = usize;

/// https://uefi.org/sites/default/files/resources/UEFI%20Spec%202.8B%20May%202020.pdf p.177
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct Guid(pub u32, pub u16, pub u16, pub [u8; 8]);

impl core::fmt::Debug for Guid {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Guid({:>8x}, {:>4x}, {:>4x}, [", self.0, self.1, self.2)?;
        for i in self.3 {
            write!(f, "{:>2x}, ", i)?;
        }
        write!(f, "])")?;

        Ok(())
    }
}

#[repr(C)]
pub enum AllocateType {
    AnyPages = 0,
    MaxAddress = 1,
    Address = 2,
}
