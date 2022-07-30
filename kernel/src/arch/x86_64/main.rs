use bkshared::Handover;

use crate::{
    debug::{
        color::Color,
        framebuffer::{DebugFramebuffer, FRAMEBUFFER}, welcome::{welcome, Stage},
    },
    kprintln, println,
};

#[no_mangle]
extern "sysv64" fn main(handover: *mut Handover) -> ! {
    let mut handover = unsafe { *handover };
    let mut framebuffer = DebugFramebuffer::new(
        handover.framebuffer,
        handover.font,
        Color::White,
        Color::Black,
        4,
    );
    unsafe {
        framebuffer.clear_screen(Color::Black);
        *FRAMEBUFFER.lock() = Some(framebuffer);
    }

    welcome(Stage::Launching);

    crate::main();
    loop {}
}
