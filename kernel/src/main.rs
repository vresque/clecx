#![no_std]
#![no_main]
#![feature(format_args_nl)]
#![feature(rustc_private)]

pub mod arch;
pub mod debug;
pub mod memutil;
pub mod panic;

pub fn main() {
    welcome(Stage::Ready);
}
