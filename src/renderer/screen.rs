use rand::Rng;

use crate::renderer::{
    model::Model,
    types::{Axis, Color, Vector2, Vector3},
};

pub struct ScreenBuffer<const W: usize, const H: usize> {
    buffer: Vec<Vec<Color>>,
    depth_buffer: Vec<Vec<f32>>,
    x_offset: isize,
    y_offset: isize,
}

impl<const W: usize, const H: usize> ScreenBuffer<W, H> {
    pub fn new() -> Self {
        ScreenBuffer {
            buffer: vec![vec![Color::BLACK; W]; H],
            depth_buffer: vec![vec![f32::INFINITY; W]; H],
            x_offset: (W / 2) as isize,
            y_offset: (H / 2) as isize,
        }
    }

    pub fn clear(&mut self) {
        self.buffer = vec![vec![Color::BLACK; W]; H];
        self.depth_buffer = vec![vec![f32::INFINITY; W]; H];
    }

    fn set(&mut self, x: isize, y: isize, color: Color) {
        let index_x = (x + self.x_offset) as usize;
        let index_y = (y + self.y_offset) as usize;

        if index_y >= H || index_x >= W {
            return;
        }

        self.buffer[index_y][index_x] = color;
    }

    fn get(&self, x: isize, y: isize) -> &Color {
        let index_x = (x + self.x_offset) as usize;
        let index_y = (y + self.y_offset) as usize;

        if index_y >= H || index_x >= W {
            panic!("Index out of bounds: x={}, y={}", index_x, index_y);
        }

        &self.buffer[index_y][index_x]
    }

    fn get_depth_buffer_index(&self, x: isize, y: isize) -> f32 {
        let index_x = (x + self.x_offset) as usize;
        let index_y = (y + self.y_offset) as usize;

        if index_y >= H || index_x >= W {
            // panic!("Index out of bounds: x={}, y={}", index_x, index_y);
            return f32::INFINITY;
        }

        self.depth_buffer[index_y][index_x]
    }

    fn set_depth_buffer(&mut self, x: isize, y: isize, depth: f32) {
        let index_x = (x + self.x_offset) as usize;
        let index_y = (y + self.y_offset) as usize;

        if index_y >= H || index_x >= W {
            return;
        }

        self.depth_buffer[index_y][index_x] = depth;
    }

    pub fn screen_resolution(&self) -> (usize, usize) {
        (W, H)
    }

    pub fn display(&self) -> String {
        let mut result = String::new();
        for y in -self.y_offset..self.y_offset {
            for x in -self.x_offset..self.x_offset {
                result.push_str(&self.get(x, y).display());
            }
            result.push('\n');
        }
        result
    }

    fn draw_triangle(&mut self, a: &Vector3, b: &Vector3, c: &Vector3, color: &Color) {
        let max_x = a.x.max(b.x.max(c.x)).ceil() as isize;
        let min_x = a.x.min(b.x.min(c.x)).floor() as isize;
        let max_y = a.y.max(b.y.max(c.y)).ceil() as isize;
        let min_y = a.y.min(b.y.min(c.y)).floor() as isize;
        let near = 0.1;
        let far = 100.0;

        let a2 = Vector2::new(a.x, a.y);
        let b2 = Vector2::new(b.x, b.y);
        let c2 = Vector2::new(c.x, c.y);

        for y in min_y..max_y {
            for x in min_x..max_x {
                let p = Vector2::new(x as f32, y as f32);
                let (u, v, w) = calculate_barycentric_coordinates(&p, &a2, &b2, &c2);

                if (u >= 0.0) && (v >= 0.0) && (w >= 0.0) {
                    let depth = normalize_depth(u * a.z + v * b.z + w * c.z, near, far);

                    if depth < self.get_depth_buffer_index(x, y) {
                        self.set(x, y, *color);
                        self.set_depth_buffer(x, y, depth);
                    }
                }
            }
        }
    }

    pub fn draw_model(&mut self, model: &Model, focal_length: f32) {
        let mut rng = rand::rng();
        for (face_index1, face_index2, face_index3) in model.faces.iter() {
            let vertex1 = model.vertices[*face_index1];
            let vertex2 = model.vertices[*face_index2];
            let vertex3 = model.vertices[*face_index3];

            let (projected_vertex1, projected_vertex2, projected_vertex3) = (
                project_coordinate(&vertex1, focal_length),
                project_coordinate(&vertex2, focal_length),
                project_coordinate(&vertex3, focal_length),
            );
            let color = Color::new(
                rng.random_range(0..=255),
                rng.random_range(0..=255),
                rng.random_range(0..=255),
            );

            self.draw_triangle(
                &projected_vertex1,
                &projected_vertex2,
                &projected_vertex3,
                &color,
            );
        }
    }
}

fn calculate_barycentric_coordinates(
    p: &Vector2,
    a: &Vector2,
    b: &Vector2,
    c: &Vector2,
) -> (f32, f32, f32) {
    let denominator = (b.y - c.y) * (a.x - c.x) + (c.x - b.x) * (a.y - c.y);
    let u = ((b.y - c.y) * (p.x - c.x) + (c.x - b.x) * (p.y - c.y)) / denominator;
    let v = ((c.y - a.y) * (p.x - c.x) + (a.x - c.x) * (p.y - c.y)) / denominator;
    let w = 1.0 - u - v;
    (u, v, w)
}

fn project_coordinate(p: &Vector3, focal_length: f32) -> Vector3 {
    let mut denominator = focal_length + p.z;
    if denominator == 0.0 {
        denominator = 0.00001;
    }
    Vector3::new(
        (focal_length * p.x) / denominator,
        -(focal_length * p.y) / denominator,
        p.z,
    )
}

fn normalize_depth(z: f32, near: f32, far: f32) -> f32 {
    (z - near) / (far - near)
}

pub fn rotate_model(model: &mut Model, rotation_axis: Axis, theta: f32) {
    let (sin_theta, cos_theta) = (theta.sin(), theta.cos());

    for vertex in &mut model.vertices {
        match rotation_axis {
            Axis::X => {
                let y = vertex.y * cos_theta - vertex.z * sin_theta;
                let z = vertex.y * sin_theta + vertex.z * cos_theta;
                vertex.y = y;
                vertex.z = z;
            }
            Axis::Y => {
                let x = vertex.x * cos_theta + vertex.z * sin_theta;
                let z = -vertex.x * sin_theta + vertex.z * cos_theta;
                vertex.x = x;
                vertex.z = z;
            }
            Axis::Z => {
                let x = vertex.x * cos_theta - vertex.y * sin_theta;
                let y = vertex.x * sin_theta + vertex.y * cos_theta;
                vertex.x = x;
                vertex.y = y;
            }
        }
    }
}
