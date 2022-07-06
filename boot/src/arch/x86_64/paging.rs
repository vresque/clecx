use core::ops::{Index, IndexMut};

use bkshared::{
    graphics::{Framebuffer, Psf1Font},
    Kernel, PAGE_SIZE,
};
use efi::status::{EfiResult, Status};

use super::alloc::zeroed;

#[derive(PartialEq, Eq)]
pub enum PagerLevel {
    PML4,
    PDP,
    PD,
    PT,
    P,
}

impl PagerLevel {
    pub fn next(&self) -> EfiResult<Self> {
        match *self {
            Self::PML4 => Ok(Self::PDP),
            Self::PDP => Ok(Self::PD),
            Self::PD => Ok(Self::PT),
            Self::PT => Ok(Self::P),
            Self::P => Err(Status::Success),
        }
    }
}

pub struct Pager {
    level: PagerLevel,
    buffer: &'static mut [u64], // Length: 512
}

impl Pager {
    pub fn new(level: PagerLevel) -> EfiResult<Pager> {
        Ok(Self {
            level,
            buffer: unsafe { core::slice::from_raw_parts_mut(zeroed(PAGE_SIZE)? as *mut u64, 512) },
        })
    }

    pub fn next(&self) -> EfiResult<Pager> {
        Ok(Self::new(self.level.next()?)?)
    }

    pub fn placable(&self) -> u64 {
        self.buffer.as_ptr() as u64 | 2 /* (1 << 1) */ | 1
    }

    pub fn place_pager(&mut self, at: usize, pager: &Pager) {
        assert!(at <= 512);
        self.buffer[at] = pager.placable();
    }

    pub fn place_addr(&mut self, at: usize, addr: u64) {
        assert!(at <= 512);
        self.buffer[at] = addr;
    }

    pub fn figure_out_address(pdp_idx: usize, pd_idx: usize, pt_idx: usize, offset: usize) -> u64 {
        (pdp_idx as u64 * 0x4000_0000
            + pd_idx as u64 * 0x20_0000
            + pt_idx as u64 * 0x1000
            + offset as u64)
            | 1 << 1
            | 1
    }

    pub fn figure_out_2mib_address(pdp_idx: usize, pd_idx: usize, offset: usize) -> u64 {
        (pdp_idx as u64 * 0x4000_0000 + pd_idx as u64 * 0x20_0000 + offset as u64)
            | 1 << 7
            | 1 << 1
            | 1
    }

    pub fn map_gib_with_offset_2mib_pages(
        &self,
        mut pdp: Pager,
        gib: usize,
        offset: usize,
    ) -> EfiResult<()> {
        for pdp_idx in 0..8 {
            let mut pd = pdp.next()?;
            pdp.place_pager(pdp_idx, &pd);
            for pd_idx in 0..pd.buffer.len() {
                let addr = Self::figure_out_2mib_address(pdp_idx, pd_idx, 0);
                pd.place_addr(pd_idx, addr);
            }
        }

        Ok(())
    }
    pub fn map_8gib(&mut self) -> EfiResult<()> {
        assert!(self.level == PagerLevel::PML4);
        let pdp = self.next()?;
        assert!(pdp.level == PagerLevel::PDP);

        self.place_pager(0, &pdp);
        self.place_pager(256, &pdp);

        Self::map_gib_with_offset_2mib_pages(&self, pdp, 8, 0)?;

        Ok(())
    }

    pub fn map_kernel(&mut self, kernel: &Kernel) -> EfiResult<()> {
        assert!(self.level == PagerLevel::PML4);
        let mut pdp = self.next()?;
        self.place_pager(510, &pdp);

        let mut mapped = 0;
        let mut idx = 0;
        while mapped < kernel.size && idx < pdp.buffer.len() {
            let mut pd = pdp.next()?;
            pdp.place_pager(idx, &pd);
            idx += 1;

            let mut pd_idx = 0;
            while mapped < kernel.size && pd_idx < pd.buffer.len() {
                let mut pt = pd.next()?;
                pd.place_pager(pd_idx, &pt);
                pd_idx += 1;

                let mut pt_idx = 0;
                while mapped < kernel.size && pt_idx < pt.buffer.len() {
                    let addr = (kernel.base + mapped as u64) | 1 << 1 | 1;
                    pt.place_addr(pt_idx, addr);
                    pt_idx += 1;
                    mapped += PAGE_SIZE;
                }
            }
        }
        assert!(mapped >= kernel.size);

        Ok(())
    }

    /// # Activate the pager
    /// This is not very well-written code
    /// Much of this needs to be documented better,
    /// and many of these values should be constants (eg: the efer msr value)
    pub unsafe fn activate(self) {
        use core::arch::asm;
        assert!(self.level == PagerLevel::PML4);

        // The following flags are activated:
        // OS_XSAVE
        // ENABLE_SSE
        // ENABLE_GLOBAL_PAGES
        // ENABLE_PAE
        // ENABLE_PSE
        let flags = 0x402b0;
        let mut cr4 = {
            let retval: usize;
            asm!("mov {}, cr4", out(reg) retval);
            retval
        };
        cr4 |= flags;

        asm!("mov cr4, {}", in(reg) cr4);

        // Enable long mode and nx
        let ia_efer = 0xc0000080u32;
        let mut efer = {
            let (hi, lo): (u32, u32);
            asm!("rdmsr", out("rax") lo, out("edx") hi, in("ecx") ia_efer, options(nomem, nostack, preserves_flags));
            ((hi as u64) << 32) | (lo as u64)
        };
        efer |= 1 << 11 | 1 << 8;
        let (hi, lo) = (((efer >> 32) as u32), efer as u32);
        asm!("wrmsr", in("ecx") ia_efer, in("eax") lo, in("edx") hi);

        // Write the page table
        asm!("mov cr3, {}", in(reg) (self.ptr()));

        // The following flags are enabled:
        // ENABLE_PAGING
        // WRITE_PROTECT
        // PROTECTED_MODE
        let cr0_flags = 0x80010001;

        let mut cr0 = {
            let retval: usize;
            asm!("mov {}, cr0", out(reg) retval);
            retval
        };

        cr0 |= cr0_flags;

        asm!("mov cr0, {}", in(reg) cr0);
    }

