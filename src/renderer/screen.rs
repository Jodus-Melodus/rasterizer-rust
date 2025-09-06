use crate::renderer::{
    types::Color,
    vector::{Vector2, Vector3},
};

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

    pub fn draw_triangle(&mut self, a: Vector2, b: Vector2, c: Vector2, color: Color) {
        let max_x = a.x.max(b.x.max(c.x)).ceil() as isize;
        let min_x = a.x.min(b.x.min(c.x)).floor() as isize;
        let max_y = a.y.max(b.y.max(c.y)).ceil() as isize;
        let min_y = a.y.min(b.y.min(c.y)).floor() as isize;

        for y in min_y..max_y {
            for x in min_x..max_x {
                let p = Vector2::new(x as f32, y as f32);

                if calculate_barycentric_coordinates(p, a, b, c) {
                    self.set(x, y, color);
                }
            }
        }
    }
}

fn calculate_barycentric_coordinates(p: Vector2, a: Vector2, b: Vector2, c: Vector2) -> bool {
    let denominator = (b.y - c.y) * (a.x - c.x) + (c.x - b.x) * (a.y - c.y);
    let u = ((b.y - c.y) * (p.x - c.x) + (c.x - b.x) * (p.y - c.y)) / denominator;
    let v = ((c.y - a.y) * (p.x - c.x) + (a.x - c.x) * (p.y - c.y)) / denominator;
    let w = 1.0 - u - v;
    return (u >= 0.0) && (v >= 0.0) && (w >= 0.0);
}

pub fn project_coordinate(p: Vector3, focal_length: f32) -> Vector2 {
    let denominator = focal_length + p.z;
    if denominator == 0.0 {
        panic!("Devision by 0");
    }
    let projected_x = (focal_length * p.x) / denominator;
    let projected_y = (focal_length * p.y) / denominator;
    Vector2::new(projected_x, projected_y)
}
