use core::panic::PanicInfo;

use crate::println;

#[panic_handler]
pub fn panic_handler(info: &PanicInfo) -> ! {
    println!("EFI-Panic: {:#?}", info);
    loop {}
}

mod efi {
    pub use crate::*;
}