    pub fn ptr(&self) -> u64 {
        self.buffer.as_ptr() as u64
    }

    pub fn map_font(&mut self, font: &Psf1Font) -> EfiResult<()> {
        assert!(self.level == PagerLevel::PML4);

        if font.buffer as u64 + font.buffer_size as u64 <= 0x2_0000_0000 {
            return Ok(()); /* Framebuffer already mapped */
        }
        assert!(font.buffer % 0x20_0000 == 0);
        let pml4_idx = ((font.buffer / 0x80_0000_0000) + 256) as usize;
        let mut pdp_idx = ((font.buffer % 0x80_0000_0000) / 0x4000_0000) as usize;
        let mut pd_idx = ((font.buffer % 0x4000_0000) / 0x20_0000) as usize;

        let mut pdp = if self[pml4_idx] == 0 {
            let pdp = self.next()?;
            self.place_pager(pml4_idx, &pdp);
            pdp
        } else {
            unsafe {
                Self {
                    level: PagerLevel::PDP,
                    buffer: (core::slice::from_raw_parts_mut(
                        (self[pml4_idx] & 0x000F_FFFF_FFFF_F000) as *mut u64,
                        512,
                    )),
                }
            }
        };

        let mut sum = 0;
        while sum < font.buffer_size && pdp_idx < pdp.buffer.len() {
            let mut pd = pdp.next()?;
            assert!(pdp[pdp_idx] == 0);
            pdp.place_pager(pdp_idx, &pd);
            while sum < font.buffer_size && pd_idx < pd.buffer.len() {
                let addr = font.buffer as u64 + sum as u64;
                assert!(pd[pd_idx] == 0);
                pd[pd_idx] = addr as u64 | 1 << 7 | 1 << 1 | 1;
                sum += 0x20_0000;
                pd_idx += 1;
            }

            pdp_idx += 1;
            pd_idx = 0;
        }

        assert!(sum >= font.buffer_size);
        Ok(())
    }

    pub fn map_framebuffer(&mut self, framebuffer: &Framebuffer) -> EfiResult<()> {
        assert!(self.level == PagerLevel::PML4);

        if framebuffer.base as u64 + framebuffer.size as u64 <= 0x2_0000_0000 {
            return Ok(()); /* Framebuffer already mapped */
        }
        assert!(framebuffer.base % 0x20_0000 == 0);
        let pml4_idx = ((framebuffer.base / 0x80_0000_0000) + 256) as usize;
        let mut pdp_idx = ((framebuffer.base % 0x80_0000_0000) / 0x4000_0000) as usize;
        let mut pd_idx = ((framebuffer.base % 0x4000_0000) / 0x20_0000) as usize;

        let mut pdp = if self[pml4_idx] == 0 {
            let pdp = self.next()?;
            self.place_pager(pml4_idx, &pdp);
            pdp
        } else {
            unsafe {
                Self {
                    level: PagerLevel::PDP,
                    buffer: (core::slice::from_raw_parts_mut(
                        (self[pml4_idx] & 0x000F_FFFF_FFFF_F000) as *mut u64,
                        512,
                    )),
                }
            }
        };

        let mut sum = 0;
        while sum < framebuffer.size && pdp_idx < pdp.buffer.len() {
            let mut pd = pdp.next()?;
            assert!(pdp[pdp_idx] == 0);
            pdp.place_pager(pdp_idx, &pd);
            while sum < framebuffer.size && pd_idx < pd.buffer.len() {
                let addr = framebuffer.base as u64 + sum as u64;
                assert!(pd[pd_idx] == 0);
                pd[pd_idx] = addr as u64 | 1 << 7 | 1 << 1 | 1;
                sum += 0x20_0000;
                pd_idx += 1;
            }

            pdp_idx += 1;
            pd_idx = 0;
        }

        assert!(sum >= framebuffer.size);
        Ok(())
    }

    pub fn pml4() -> EfiResult<Self> {
        let mut p = Pager::new(PagerLevel::PML4)?;
        // Recursive map
        // We cannot use place_pager here as then, two different borrows would occur
        p[511] = p.placable();
        Ok(p)
    }
}

impl Index<usize> for Pager {
    type Output = u64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.buffer[index]
    }
}

impl IndexMut<usize> for Pager {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buffer[index]
    }
}

pub fn enable_paging(kernel: &Kernel) -> EfiResult<Pager> {
    let mut pml4 = Pager::pml4()?;
    pml4.map_8gib()?;
    pml4.map_kernel(kernel)?;

    Ok(pml4)
}
