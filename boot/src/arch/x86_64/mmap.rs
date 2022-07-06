use bkshared::mmap::{ComplexMemoryType, MemoryType};
use bkshared::mmap::{MemoryDescriptor, MemoryMap};
use bkshared::PAGE_SIZE;
use efi::format;
use efi::raw::runtime::EfiMemoryDescriptor;
use efi::status::EfiResult;
use efi::vec::Vec;
use efi::{handle, println, table, vec};

pub struct BootloaderMMap {
    map: efi::vec::Vec<u8>,
    key: usize,
    descr_size: usize,
    version: u32,
    iterator_idx: usize,
}

impl BootloaderMMap {
    pub fn exit(&self) {
        let han = handle();
        (table().boot.exit_boot_services)(han, self.key)
            .result()
            .expect("Failed to exit boot services");
    }

    pub fn set_vaddr_map(&mut self, off: u64) {
        for i in 0..self.map.len() / self.descr_size {
            let descriptor = unsafe {
                &mut *(self.map.as_mut_ptr().add(i * self.descr_size) as *mut EfiMemoryDescriptor)
            };
            if descriptor.attribute & 0x8000000000000000 /* Runtime */== 0x8000000000000000 {
                descriptor.virt_start.0 = descriptor.phys_start.0 + off;
            }
        }

        (table().runtime.set_virt_addr_map)(
            self.map.len(),
            self.descr_size,
            self.version,
            self.map.as_ptr() as *const EfiMemoryDescriptor,
        )
        .result()
        .expect("Failed to set virt addr map");
    }
}

impl Iterator for BootloaderMMap {
    type Item = MemoryDescriptor;
    fn next(&mut self) -> Option<Self::Item> {
        if self.iterator_idx < self.map.len() / self.descr_size {
            let descriptor: EfiMemoryDescriptor = unsafe {
                core::ptr::read(
                    (self.map.as_ptr().add(self.iterator_idx * self.descr_size))
                        as *const EfiMemoryDescriptor,
                )
            };
            self.iterator_idx += 1;
            let right_type = MemoryType::from(descriptor.ty);
            Some(MemoryDescriptor {
                base: descriptor.phys_start.0,
                size: descriptor.count * PAGE_SIZE as u64,
                ty: right_type,
                complex: descriptor.ty,
            })
        } else {
            None
        }
    }
}

pub fn locate_memory_map() -> EfiResult<(MemoryMap, BootloaderMMap)> {
    println!("Locating memory map");
    let mut mem = vec![0; 65536];
    let mut sz = mem.len();
    let mut key = 0;
    let mut descriptor_size = 0;
    let mut version = 0;
    (table().boot.get_memory_map)(
        &mut sz,
        mem.as_mut_ptr() as *mut EfiMemoryDescriptor,
        &mut key,
        &mut descriptor_size,
        &mut version,
    )
    .result()?;

    assert!(descriptor_size >= core::mem::size_of::<EfiMemoryDescriptor>());
    mem.truncate(sz);

    let mut boot = BootloaderMMap {
        map: mem,
        key,
        descr_size: descriptor_size,
        version,
        iterator_idx: 0,
    };

    let mut map = [MemoryDescriptor {
        complex: ComplexMemoryType::Reserved,
        ty: MemoryType::Null,
        base: 0,
        size: 0,
    }; 512];

    while let Some(i) = boot.next() {
        map[boot.iterator_idx] = i;
    }

    boot.iterator_idx = 0;

    let mmap = MemoryMap { members: map };
    println!("Found memory map");
    Ok((mmap, boot))
}
