#![no_std]
pub type Result<T> = ::core::result::Result<T, Error>;
#[derive(Debug)]

pub enum Error {
    OutOfBounds,
    OutOfMemory,
    // Not
    NotAligned { expected_alignment: u64 },
}
