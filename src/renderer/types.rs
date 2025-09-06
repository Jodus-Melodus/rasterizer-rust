#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
    };

    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub fn to_gray(&self) -> u8 {
        let (r, g, b) = (self.r, self.g, self.b);
        let gray = ((0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) / 255.0) as u8;
        gray
    }

    pub fn to_u32(&self) -> u32 {
        let a = 0xFF;
        ((a as u32) << 24) | ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
}

impl Into<(u8, u8, u8)> for Color {
    fn into(self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
}
