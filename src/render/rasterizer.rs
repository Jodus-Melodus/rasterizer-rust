use rayon::prelude::*;

use crate::render::{
    model::{Mesh, Model, TextureCoordinate2, TextureMap},
    types::{barycentric, Camera, Color, FrameBufferSize, M3x3, Vertex2, Vertex3},
};

pub struct Screen {
    frame_buffer: Vec<u32>,
    frame_buffer_size: FrameBufferSize,
}

impl Screen {
    pub fn new(frame_buffer_size: FrameBufferSize) -> Self {
        Screen {
            frame_buffer: vec![0; frame_buffer_size.width * frame_buffer_size.height],
            frame_buffer_size,
        }
    }

    pub fn frame_buffer(&self) -> Vec<u32> {
        self.frame_buffer.clone()
    }

    pub fn clear(&mut self) {
        self.frame_buffer = vec![0; self.frame_buffer_size.width * self.frame_buffer_size.height];
    }

    fn project(&self, point: Vertex3, camera: Camera) -> (Vertex2, f32) {
        let rel = Vertex3::new(point.x - camera.x, point.y - camera.y, point.z - camera.z);

        if rel.z <= 0.0 {
            return (Vertex2::new(0.0, 0.0), 0.0);
        }

        let aspect = self.frame_buffer_size.width as f32 / self.frame_buffer_size.height as f32;
        let f = 1.0 / (camera.fov / 2.0).tan();
        let x_ndc = (rel.x as f32 * f / aspect) / rel.z as f32;
        let y_ndc = (rel.y as f32 * f) / rel.z as f32;

        (
            Vertex2::new(
                x_ndc * (self.frame_buffer_size.width as f32 / 2.0),
                y_ndc * (self.frame_buffer_size.height as f32 / 2.0),
            ),
            point.z / rel.z,
        )
    }

    fn project_triangle(&self, triangle: [Vertex3; 3], camera: Camera) -> [(Vertex2, f32); 3] {
        [
            self.project(triangle[0], camera),
            self.project(triangle[1], camera),
            self.project(triangle[2], camera),
        ]
    }

    fn draw_triangle(
        &mut self,
        triangle: ([(Vertex2, f32); 3], &[TextureCoordinate2; 3]),
        texture_map: &TextureMap,
    ) {
        let width = self.frame_buffer_size.width as isize;
        let height = self.frame_buffer_size.height as isize;
        let offset = Vertex2::new(
            (self.frame_buffer_size.width / 2) as f32,
            (self.frame_buffer_size.height / 2) as f32,
        );
        let triangle_points = triangle.0.iter().map(|(v, _)| v).collect::<Vec<_>>();
        let texture_coords = triangle.1;
        let min_x = triangle_points
            .iter()
            .map(|v| v.x)
            .fold(f32::MAX, f32::min)
            .clamp(-offset.x, offset.x) as isize;
        let max_x = triangle_points
            .iter()
            .map(|v| v.x)
            .fold(f32::MIN, f32::max)
            .clamp(-offset.x, offset.x) as isize;
        let min_y = triangle_points
            .iter()
            .map(|v| v.y)
            .fold(f32::MAX, f32::min)
            .clamp(-offset.y, offset.y) as isize;
        let max_y = triangle_points
            .iter()
            .map(|v| v.y)
            .fold(f32::MIN, f32::max)
            .clamp(-offset.y, offset.y) as isize;

        let a = triangle_points[0].clone();
        let b = triangle_points[1].clone();
        let c = triangle_points[2].clone();
        let e0 = (b.y - a.y, a.x - b.x, b.x * a.y - a.x * b.y);
        let e1 = (c.y - b.y, b.x - c.x, c.x * b.y - b.x * c.y);
        let e2 = (a.y - c.y, c.x - a.x, a.x * c.y - c.x * a.y);

        let pixel_writes: Vec<(usize, u32)> = (min_y..=max_y)
            .into_par_iter()
            .flat_map_iter(|y| {
                let mut writes = Vec::new();
                for x in min_x..=max_x {
                    let point = Vertex2::new(x as f32, y as f32);
                    let w0 = e0.0 * x as f32 + e0.1 * y as f32 + e0.2;
                    let w1 = e1.0 * x as f32 + e1.1 * y as f32 + e1.2;
                    let w2 = e2.0 * x as f32 + e2.1 * y as f32 + e2.2;

                    if (w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0)
                        || (w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0)
                    {
                        let offset_point = point + offset;
                        let ix = offset_point.x as isize;
                        let iy = offset_point.y as isize;
                        if ix >= 0 && iy >= 0 && ix < width && iy < height {
                            let index = (width * iy + ix) as usize;
                            let (u, v, w) = barycentric(a, b, c, point);
                            let tex_coord = texture_coords[0] * u
                                + texture_coords[1] * v
                                + texture_coords[2] * w;
                            let color = texture_map.get_pixel(tex_coord).map_or(0, |c| c.to_u32());
                            writes.push((index, color));
                        }
                    }
                }
                writes
            })
            .collect();

        for (index, color) in pixel_writes {
            self.frame_buffer[index] = color;
        }
    }

    pub fn draw_shape(&mut self, shape: Model, theta: f32, camera: Camera) {
        let vertices = shape.mesh.vertices;
        let edges = shape.mesh.vertex_indices;
        let texture_coordinates = shape.mesh.texture_coordinates;
        let texture_coordinate_indices = shape.mesh.texture_coordinate_indices;
        let x_rotation_matrix = M3x3::x_rotation_matrix(theta);
        let y_rotation_matrix = M3x3::y_rotation_matrix(theta);
        let z_rotation_matrix = M3x3::z_rotation_matrix(theta);

        let rotated_points = vertices
            .iter()
            .map(|v| *v * x_rotation_matrix * y_rotation_matrix * z_rotation_matrix)
            .collect::<Vec<_>>();

        let mut triangles = Vec::new();

        for i in 0..edges.len() {
            let (i1, i2, i3) = edges[i];
            let (vt1, vt2, vt3) = texture_coordinate_indices[i];

            triangles.push((
                [rotated_points[i1], rotated_points[i2], rotated_points[i3]],
                [
                    texture_coordinates[vt1],
                    texture_coordinates[vt2],
                    texture_coordinates[vt3],
                ],
            ));
        }

        let mut projected_triangles = triangles
            .iter()
            .map(|(triangle, texture_coordinates)| {
                (
                    self.project_triangle(*triangle, camera),
                    texture_coordinates,
                )
            })
            .collect::<Vec<_>>();

        projected_triangles.sort_by(|a, b| {
            let avg_a = (a.0[0].1 + a.0[1].1 + a.0[2].1) / 3.0;
            let avg_b = (b.0[0].1 + b.0[1].1 + b.0[2].1) / 3.0;
            avg_a
                .partial_cmp(&avg_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if let Some(ref texture_map) = shape.texture_map {
            for triangle in projected_triangles.iter().rev() {
                self.draw_triangle(*triangle, texture_map);
            }
        }
    }
}
