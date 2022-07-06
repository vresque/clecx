use core::mem::size_of;

use bkshared::{
    graphics::{Psf1Font, Psf1Header, PSF1_MAGIC},
    MIB, PAGE_SIZE,
};
use efi::{
    file::Searcher,
    println,
    status::{EfiResult, Status},
    WString,
};
use elf::Header;

use super::alloc::zeroed;

pub fn load_kernel(name: &str) -> EfiResult<(&mut [u8], u64)> {
    let mut file = Searcher::find(name)?;

    let length = file.get_file_info()?.file_size as usize;
    let ptr = zeroed(length)?;
    let kernel_bytes = unsafe { core::slice::from_raw_parts_mut(ptr as *mut u8, length) };

    for mut part in kernel_bytes.chunks_mut(MIB) {
        let count = file.read(&mut part)?;
        if count == 0 {
            break; // Reached end of file
        }

        if count != part.len() {
            return Err(Status::BadBufferSize);
        }
    }

    let elf: Header<u64> = unsafe {
        let bytes = &kernel_bytes[..size_of::<Header<u64>>()];
        core::ptr::read(bytes.as_ptr() as *const _)
    };

    if b"\x7FELF" != &elf.magic {
        println!(
            "Kernel has bad elf magic: Expected \x7FELF, found {:#X?}",
            &kernel_bytes[..4]
        );
        return Err(Status::CompromisedData);
    } else {
        println!("Verified kernel file");
    }

    println!("Loaded Kernel!");

    Ok((kernel_bytes, elf.entry))
}

pub fn load_font(path: &str) -> EfiResult<Psf1Font> {
    let mut file = Searcher::find(path)?;
    let header = unsafe {
        let hdr_size = core::mem::size_of::<Psf1Header>();
        let ptr = zeroed(hdr_size)?;
        let slice = core::slice::from_raw_parts_mut(ptr as *mut u8, hdr_size);
        file.read(slice)?;
        core::ptr::read(ptr as *const Psf1Header)
    };

    if header.magic != PSF1_MAGIC {
        return Err(Status::CompromisedData);
    }

    let glyph_buffer_size: usize = match header.mode {
        1 => header.charsize as usize * 512,
        _ => header.charsize as usize * 256,
    };

    (file.0.set_position)(&mut file.0, core::mem::size_of::<Psf1Header> as u64).result()?;
    let glyphs = unsafe {
        let ptr = zeroed(glyph_buffer_size)?;
        let slice = core::slice::from_raw_parts_mut(ptr as *mut u8, glyph_buffer_size);
        file.read(slice);
        ptr as u64
    };

    Ok(Psf1Font {
        header,
        buffer: glyphs,
        buffer_size: glyph_buffer_size,
    })
}
