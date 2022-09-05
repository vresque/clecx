use bkshared::Handover;

use crate::{
    debug::{
        color::Color,
        framebuffer::{DebugFramebuffer, FRAMEBUFFER}, welcome::{welcome, Stage},
    },
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

    #[derive(Debug)]
    pub struct Tester {
        pub a: u32,
        pub b: u64,
    }

    let mut a = Tester { a: 0, b: 100 };
    
    for i in 0..100 {
        a.a += i;
        a.b += i as u64;
        println!(error: "{:#?}", a);
    }

    welcome(Stage::Launching);
    //crate::main();
    loop {}
}
