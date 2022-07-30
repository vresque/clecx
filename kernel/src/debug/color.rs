#[repr(u32)]
// Not all colours are constructed
#[allow(dead_code)]
pub enum Color {
    Black = 0x00000000,
    White = 0xffffffff,
    Red = 0xff0000,
    Orange = 0xf5a905,
    Yellow = 0xf5f505,
    LightGreen = 0x84f505,
    DarkGreen = 0x4b6430,
    Cyan = 0x306450,
    LightBlue = 0x0bc1e2,
    DarkBlue = 0x0b3ce2,
    Purple = 0x630be2,
    Pink = 0xdb0be2,
    // TODO: Custom Variant Custom(u8, u8, u8)
}

impl Into<u32> for Color {
    fn into(self) -> u32 {
        self as u32
    }
}
