use rand::random_range;

use crate::renderer::types::vertices::Vertex3;

pub mod matrices {
    use std::ops::Mul;

    use crate::renderer::types::Vertex3;

    #[derive(Clone, Copy)]
    pub struct M3x3(pub [[f32; 3]; 3]);

    impl M3x3 {
        pub fn x_rotation_matrix(theta: f32) -> Self {
            M3x3([
                [1.0, 0.0, 0.0],
                [0.0, theta.cos(), -theta.sin()],
                [0.0, theta.sin(), theta.cos()],
            ])
        }
        pub fn y_rotation_matrix(theta: f32) -> Self {
            M3x3([
                [theta.cos(), 0.0, theta.sin()],
                [0.0, 1.0, 0.0],
                [-theta.sin(), 0.0, theta.cos()],
            ])
        }
        pub fn z_rotation_matrix(theta: f32) -> Self {
            M3x3([
                [theta.cos(), -theta.sin(), 0.0],
                [theta.sin(), theta.cos(), 0.0],
                [0.0, 0.0, 1.0],
            ])
        }
    }

    impl Mul<Vertex3> for M3x3 {
        type Output = Vertex3;
        fn mul(self, rhs: Vertex3) -> Self::Output {
            let x = self.0[0][0] * rhs.x + self.0[0][1] * rhs.y + self.0[0][2] * rhs.z;
            let y = self.0[1][0] * rhs.x + self.0[1][1] * rhs.y + self.0[1][2] * rhs.z;
            let z = self.0[2][0] * rhs.x + self.0[2][1] * rhs.y + self.0[2][2] * rhs.z;
            Vertex3::new(x, y, z)
        }
    }
}

pub mod vertices {
    use std::ops::{Add, Mul, Sub};

    use crate::renderer::types::matrices::M3x3;

    #[derive(Clone, Copy, Debug)]
    pub struct Vertex2 {
        pub x: f32,
        pub y: f32,
    }

    impl Vertex2 {
        pub fn new(x: f32, y: f32) -> Self {
            Vertex2 { x, y }
        }

        pub fn dot(self, rhs: Self) -> f32 {
            self.x * rhs.x + self.y * rhs.y
        }
    }

    impl Add for Vertex2 {
        type Output = Vertex2;
        fn add(self, rhs: Self) -> Self::Output {
            Vertex2::new(self.x + rhs.x, self.y + rhs.y)
        }
    }

    impl Mul<f32> for Vertex2 {
        type Output = Vertex2;
        fn mul(self, rhs: f32) -> Self::Output {
            Vertex2::new(self.x * rhs, self.y * rhs)
        }
    }

    impl Sub for Vertex2 {
        type Output = Vertex2;
        fn sub(self, rhs: Self) -> Self::Output {
            Vertex2::new(self.x - rhs.x, self.y - rhs.y)
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Vertex3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }

    impl Vertex3 {
        pub fn new(x: f32, y: f32, z: f32) -> Self {
            Vertex3 { x, y, z }
        }
    }

    impl Add for Vertex3 {
        type Output = Vertex3;
        fn add(self, rhs: Self) -> Self::Output {
            Vertex3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
        }
    }

    impl Mul<f32> for Vertex3 {
        type Output = Vertex3;
        fn mul(self, rhs: f32) -> Self::Output {
            Vertex3::new(self.x * rhs, self.y * rhs, self.z * rhs)
        }
    }

    impl Mul<M3x3> for Vertex3 {
        type Output = Vertex3;
        fn mul(self, rhs: M3x3) -> Self::Output {
            let x = rhs.0[0][0] * self.x + rhs.0[0][1] * self.y + rhs.0[0][2] * self.z;
            let y = rhs.0[1][0] * self.x + rhs.0[1][1] * self.y + rhs.0[1][2] * self.z;
            let z = rhs.0[2][0] * self.x + rhs.0[2][1] * self.y + rhs.0[2][2] * self.z;
            Vertex3::new(x, y, z)
        }
    }

    // Calculate barycentric coordinates of point p with respect to triangle (a, b, c)
    // Returns (alpha, beta, gamma)
    pub fn barycentric(a: Vertex2, b: Vertex2, c: Vertex2, p: Vertex2) -> (f32, f32, f32) {
        let v0 = b - a;
        let v1 = c - a;
        let v2 = p - a;

        let d00 = v0.dot(v0);
        let d01 = v0.dot(v1);
        let d11 = v1.dot(v1);
        let d20 = v2.dot(v0);
        let d21 = v2.dot(v1);

        let denom = d00 * d11 - d01 * d01;
        let v = (d11 * d20 - d01 * d21) / denom;
        let w = (d00 * d21 - d01 * d20) / denom;
        let u = 1.0 - v - w;
        (u, v, w)
    }
}

#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub const BLACK: Color = Color::new(0, 0, 0, 0);
    pub const RED: Color = Color::new(255, 0, 0, 0);
    pub const GREEN: Color = Color::new(0, 255, 0, 0);
    pub const BLUE: Color = Color::new(0, 0, 255, 0);

    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    pub fn to_u32(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | self.b as u32
    }

    pub fn random() -> Self {
        Color {
            r: random_range(0..=255),
            g: random_range(0..=255),
            b: random_range(0..=255),
            a: 255,
        }
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8, u8)) -> Self {
        Color {
            r: value.0,
            g: value.1,
            b: value.2,
            a: value.3,
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

#[derive(Clone, Copy)]
pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub fov: f32,
}

impl Camera {
    pub fn new(position: Vertex3, fov: f32) -> Self {
        Camera {
            x: position.x,
            y: position.y,
            z: position.z,
            fov,
        }
    }
}
