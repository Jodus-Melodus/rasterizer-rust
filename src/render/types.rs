use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    ops::{Add, Mul},
};

use rand::random_range;

#[derive(Clone, Copy)]
pub struct M3x3([[f32; 3]; 3]);

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
pub struct Vertex2 {
    pub x: f32,
    pub y: f32,
}

impl Vertex2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vertex2 { x, y }
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

impl Mul<Vertex3> for M3x3 {
    type Output = Vertex3;
    fn mul(self, rhs: Vertex3) -> Self::Output {
        let x = self.0[0][0] * rhs.x + self.0[0][1] * rhs.y + self.0[0][2] * rhs.z;
        let y = self.0[1][0] * rhs.x + self.0[1][1] * rhs.y + self.0[1][2] * rhs.z;
        let z = self.0[2][0] * rhs.x + self.0[2][1] * rhs.y + self.0[2][2] * rhs.z;
        Vertex3::new(x, y, z)
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

#[derive(Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex3>,
    pub vertex_indices: Vec<(usize, usize, usize)>,
    pub texture_coordinate_indices: Vec<(usize, usize, usize)>,
    pub vertex_normal_indices: Vec<(usize, usize, usize)>,
}

impl Mesh {
    pub fn new(
        vertices: Vec<Vertex3>,
        vertex_indices: Vec<(usize, usize, usize)>,
        texture_coordinate_indices: Vec<(usize, usize, usize)>,
        vertex_normal_indices: Vec<(usize, usize, usize)>,
    ) -> Self {
        Mesh {
            vertices,
            vertex_indices,
            texture_coordinate_indices,
            vertex_normal_indices,
        }
    }

    pub fn load_from_file(path: &str) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut vertices = Vec::new();
        let mut vertex_indices = Vec::new();
        let mut texture_coordinate_indices = Vec::new();
        let mut vertex_normal_indices = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let mut parts = line.split_ascii_whitespace();
            if let Some(line_type) = parts.next() {
                match line_type {
                    "v" => {
                        let coords: Vec<f32> = parts.map(|p| p.parse().unwrap()).collect();
                        if coords.len() == 3 {
                            vertices.push(Vertex3::new(coords[0], coords[1], coords[2]));
                        } else {
                            eprintln!("Unknown vertex syntax: {}", line);
                        }
                    }
                    "f" => {
                        let face_indices: Vec<Vec<Option<usize>>> = parts
                            .map(|f| {
                                let mut indices = f.split('/');
                                let v = indices
                                    .next()
                                    .and_then(|s| s.parse().ok())
                                    .map(|i: usize| i - 1);
                                let vt = indices.next().and_then(|s| {
                                    if !s.is_empty() {
                                        s.parse().ok().map(|i: usize| i - 1)
                                    } else {
                                        None
                                    }
                                });
                                let vn = indices.next().and_then(|s| {
                                    if !s.is_empty() {
                                        s.parse().ok().map(|i: usize| i - 1)
                                    } else {
                                        None
                                    }
                                });
                                vec![v, vt, vn]
                            })
                            .collect();

                        for i in 1..face_indices.len() - 1 {
                            if let (Some(v0), Some(v1), Some(v2)) = (
                                face_indices[0][0],
                                face_indices[i][0],
                                face_indices[i + 1][0],
                            ) {
                                vertex_indices.push((v0, v1, v2));
                            }
                            if let (Some(t0), Some(t1), Some(t2)) = (
                                face_indices[0][1],
                                face_indices[i][1],
                                face_indices[i + 1][1],
                            ) {
                                texture_coordinate_indices.push((t0, t1, t2));
                            }
                            if let (Some(n0), Some(n1), Some(n2)) = (
                                face_indices[0][2],
                                face_indices[i][2],
                                face_indices[i + 1][2],
                            ) {
                                vertex_normal_indices.push((n0, n1, n2));
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(Mesh::new(
            vertices,
            vertex_indices,
            texture_coordinate_indices,
            vertex_normal_indices,
        ))
    }
}
