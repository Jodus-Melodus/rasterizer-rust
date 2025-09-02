#[derive(Clone, Copy)]
pub enum Color {
    RGB(u8, u8, u8),
    Black,
    White,
    Red,
    Green,
    Blue,
}

impl Default for Color {
    fn default() -> Self {
        Self::Black
    }
}
