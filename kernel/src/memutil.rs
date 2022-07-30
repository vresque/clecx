extern crate compiler_builtins;

// Memset for u32
pub unsafe fn memset32(dest: *mut u8, val: u32, count: usize) {
    use compiler_builtins::mem::memcpy;
    let mut i = 0;
    for _ in (0..(count & !7)).step_by(8) {
        memcpy(dest.add(i), val as *const u8, 8);
        i += 8;
    }
    while i < count {
        *dest.add(i) = *(val as *mut u8).add(i & 7);
    }
}