use bkshared::Handover;

use crate::{debug::{framebuffer::{DebugFramebuffer, FRAMEBUFFER}, color::Color}, kprintln};


#[no_mangle]
extern "sysv64" fn main(handover: *mut Handover) -> ! {
    let mut handover = unsafe { *handover };
    let mut framebuffer = DebugFramebuffer::new(handover.framebuffer, handover.font, Color::DarkBlue, Color::Black, 4);
    unsafe { framebuffer.clear_screen(Color::Cyan); *FRAMEBUFFER.lock() = Some(framebuffer); }
    
    for i in 0..1000 {
        kprintln!("{i} Hello wrold");
    }
    
    
    crate::main();
    loop {}
}