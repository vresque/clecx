use bkshared::Handover;

use crate::{
    debug::{
        color::Color,
        framebuffer::{DebugFramebuffer, FRAMEBUFFER},
    },
    kprintln, println,
};

#[no_mangle]
extern "sysv64" fn main(handover: *mut Handover) -> ! {
    let mut handover = unsafe { *handover };
    let mut framebuffer = DebugFramebuffer::new(
        handover.framebuffer,
        handover.font,
        Color::LightGreen,
        Color::Black,
        4,
    );
    unsafe {
        framebuffer.clear_screen(Color::Black);
        *FRAMEBUFFER.lock() = Some(framebuffer);
    }

    println!(info:" Eh");
    println!(error: "Oh no!");
    println!(warning: "Warning!");
    println!(success: "Finally!");
    println!(fatal: "This is bad");

    crate::main();
    loop {}
}
