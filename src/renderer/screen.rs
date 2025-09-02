use std::usize;

use crate::renderer::types::Color;

pub struct ScreenBuffer<const W: usize, const H: usize> {
    buffer: Vec<Vec<Color>>,
    x_offset: usize,
    y_offset: usize,
}

impl<const W: usize, const H: usize> ScreenBuffer<W, H> {
    pub fn new() -> Self {
        ScreenBuffer {
            buffer: vec![vec![Color::default(); W]; H],
            x_offset: W / 2,
            y_offset: H / 2,
        }
    }

    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        // center around (0; 0) and add 1 because arrays are 0 index based
        self.buffer[y + self.y_offset + 1][x + self.x_offset + 1] = color;
    }
}
