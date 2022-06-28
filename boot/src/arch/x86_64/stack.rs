use bkshared::{Stack, PAGE_SIZE, PHYSICAL_OFFSET, STACK_SIZE};
use efi::status::EfiResult;

use super::alloc::zeroed;

pub fn create_stack() -> EfiResult<Stack> {
    let addr = zeroed(STACK_SIZE)?;
    Ok(Stack::new(addr as u64, STACK_SIZE))
}

#[inline(always)]
pub fn switch_stack(stack: u64) {
    unsafe { core::arch::asm!("mov rsp, {}", in(reg) (stack as u64)) }
}
