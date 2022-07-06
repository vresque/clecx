use core::mem::MaybeUninit;

use bkshared::graphics::{Framebuffer, Psf1Font};
use sync::Mutex;

use super::{color::Color, resolution::Resolution};

pub static mut FRAMEBUFFER: Mutex<Option<DebugFramebuffer>> = Mutex::new(None);



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
        Self { framebuffer, font, column: 0, row: 0, background: background.into(), foreground: foreground.into(), bytes_per_pixel }
    }


    pub const fn resolution(&self) -> Resolution {
        Resolution::new(self.framebuffer.width, self.framebuffer.height, self.framebuffer.pixels_per_scanline)
    }

    pub unsafe fn clear_screen<T>(&mut self, color: T)
    where
        T: Into<u32>,
    {
        let resolution = self.resolution();
        let color = color.into();


        for vertical in 0..resolution.height {
            let line = {
                let base = self.framebuffer.base + (vertical * (self.framebuffer.pixels_per_scanline * self.bytes_per_pixel as u64));
                core::slice::from_raw_parts_mut(base as *mut u32, self.framebuffer.pixels_per_scanline as usize * self.bytes_per_pixel as usize)
            };
            line.into_iter().for_each(|e| *e = color)
        }
    }
}