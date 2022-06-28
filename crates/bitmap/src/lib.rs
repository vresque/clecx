#![no_std]
use core::ops::{Index, IndexMut};

use kresult::{Error, Result};

pub struct Bitmap<'buffer> {
    pub buffer: &'buffer mut [u8],
}

impl<'buffer> Bitmap<'buffer> {
    pub fn new(buffer: &'buffer mut [u8]) -> Self {
        Self { buffer }
    }

    pub fn set(&mut self, index: usize, value: bool) -> Result<()> {
        if index > self.buffer.len() * 8 {
            return Err(Error::OutOfBounds);
        }
        let byte_index = index / 8;
        let bit_index = index % 8;
        let indexer = 0b10000000 >> bit_index;
        *&mut self.buffer[byte_index] &= !indexer;
        if value {
            *&mut self.buffer[byte_index] |= indexer;
        }
        Ok(())
    }
}

impl<'a> Index<usize> for Bitmap<'a> {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        if index > self.buffer.len() * 8 {
            return &false;
        }

        let byte_index: u64 = (index / 8) as u64;
        let bit_index: u8 = (index % 8) as u8;
        let indexer = 0b10000000 >> bit_index;
        if (self.buffer[byte_index as usize] & indexer) > 0 {
            &true
        } else {
            &false
        }
    }
}
