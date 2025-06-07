use std::ops::{Add, Mul};

#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub const BLACK: Color = Color::new(0, 0, 0);
    pub const RED: Color = Color::new(255, 0, 0);
    pub const GREEN: Color = Color::new(0, 255, 0);
    pub const BLUE: Color = Color::new(0, 0, 255);

    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub fn to_u32(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | self.b as u32
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8, u8)) -> Self {
        Color {
            r: value.0,
            g: value.1,
            b: value.2,
        }
    }
}

#[derive(Clone)]
pub struct FrameBufferSize {
    pub width: usize,
    pub height: usize,
}

impl FrameBufferSize {
    pub fn new(width: usize, height: usize) -> Self {
        FrameBufferSize { width, height }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vector2 {
    pub x: isize,
    pub y: isize,
}

impl Vector2 {
    pub fn new(x: isize, y: isize) -> Self {
        Vector2 { x, y }
    }
}

impl Add for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Mul<isize> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: isize) -> Self::Output {
        Vector2::new(self.x * rhs, self.y * rhs)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vector3 {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Vector3 {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Vector3 { x, y, z }
    }
}

impl Add for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Mul<isize> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: isize) -> Self::Output {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

#[derive(Clone, Copy)]
pub struct Camera {
    pub x: isize,
    pub y: isize,
    pub z: isize,
    pub fov: f32,
}

impl Camera {
    pub fn new(position: Vector3, fov: f32) -> Self {
        Camera {
            x: position.x,
            y: position.y,
            z: position.z,
            fov,
        }
    }
}
