pub struct Resolution {
    pub width: u64,
    pub height: u64,
    pub stride: u64,
}

impl Resolution {
    pub const fn new(width: u64, height: u64, stride: u64) -> Self {
        Self {
            width,
            height,
            stride,
        }
    }
}
