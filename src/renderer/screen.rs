use rand::Rng;

use crate::renderer::{
    model::Model,
    types::Color,
    vector::{Vector2, Vector3},
};

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

    pub fn clear(&mut self) {
        self.buffer = vec![vec![Color::BLACK; W]; H];
    }

    pub fn pixels(&self) -> Vec<u32> {
        self.buffer
            .iter()
            .flat_map(|row| row.iter().map(|c| c.to_u32()))
            .collect()
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
            panic!("Index out of bounds");
        }

        &self.buffer[index_y][index_x]
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

    fn draw_triangle(&mut self, a: &Vector2, b: &Vector2, c: &Vector2, color: &Color) {
        let max_x = a.x.max(b.x.max(c.x)).ceil() as isize;
        let min_x = a.x.min(b.x.min(c.x)).floor() as isize;
        let max_y = a.y.max(b.y.max(c.y)).ceil() as isize;
        let min_y = a.y.min(b.y.min(c.y)).floor() as isize;

        for y in min_y..max_y {
            for x in min_x..max_x {
                let p = Vector2::new(x as f32, y as f32);

                if calculate_barycentric_coordinates(&p, &a, &b, &c) {
                    self.set(x, y, *color);
                }
            }
        }
    }

    pub fn draw_model(&mut self, model: &Model, focal_length: f32) {
        let mut rng = rand::rng();
        for (face_index1, face_index2, face_index3) in model.faces.iter() {
            let (vertex1, vertex2, vertex3) = (
                project_coordinate(&model.vertices[*face_index1], focal_length),
                project_coordinate(&model.vertices[*face_index2], focal_length),
                project_coordinate(&model.vertices[*face_index3], focal_length),
            );
            let color = Color::new(
                rng.random_range(0..=255),
                rng.random_range(0..=255),
                rng.random_range(0..=255),
            );

            self.draw_triangle(&vertex1, &vertex2, &vertex3, &color);
        }
    }
}

fn calculate_barycentric_coordinates(p: &Vector2, a: &Vector2, b: &Vector2, c: &Vector2) -> bool {
    let denominator = (b.y - c.y) * (a.x - c.x) + (c.x - b.x) * (a.y - c.y);
    let u = ((b.y - c.y) * (p.x - c.x) + (c.x - b.x) * (p.y - c.y)) / denominator;
    let v = ((c.y - a.y) * (p.x - c.x) + (a.x - c.x) * (p.y - c.y)) / denominator;
    let w = 1.0 - u - v;
    return (u >= 0.0) && (v >= 0.0) && (w >= 0.0);
}

fn project_coordinate(p: &Vector3, focal_length: f32) -> Vector2 {
    let mut denominator = focal_length + p.z;
    if denominator == 0.0 {
        denominator = 0.00001;
    }
    let projected_x = (focal_length * p.x) / denominator;
    let projected_y = (focal_length * p.y) / denominator;
    Vector2::new(projected_x, projected_y)
}
