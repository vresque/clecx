use core::mem::MaybeUninit;

use bkshared::graphics::{
    Framebuffer, Psf1Font, LINE_HEIGHT, LINE_WIDTH, PSF1_DRAW_MASK, PSF1_PIXELS_PER_CHARACTER,
};
use sync::Mutex;

use crate::mem::memset32;

use super::{color::Color, resolution::Resolution};

pub static mut FRAMEBUFFER: Mutex<Option<DebugFramebuffer>> = Mutex::new(None);
extern crate compiler_builtins;

pub struct DebugFramebuffer {
    framebuffer: Framebuffer,
    font: Psf1Font,
    column: u32,
    row: u32,
    background: u32,
    foreground: u32,
    bytes_per_pixel: u8,
}

impl DebugFramebuffer {
    pub fn new<ColorTy1, ColorTy2>(
        framebuffer: Framebuffer,
        font: Psf1Font,
        background: ColorTy1,
        foreground: ColorTy2,
        bytes_per_pixel: u8,
    ) -> Self
    where
        ColorTy1: Into<u32>,
        ColorTy2: Into<u32>,
    {
        Self {
            framebuffer,
            font,
            column: 0,
            row: 0,
            background: background.into(),
            foreground: foreground.into(),
            bytes_per_pixel,
        }
    }

    pub const fn resolution(&self) -> Resolution {
        Resolution::new(
            self.framebuffer.width,
            self.framebuffer.height,
            self.framebuffer.stride,
        )
    }

    pub fn change_colors<ColorTy1, ColorTy2>(&mut self, background: ColorTy1, foreground: ColorTy2)
    where
        ColorTy1: Into<u32>,
        ColorTy2: Into<u32>,
    {
        self.background = background.into();
        self.foreground = foreground.into();
    }

    pub fn background(&self) -> u32 { self.background }
    pub fn foreground(&self) -> u32 { self.foreground }

    pub fn clear_screen<T>(&mut self, color: T)
    where
        T: Into<u32>,
    {
        let size = self.framebuffer.size;
        unsafe { memset32(self.framebuffer.base as *mut u8, color.into(), size); }

        // let resolution = self.resolution();
        // let color = color.into();

        // for vertical in 0..resolution.height {
        //     let line = unsafe {
        //         let base = self.framebuffer.base as u64
        //             + (vertical * (self.framebuffer.stride * self.bytes_per_pixel as u64));
        //         core::slice::from_raw_parts_mut(
        //             base as *mut u32,
        //             self.framebuffer.stride as usize * self.bytes_per_pixel as usize,
        //         )
        //     };
        //     line.into_iter().for_each(|e| *e = color)
        // }
    }

    pub fn print(&mut self, text: &str) {
        for chr in text.chars() {
            match chr {
                c if chr.is_ascii() => self.put_character(c),
                _ => {}
            }
        }
    }

    pub fn put_character(&mut self, character: char) {
        match character {
            '\n' | '\r' => self.draw_new_line(),
            '\t' => self.draw_tab(),
            char => {
                self.draw_ascii_character(char);
                if self.framebuffer.width <= self.column as u64 {
                    self.draw_new_line()
                } else {
                    self.column += 8;
                }
            }
        }
    }

    fn draw_new_line(&mut self) {
        self.column = 0;
        self.row += 16;

        if (self.row + 16) as u64 >= self.framebuffer.height {
            unsafe { self.scroll_one_line() }
        }
    }

    unsafe fn scroll_one_line(&mut self) {
        use compiler_builtins::mem::{memcpy, memmove, memset};
        let top_row_end = (self.framebuffer.stride as usize * LINE_HEIGHT) * 4;
        let top_row_size = (self.framebuffer.stride as usize * 4 as usize) * LINE_HEIGHT;

        let base_without_top_row = self.framebuffer.base + top_row_size;

        // Resetting the first row of the framebuffer
        memset(self.framebuffer.base as *mut u8, 0, top_row_size);
        //// Move the rest of the framebuffer to the top
        memmove(
            self.framebuffer.base as *mut u8,
            base_without_top_row as *mut u8,
            self.framebuffer.size - top_row_size,
        );
        //
        memset32(
            (self.framebuffer.base + (self.framebuffer.size - top_row_size)) as *mut u8,
            self.foreground,
            top_row_size,
        );
        self.row -= 16;
    }

    fn draw_tab(&mut self) {
        for i in 0..12 {
            self.put_character(' ');
        }
    }

    fn draw_ascii_character(&mut self, character: char) {
        let character_size = self.font.header.charsize;
        let stride = self.framebuffer.stride;
        let font = unsafe {
            let offset = (character as usize) * (character_size as usize);
            let base = self.font.buffer + offset;
            core::slice::from_raw_parts((base as *const u8), PSF1_PIXELS_PER_CHARACTER)
        };

        let framebuffer = self.framebuffer.buffer_mut();

        for (y, font_index) in (self.row..(self.row + 16)).zip(0..PSF1_PIXELS_PER_CHARACTER) {
            for x in self.column..(self.column + 8) {
                let offset = x as usize + (y as usize * stride as usize);
                if font[font_index] as usize & (PSF1_DRAW_MASK >> (x - self.column)) > 0 {
                    framebuffer[offset] = self.foreground;
                } else {
                    framebuffer[offset] = self.background;
                }
            }
        }
    }
}

