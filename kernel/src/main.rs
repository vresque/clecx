#![no_std]
#![no_main]
#![feature(format_args_nl)]
#![feature(rustc_private)]
#![feature(prelude_import)]
#![feature(trace_macros)]
#![feature(concat_idents)]
#![feature(log_syntax)]
#![feature(concat_bytes)]

pub mod arch;
pub mod debug;
pub mod panic;
#[prelude_import]
pub use prelude::*;
pub mod prelude;
pub mod mem;

pub fn main() {
    debug::welcome::welcome(debug::welcome::Stage::Ready);
}
