use bkshared::mmap::MemoryMap;

use crate::prepare_dump;
use crate::println;
use bkshared::PAGE_SIZE;

pub fn dump_memory_map(mmap: MemoryMap) {
    prepare_dump! {
        dumping "MemoryMap";
        || {
           for member in mmap.members {
                println!(dump: "{:?} [{:?}]: {}mb (at {:#x?})", member.ty, member.complex, member.size * PAGE_SIZE as u64 / 1024 / 1024, member.base);
           } 
        }
    }
} 