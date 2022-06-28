#![no_std]
#![feature(const_slice_from_raw_parts)]

use graphics::{Framebuffer, Psf1Font};
use mmap::MemoryMap;
use rsdp::{RsdpManager, RsdpWrapper};

pub mod graphics;
pub mod memory;
pub mod mmap;
pub mod rsdp;

pub const PAGE_SIZE: usize = 0x1000;
pub const STACK_SIZE: usize = 128 * KIB;
pub const PHYSICAL_OFFSET: u64 = 0xFFFF_8000_0000_0000;

pub const KIB: usize = 1024;
pub const MIB: usize = KIB * KIB;

#[repr(packed)]
#[derive(Copy, Clone)]
pub struct Kernel {
    pub base: u64,
    pub size: usize,
    pub entry: u64,
}

impl Kernel {
    pub fn new(base: u64, size: usize, entry: u64) -> Self {
        Self { base, size, entry }
    }
}

#[repr(packed)]
#[derive(Copy, Clone)]
pub struct Stack {
    pub base: u64,
    pub size: usize,
}

impl Stack {
    pub fn new(base: u64, size: usize) -> Self {
        Self { base, size }
    }
}

#[repr(packed)]
#[derive(Copy, Clone)]
pub struct Handover {
    pub magic: u64, // 0xC1EC7
    pub framebuffer: Framebuffer,
    pub font: Psf1Font,
    pub kernel: Kernel,
    pub stack: Stack,
    pub rsdp: RsdpManager,
    pub mmap: MemoryMap,
}
