use bkshared::mmap::ComplexMemoryType;

use crate::{table, UIntN};
use core::{
    alloc::{GlobalAlloc, Layout},
    ptr,
};

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

pub struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut ptr = 0;
        let result =
            (table().boot.allocate_pool)(ComplexMemoryType::LoaderData, layout.size(), &mut ptr)
                .result();

        match result {
            Ok(_) => ptr as *mut u8,
            Err(_) => ptr::null_mut(),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _: Layout) {
        (table().boot.free_pool)(ptr as UIntN);
    }
}

#[alloc_error_handler]
pub fn alloc_error(_: core::alloc::Layout) -> ! {
    panic!("Allocation error occured");
}
