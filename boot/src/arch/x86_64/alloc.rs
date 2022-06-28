use bkshared::{memory::Addr, mmap::ComplexMemoryType, PAGE_SIZE};
use efi::{status::EfiResult, table, AllocateType};

pub fn zeroed(count: usize) -> EfiResult<Addr> {
    let count = (count + PAGE_SIZE - 1) / PAGE_SIZE;

    let mut result: Addr = 0x2_0000_0000;

    (table().boot.allocate_pages)(
        AllocateType::MaxAddress,
        ComplexMemoryType::RuntimeServicesData,
        count,
        &mut result,
    )
    .result()?;
    unsafe {
        core::ptr::write_bytes(result as *mut u8, 0, PAGE_SIZE * count);
    };
    assert!(result != 0);
    Ok(result)
}
