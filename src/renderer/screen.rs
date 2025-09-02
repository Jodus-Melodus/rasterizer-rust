use crate::renderer::types::Color;

const GRADIENT: [char; 10] = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];

fn get_ascii_gradient_value(color: Color) -> char {
    let (r, g, b) = color;
    let gray = 0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32;
    let normalized = gray / 255.0;
    let index = (normalized * (GRADIENT.len() - 1) as f32).round() as usize;
    GRADIENT[index]
}

pub struct ScreenBuffer<const W: usize, const H: usize> {
    buffer: Vec<Vec<Color>>,
    x_offset: isize,
    y_offset: isize,
}

impl<const W: usize, const H: usize> ScreenBuffer<W, H> {
    pub fn new() -> Self {
        ScreenBuffer {
            buffer: vec![vec![Color::default(); W]; H],
            x_offset: (W / 2) as isize,
            y_offset: (H / 2) as isize,
        }
    }

    pub fn set(&mut self, x: isize, y: isize, color: Color) {
        // center around (0; 0) and add 1 because arrays are 0 index based
        self.buffer[(y + self.y_offset) as usize][(x + self.x_offset) as usize] = color;
    }

    pub fn get(&self, x: isize, y: isize) -> Color {
        self.buffer[(y + self.y_offset) as usize][(x + self.x_offset) as usize]
    }

    pub fn get_screen_resolution(&self) -> (usize, usize) {
        (W, H)
    }

    pub fn ascii(&self) -> String {
        let mut result = String::new();
        for y in -self.y_offset..self.y_offset {
            for x in -self.x_offset..self.x_offset {
                result.push(get_ascii_gradient_value(self.get(x, y)));
            }
            result.push('\n');
        }
        result
    }
}
