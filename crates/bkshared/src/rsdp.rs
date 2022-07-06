#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct Rsdp {
    pub signature: [u8; 8], // "RSD PTR "
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub revision: u8,
    pub rsdt: u32,
    pub length: u32,
    pub xsdt: u64,
    pub xchecksum: u8,
    _rsvd: [u8; 3],
}

#[derive(Debug)]
pub enum RsdpError {
    InvalidSignature,
    BadChecksum,
    BadExtendedChecksum,
    NotV2,
}

#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct RsdpWrapper(u64, usize);

impl RsdpWrapper {
    /// New creates a Wrapper without checking anything
    /// Use RsdpWrapper::validate to validate the address and get the
    /// pointer if it is valid
    pub unsafe fn new(addr: u64, len: usize) -> Self {
        Self(addr, len)
    }

    pub fn addr(&self) -> u64 {
        self.0
    }
    pub fn len(&self) -> usize {
        self.1
    }

    pub fn rsdp(&self) -> &'static Rsdp {
        unsafe { (self.0 as *const Rsdp).as_ref::<'static>().unwrap() }
    }

    pub fn bytes(&self) -> &'static [u8] {
        unsafe { core::slice::from_raw_parts(self.0 as *mut u8, core::mem::size_of::<Rsdp>()) }
    }

    pub fn validate(addr: u64) -> Result<Self, RsdpError> {
        let mut me = Self(addr, 0);
        let rsdp = me.rsdp();
        let bytes = me.bytes();

        if rsdp.signature != *b"RSD PTR " {
            return Err(RsdpError::InvalidSignature);
        }

        if rsdp.revision != 2 {
            return Err(RsdpError::NotV2);
        } // TODO: Should we accept RSDP v1?

        let mut sum: u8 = 0;
        for i in &bytes[..20] {
            sum = sum.wrapping_add(*i); // u8s overflow easily
        }

        if sum != 0 {
            return Err(RsdpError::BadChecksum);
        }

        // This is only available if it is a v2 rsdp
        let mut xsum: u8 = 0;
        for i in bytes {
            xsum = xsum.wrapping_add(*i);
        }

        if xsum != 0 {
            return Err(RsdpError::BadExtendedChecksum);
        }

        // rsdp.length if revision = 2, otherwise size_of::<Rsdp>()
        let len = rsdp.length;
        me.1 = len as usize;

        Ok(me)
    }
}

///  # RsdpManager
///  Keeps track of an array of RsdpWrappers
#[repr(packed)]
#[derive(Copy, Clone)]
pub struct RsdpManager {
    pub base: u64,
    pub size: usize,
}
