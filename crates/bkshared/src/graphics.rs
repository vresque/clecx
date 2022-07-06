pub const PSF1_MAGIC: [u8; 2] = [0x36, 0x04];
pub const PSF1_DRAW_MASK: usize = 0b10000000;
pub const PSF1_PIXELS_PER_CHARACTER: usize = 16;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum PixelFormat {
    RGBReserved8BitPerColor,
    BGRReserved8BitPerColor,
    BitMask,   /* Physical framebuffer */
    BltOnly,   /* No physical frambuffer */
    FormatMax, /* Last format entry */
}

#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct Framebuffer {
    pub format: PixelFormat,
    pub base: usize,
    pub size: usize,
    pub width: u64,
    pub height: u64,
    pub stride: u64,
}

impl Framebuffer {
    pub const fn buffer(&self) -> &'static [u32] {
        unsafe { core::slice::from_raw_parts(self.base as *mut u32, self.size) }
    }

    /// SAFETY: As this is a mutable borrow, it should be asserted that two cannot access this at the same time
    pub fn buffer_mut(&mut self) -> &'static mut [u32] {
        unsafe {
            core::slice::from_raw_parts_mut(
                self.base as *mut u32,
                (self.width * self.stride) as usize,
            )
        }
    }
}

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub struct Psf1Header {
    pub magic: [u8; 2],
    pub mode: u8,
    pub charsize: u8,
}

#[repr(packed)]
#[derive(Default, Copy, Clone)]
pub struct Psf1Font {
    pub header: Psf1Header,
    pub buffer: usize,
    pub buffer_size: usize,
}

impl Psf1Font {
    pub fn glyphs(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.buffer as *mut u8, self.buffer_size) }
    }
}
