use bkshared::{graphics::Psf1Font, rsdp::RsdpManager, Handover, Kernel};
use efi::{
    print, println,
    raw::runtime::ResetType,
    status::{EfiResult, Status},
    table,
};

use crate::arch::x86_64::{
    loader::load_font,
    mmap::locate_memory_map,
    rsdp::find_rsdps,
    stack::{create_stack, switch_stack},
};
use bkshared::PHYSICAL_OFFSET;
mod alloc;
mod loader;
mod paging;
use loader::load_kernel;
use paging::enable_paging;
mod framebuffer;
mod rsdp;
mod stack;
use framebuffer::find_framebuffer;
mod mmap;

pub fn main() -> EfiResult<!> {
    framebuffer::make_max_mode(table().con_out);

    println!("Launching architecture specific booting process...");
    println!("Finding all RSDP pointers");
    let rsdp = find_rsdps()?;
    let (kernel_bytes, kernel_entry) = load_kernel("clecx")?;
    let kernel_base = kernel_bytes.as_ptr() as u64;
    let kernel_size = kernel_bytes.len();
    println!(
        "Found Clecx Kernel: Allocated at {:x?}, size of {} bytes and entry at {:x?}",
        kernel_base, kernel_size, kernel_entry
    );
    let kernel = Kernel::new(kernel_base, kernel_size, kernel_entry);
    let stack = create_stack()?;

    let framebuffer = find_framebuffer()?;
    println!("{:?}", framebuffer);

    let mgr = RsdpManager {
        base: rsdp.as_ptr() as u64,
        size: rsdp.len(),
    };
    let stack_base = stack.base;
    println!("Loading the font");
    let font = load_font("font.psf")?;

    println!("Paging...");
    let mut pager = enable_paging(&kernel)?;

    println!("Mapping the Framebuffer");
    pager.map_framebuffer(&framebuffer);
    println!("Mapping the Font");
    pager.map_font(&font);

    unsafe {
        println!("Activating paging");
        core::arch::asm!("cli");
        pager.activate();
    }
    println!("Entering kernel...");
    // - No more printing from here on
    let (mmap, mut boot) = locate_memory_map()?;

    boot.exit();
    boot.set_vaddr_map(PHYSICAL_OFFSET);
    core::mem::forget(boot);


    let mut handover = Handover {
        magic: 0xC1EC7,
        framebuffer,
        font,
        kernel,
        stack,
        rsdp: mgr,
        mmap,
    };




    //switch_stack(stack_base);
    let kmain: extern "sysv64" fn(*mut Handover) -> ! =
        unsafe { core::mem::transmute(kernel_entry) };

    kmain(&mut handover);

    loop {}
}
