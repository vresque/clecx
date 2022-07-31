#![no_std]
#![no_main]
#![feature(format_args_nl)]
#![feature(rustc_private)]

pub mod arch;
pub mod debug;
pub mod panic;
pub mod mem;

pub fn main() {
    debug::welcome::welcome(debug::welcome::Stage::Ready);
}
