use minmath::linear_algebra::vector::Vector;

use crate::renderer::types::Color;

const GRADIENT: [char; 10] = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];

fn get_ascii_gradient_value(color: Color) -> char {
    let normalized = color.to_gray() as f32;
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
            buffer: vec![vec![Color::BLACK; W]; H],
            x_offset: (W / 2) as isize,
            y_offset: (H / 2) as isize,
        }
    }

    pub fn set(&mut self, x: isize, y: isize, color: Color) {
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

    pub fn draw_triangle(&mut self, a: Vector<2>, b: Vector<2>, c: Vector<2>, color: Color) {
        let max_x = a[0].max(b[0].max(c[0])).ceil() as isize;
        let min_x = a[0].min(b[0].min(c[0])).floor() as isize;
        let max_y = a[1].max(b[1].max(c[1])).ceil() as isize;
        let min_y = a[1].min(b[1].min(c[1])).floor() as isize;

        for y in min_y..max_y {
            for x in min_x..max_x {
                let p = Vector::new([x as f32, y as f32]);

                if calculate_barycentric_coordinates(p, a, b, c) {
                    self.set(x, y, color);
                }
            }
        }
    }
}

fn calculate_barycentric_coordinates(
    p: Vector<2>,
    a: Vector<2>,
    b: Vector<2>,
    c: Vector<2>,
) -> bool {
    let denominator = (b[1] - c[1]) * (a[0] - c[0]) + (c[0] - b[0]) * (a[1] - c[1]);

    let u = ((b[1] - c[1]) * (p[0] - c[0]) + (c[0] - b[0]) * (p[1] - c[1])) / denominator;

    let v = ((c[1] - a[1]) * (p[0] - c[0]) + (a[0] - c[0]) * (p[1] - c[1])) / denominator;

    let w = 1.0 - u - v;
    return (u >= 0.0) && (v >= 0.0) && (w >= 0.0);
}
