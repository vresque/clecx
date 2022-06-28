#![no_std]
#![feature(abi_efiapi)]
#![feature(alloc_error_handler)]
#![feature(allocator_api)]
#![feature(arbitrary_enum_discriminant)]
pub mod status;
pub mod types;
use core::fmt::{self, Write};

extern crate alloc;
pub use alloc::*;
use raw::{graphics::GraphicsOutputProtocol, table::Table};
pub use types::*;
pub mod allocator;
pub mod cstr;
pub mod file;
pub mod loaded_image;
pub mod macros;
pub mod panic;
pub mod proto;
pub mod raw;
pub use cstr::*;

static mut HANDLE: Handle = 0;
static mut TABLE: *mut Table = addr!(mut 0);

pub fn handle() -> Handle {
    unsafe { HANDLE }
}

pub fn table() -> &'static Table {
    unsafe { &*TABLE }
}

pub fn setup(table: *mut Table, handle: Handle) {
    unsafe {
        TABLE = table;
        HANDLE = handle;
    }
}

pub fn table_mut() -> &'static mut Table {
    unsafe { &mut *TABLE }
}

pub struct Writer;
impl Write for Writer {
    fn write_str(&mut self, string: &str) -> Result<(), core::fmt::Error> {
        let tab = table();

        for c in string.chars() {
            let _ = (tab.con_out.output_string)(tab.con_out, WString([c as u16, 0].as_ptr()));
            if c == '\n'
            // EFI: Newline == \n\r
            {
                let _ =
                    (tab.con_out.output_string)(tab.con_out, WString(['\r' as u16, 0].as_ptr()));
            }
        }

        Ok(())
    }
}

pub fn __io_print(args: fmt::Arguments) {
    Writer.write_fmt(args).unwrap()
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::__io_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (efi::print!("\n"));
    ($fmt:expr) => (efi::print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (efi::print!(concat!($fmt, "\n"), $($arg)*));
}

#[macro_export]
macro_rules! debugln {
    () => (crate::print!("\n"));
    ($fmt:expr) => (crate::print!(concat!("[EFILIB]", $fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (crate::print!(concat!("[EFILIB] ", $fmt, "\n"), $($arg)*));
}

wrap_proto!(GraphicsOutput wraps GraphicsOutputProtocol; GRAPHICS_OUTPUT_PROTOCOL_GUID);
