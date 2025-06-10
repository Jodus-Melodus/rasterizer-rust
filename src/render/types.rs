use std::ops::{Add, Mul};

use rand::random_range;

pub type M3x3 = [[f32; 3]; 3];

pub fn x_rotation_matrix(theta: f32) -> M3x3 {
    [
        [1.0, 0.0, 0.0],
        [0.0, theta.cos(), -theta.sin()],
        [0.0, theta.sin(), theta.cos()],
    ]
}
pub fn y_rotation_matrix(theta: f32) -> M3x3 {
    [
        [theta.cos(), 0.0, theta.sin()],
        [0.0, 1.0, 0.0],
        [-theta.sin(), 0.0, theta.cos()],
    ]
}
pub fn z_rotation_matrix(theta: f32) -> M3x3 {
    [
        [theta.cos(), -theta.sin(), 0.0],
        [theta.sin(), theta.cos(), 0.0],
        [0.0, 0.0, 1.0],
    ]
}

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

    pub fn random() -> Self {
        Color {
            r: random_range(0..=255),
            g: random_range(0..=255),
            b: random_range(0..=255),
        }
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
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vector2 { x, y }
    }
}

impl Add for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Mul<f32> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: f32) -> Self::Output {
        Vector2::new(self.x * rhs, self.y * rhs)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { x, y, z }
    }
}

impl Add for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: f32) -> Self::Output {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

#[derive(Clone, Copy)]
pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub z: f32,
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

impl Mul<Vector3> for M3x3 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Self::Output {
        let x = self[0][0] * rhs.x + self[0][1] * rhs.y + self[0][2] * rhs.z;
        let y = self[1][0] * rhs.x + self[1][1] * rhs.y + self[1][2] * rhs.z;
        let z = self[2][0] * rhs.x + self[2][1] * rhs.y + self[2][2] * rhs.z;
        Vector3::new(x, y, z)
    }
}

impl Mul<M3x3> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: M3x3) -> Self::Output {
        let x = rhs[0][0] * self.x + rhs[0][1] * self.y + rhs[0][2] * self.z;
        let y = rhs[1][0] * self.x + rhs[1][1] * self.y + rhs[1][2] * self.z;
        let z = rhs[2][0] * self.x + rhs[2][1] * self.y + rhs[2][2] * self.z;
        Vector3::new(x, y, z)
    }
}
