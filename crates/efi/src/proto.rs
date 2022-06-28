use crate::alloc::vec::Vec;

use crate::{raw::boot::SearchType, status::EfiResult, table, Guid, Handle};

pub trait Protocol<Inner: 'static>: Sized {
    fn guid() -> Guid;
    fn new(inner: &'static mut Inner) -> Self;

    fn locate() -> EfiResult<Self> {
        let mut interface = 0;
        (table().boot.locate_protocol)(&Self::guid(), 0, &mut interface).result()?;
        Ok(Self::new(unsafe { &mut *(interface as *mut Inner) }))
    }

    fn handle(handle: Handle) -> EfiResult<Self> {
        let mut interface = 0;
        (table().boot.handle_protocol)(handle, &Self::guid(), &mut interface).result()?;
        Ok(Self::new(unsafe { &mut *(interface as *mut Inner) }))
    }

    fn locate_handles() -> EfiResult<Vec<Self>> {
        let mut handles = Vec::with_capacity(256);
        let mut len = 256 * core::mem::size_of::<Handle>();
        (table().boot.locate_handle)(
            SearchType::SearchByProtocol,
            &Self::guid(),
            0,
            &mut len,
            handles.as_mut_ptr(),
        )
        .result()?;
        unsafe {
            handles.set_len(len / core::mem::size_of::<Handle>());
        }
        let mut instances = Vec::new();
        for handle in handles {
            if let Ok(ins) = Self::handle(handle) {
                instances.push(ins)
            }
        }
        Ok(instances)
    }

    fn get_all() -> Vec<Self> {
        Self::locate_handles().unwrap_or_default()
    }
}
