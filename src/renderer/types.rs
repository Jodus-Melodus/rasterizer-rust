const GRADIENT: [char; 10] = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];

#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };

    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    pub fn as_char(&self) -> char {
        let (r, g, b, _a) = (self.r, self.g, self.b, self.a);
        let gray = ((0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) / 255.0) as f32;
        let index = (gray * (GRADIENT.len() - 1) as f32).round() as usize;
        GRADIENT[index]
    }

    pub fn display(&self) -> String {
        format!(
            "\x1b[38;2;{};{};{}m{}\x1b[0m",
            self.r,
            self.g,
            self.b,
            self.as_char()
        )
    }
}

#[derive(Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Clone, Copy, Debug)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

impl Point2D {
    pub fn new(x: f32, y: f32) -> Self {
        Point2D { x, y }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Point3D { x, y, z }
    }
}
