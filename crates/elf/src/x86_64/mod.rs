// https://en.wikipedia.org/wiki/Executable_and_Linkable_Format

#[repr(C)]
#[derive(Debug)]
pub struct Header<Arch> {
    pub magic: [u8; 4],
    pub class: u8,
    pub endianness: u8,
    pub abi: u8,
    _unused: [u8; 7],
    pub ty: u16,
    pub machine: u16,
    pub version: u32,
    pub entry: Arch,
    pub phoff: Arch,
    pub shoff: Arch,
    pub flags: u32,
    pub ehsize: u16,
    pub phentsize: u16,
    pub phnum: u16,
    pub shentsize: u16,
    pub shnum: u16,
    pub shstrndx: u16,
}
