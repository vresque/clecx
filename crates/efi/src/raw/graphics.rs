use crate::{status::Status, UInt32, UInt8, UIntN};

#[repr(C)]
pub struct Pixel {
    pub blue: UInt8,
    pub green: UInt8,
    pub red: UInt8,
    pub reserved: UInt8,
}

#[repr(C)]
pub enum GraphicsOperation {
    // Fill with data from first pixel
    Fill,
    // Copy from display to buffer
    DisplayToBuffer,
    // Copy from buffer to to display
    BufferToDisplay,
    // From display -> display
    DisplayToDisplay,
}

pub use bkshared::graphics::PixelFormat;

#[repr(C)]
pub struct PixelBitmask {
    pub rmask: UInt32,
    pub gmask: UInt32,
    pub bmask: UInt32,
    pub reserved: UInt32,
}

#[repr(C)]
pub struct GOPInfo {
    pub version: UInt32,
    pub horizontal_res: UInt32,
    pub vertical_res: UInt32,
    pub format: PixelFormat,
    pub info: PixelBitmask,
    pub stride: UInt32,
}

#[repr(C)]
pub struct GOPMode {
    pub max: UInt32,
    pub mode: UInt32,
    pub info: &'static GOPInfo,
    pub info_size: UIntN,
    pub base: UIntN,
    pub size: UIntN,
}

#[repr(C)]
pub struct GraphicsOutputProtocol {
    pub query_mode: extern "win64" fn(
        &mut GraphicsOutputProtocol,
        UInt32,
        &mut UIntN,
        &mut *mut GOPInfo,
    ) -> Status,
    pub set_mode: extern "win64" fn(&mut GraphicsOutputProtocol, UInt32) -> Status,
    pub blt: extern "win64" fn(
        &mut GraphicsOutputProtocol,
        *mut Pixel,
        GraphicsOperation,
        UIntN,
        UIntN,
        UIntN,
        UIntN,
        UIntN,
        UIntN,
    ) -> Status,
    pub mode: &'static mut GOPMode,
}
