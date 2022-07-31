pub use arch::mem::*;
pub use arch::mem::{addr::*, page::*};

use crate::arch;


extern crate compiler_builtins;

// Memset for u32
pub unsafe fn memset32(dest: *mut u8, val: u32, count: usize) {
    // TODO: Fix this function
    //use compiler_builtins::mem::memcpy;
    //let mut i = 0;
    //for _ in (0..(count & !7)).step_by(8) {
    //    memcpy(dest.add(i), val as *const u8, 8);
    //    i += 8;
    //}
    //while i < count {
    //    *dest.add(i) = *(val as *mut u8).add(i & 7);
    //}
    slice::from_raw_parts_mut(dest as *mut u32, count).fill(val)
}