impl core::fmt::Write for DebugFramebuffer {
    fn write_char(&mut self, c: char) -> core::fmt::Result {
        self.put_character(c);
        Ok(())
    }

    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! scoped_color_change {
    (background: $bg:expr; foreground: $fg:expr; body: $bdy:block) => ({
        use crate::debug::framebuffer::FRAMEBUFFER;

        
        let (foreground, background) = unsafe {
            let fg = FRAMEBUFFER.lock().as_mut().unwrap().foreground();
            (fg, FRAMEBUFFER.lock().as_mut().unwrap().background())
        };
        unsafe { FRAMEBUFFER.lock().as_mut().unwrap().change_colors($bg, $fg); }

        { $bdy }

        unsafe { FRAMEBUFFER.lock().as_mut().unwrap().change_colors(background, foreground) }
    })
}

#[macro_export]
macro_rules! kprintln {
    () => ({
        use crate::debug::framebuffer::FRAMEBUFFER;
        use core::fmt::Write;

        unsafe { FRAMEBUFFER.lock().as_mut().unwrap().write_str("\n") };
    });
    ($($arg:tt)*) => ({
        use crate::debug::framebuffer::FRAMEBUFFER;
        use core::fmt::Write;

        unsafe { FRAMEBUFFER.lock().as_mut().unwrap().write_fmt(format_args_nl!($($arg)*)) };
    })
}

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ({
        use crate::debug::framebuffer::FRAMEBUFFER;
        use core::fmt::Write;

        unsafe { FRAMEBUFFER.lock().as_mut().unwrap().write_fmt(format_args!($($arg)*)) };
    })
}

#[macro_export]
macro_rules! println {
    () => ({
        use crate::debug::framebuffer::FRAMEBUFFER;
        use core::fmt::Write;

        unsafe { FRAMEBUFFER.lock().as_mut().unwrap().write_str("\n") };
    });

    (info: $($arg:tt)*) => ({
        use crate::{kprint, kprintln, scoped_color_change};
        use crate::debug::color::Color;
        scoped_color_change! {
            background: Color::Black;
            foreground: Color::DarkGreen;
            body: {
                kprint!("info: ");
                kprintln!($($arg)*)
            }
        }
    });

    (error: $($arg:tt)*) => ({
        use crate::{kprint, kprintln, scoped_color_change};
        use crate::debug::color::Color;
        scoped_color_change! {
            background: Color::Black;
            foreground: Color::Red;
            body: {
                kprint!("error: ");
                kprintln!($($arg)*)
            }
        }
    });

    (warning: $($arg:tt)*) => ({
        use crate::{kprint, kprintln, scoped_color_change};
        use crate::debug::color::Color;
        scoped_color_change! {
            background: Color::Black;
            foreground: Color::Orange;
            body: {
                kprint!("warning: ");
                kprintln!($($arg)*)
            }
        }
    });

    (success: $($arg:tt)*) => ({
        use crate::{kprint, kprintln, scoped_color_change};
        use crate::debug::color::Color;
        scoped_color_change! {
            background: Color::Black;
            foreground: Color::LightGreen;
            body: {
                kprint!("success: ");
                kprintln!($($arg)*)
            }
        }
    });

    (fatal: $($arg:tt)*) => ({
        use crate::{kprint, kprintln, scoped_color_change};
        use crate::debug::color::Color;
        scoped_color_change! {
            background: Color::White;
            foreground: Color::Red;
            body: {
                kprint!("fatal: ");
                kprintln!($($arg)*)
            }
        }
    });

    (dump: $($arg:tt)*) => ({
        use crate::{kprint, kprintln, scoped_color_change};
        kprint!(">>: ");
        kprintln!($($arg)*)
    });

}

#[macro_export]
macro_rules! prepare_dump {
    (dumping $name:expr; || $bdy:block) => {
        use crate::{scoped_color_change, kprint, kprintln};
        use crate::debug::color::Color;

        scoped_color_change! {
            background: Color::Black;
            foreground: Color::White;
            body: {
                kprint!("---- DUMPING ");
                scoped_color_change! {
                    background: Color::Black;
                    foreground: Color::LightGreen;
                    body: {
                        kprint!("\"{}\"", $name);
                    }
                }
                kprintln!(" ----");
                { $bdy }
                kprint!("---- DUMPING OF ");
                scoped_color_change! {
                    background: Color::Black;
                    foreground: Color::LightGreen;
                    body: {
                        kprint!("\"{}\"", $name);
                    }
                }
                kprintln!(" FINISHED ----");
            }
        }
    }
}