#[derive(Clone, Copy)]
pub struct Resolution {
    pub width: u16,
    pub height: u16,
}

impl Resolution {
    pub fn new(width: u16, height: u16) -> Self {
        Resolution { width, height }
    }
}
