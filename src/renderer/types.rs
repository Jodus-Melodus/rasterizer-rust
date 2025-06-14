use crate::renderer::types::vertices::Vertex3;

const SCALE: f32 = 1000.0;

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

    use crate::renderer::types::{matrices::M3x3, SCALE};

    #[derive(Clone, Copy)]
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

    #[derive(Clone, Copy)]
    pub struct Vertex3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }

    #[derive(Eq, PartialEq, Hash)]
    pub struct Vertex3Key {
        pub x: i32,
        pub y: i32,
        pub z: i32,
    }

    impl Vertex3 {
        pub fn new(x: f32, y: f32, z: f32) -> Self {
            Vertex3 { x, y, z }
        }

        pub fn dot(&self, other: Self) -> f32 {
            self.x * other.x + self.y * other.y + self.z * other.z
        }
    }

    impl From<(f32, f32, f32)> for Vertex3 {
        fn from(value: (f32, f32, f32)) -> Self {
            Vertex3::new(value.0, value.1, value.2)
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

    impl From<Vertex3> for Vertex3Key {
        fn from(v: Vertex3) -> Self {
            Vertex3Key {
                x: (v.x * SCALE) as i32,
                y: (v.y * SCALE) as i32,
                z: (v.z * SCALE) as i32,
            }
        }
    }

    pub fn barycentric(a: Vertex2, b: Vertex2, c: Vertex2, p: Vertex2) -> Vertex3 {
        let v0 = b - a;
        let v1 = c - a;
        let v2 = p - a;

        let d00 = &v0.dot(v0);
        let d01 = &v0.dot(v1);
        let d11 = &v1.dot(v1);
        let d20 = &v2.dot(v0);
        let d21 = &v2.dot(v1);

        let denom = d00 * d11 - d01 * d01;
        let v = (d11 * d20 - d01 * d21) / denom;
        let w = (d00 * d21 - d01 * d20) / denom;
        let u = 1.0 - v - w;
        Vertex3::new(u, v, w)
    }
}

pub type Color = u32;

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

#[derive(Eq, PartialEq, Hash)]
pub struct CameraKey {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub fov: i32,
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

impl From<Camera> for CameraKey {
    fn from(c: Camera) -> Self {
        CameraKey {
            x: (c.x * SCALE) as i32,
            y: (c.y * SCALE) as i32,
            z: (c.z * SCALE) as i32,
            fov: (c.fov * SCALE) as i32,
        }
    }
}

#[derive(Clone, Copy)]
pub struct TextureCoordinate {
    pub u: f32,
    pub v: f32,
}

impl TextureCoordinate {
    pub fn new(u: f32, v: f32) -> Self {
        TextureCoordinate { u, v }
    }
}
