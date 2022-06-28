use crate::{Bool, UInt16, UInt32, UInt8};

#[repr(C)]
#[derive(Default)]
pub struct Time {
    pub year: UInt16,
    pub month: UInt8,
    pub day: UInt8,
    pub hour: UInt8,
    pub minute: UInt8,
    pub second: UInt8,
    _reserved: u8,
    pub nanosecond: UInt32,
    pub timezone: UInt16,
    pub daylight: UInt8,
    _reserved2: u8,
}

#[repr(C)]
pub struct TimeCapabilities {
    pub resolution: UInt32,
    pub accuracy: UInt32,
    pub sets_to_zero: Bool,
}
