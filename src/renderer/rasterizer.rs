use std::{collections::HashMap, f32};

use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use crate::renderer::{
    model::{Model, TextureMap},
    types::{
        matrices::M3x3,
        vertices::{barycentric, Vertex2, Vertex3, Vertex3Key},
        Camera, CameraKey, Color, FrameBufferSize, TextureCoordinate,
    },
};

pub struct Screen {
    frame_buffer: Vec<u32>,
    frame_buffer_size: FrameBufferSize,
    depth_buffer: Vec<f32>,
    projection_cache: HashMap<(Vertex3Key, CameraKey), (Vertex2, f32)>,
}

impl Screen {
    pub fn new(frame_buffer_size: FrameBufferSize) -> Self {
        let (width, height) = (frame_buffer_size.width, frame_buffer_size.height);
        Screen {
            frame_buffer: vec![0; width * height],
            frame_buffer_size,
            depth_buffer: vec![f32::MAX; width * height],
            projection_cache: HashMap::new(),
        }
    }

    pub fn frame_buffer(&self) -> &Vec<u32> {
        &self.frame_buffer
    }

    pub fn clear(&mut self) {
        self.frame_buffer.par_iter_mut().for_each(|p| *p = 0);
        self.depth_buffer.par_iter_mut().for_each(|d| *d = f32::MAX);
    }

    fn point_in_triangle(a: Vertex2, b: Vertex2, c: Vertex2, p: Vertex2) -> bool {
        let sign = |v1: Vertex2, v2: Vertex2, v3: Vertex2| {
            (v1.x - v3.x) * (v2.y - v3.y) - (v2.x - v3.x) * (v1.y - v3.y)
        };

        let d1 = sign(p, a, b);
        let d2 = sign(p, b, c);
        let d3 = sign(p, c, a);

        let has_neg = (d1 < 0.0) || (d2 < 0.0) || (d3 < 0.0);
        let has_pos = (d1 > 0.0) || (d2 > 0.0) || (d3 > 0.0);

        !(has_neg && has_pos)
    }

    fn project_point(&mut self, p: Vertex3, camera: Camera) -> (Vertex2, f32) {
        let key = (Vertex3Key::from(p), CameraKey::from(camera));

        if let Some(&projected_point) = self.projection_cache.get(&key) {
            projected_point
        } else {
            let px = p.x - camera.x;
            let py = p.y - camera.y;
            let pz = p.z - camera.z;

            let focal_length =
                self.frame_buffer_size.height as f32 / (2.0 * (camera.fov / 2.0).tan());

            let x_proj = px * focal_length / pz;
            let y_proj = -py * focal_length / pz;

            let projection = (Vertex2::new(x_proj, y_proj), pz);
            self.projection_cache.insert(key, projection);
            projection
        }
    }

    fn project_triangle(&mut self, triangle: [Vertex3; 3], camera: Camera) -> [(Vertex2, f32); 3] {
        triangle.map(|p| self.project_point(p, camera))
    }

    fn get_index(&mut self, p: Vertex2) -> Option<usize> {
        let width = self.frame_buffer_size.width as isize;
        let height = self.frame_buffer_size.height as isize;

        let x = (p.x.round() as isize) + width / 2;
        let y = (p.y.round() as isize) + height / 2;

        if x >= 0 && x < width && y >= 0 && y < height {
            Some((y * width + x) as usize)
        } else {
            None
        }
    }

    fn draw_triangle(
        &mut self,
        triangle: [(Vertex2, f32); 3],
        texture_coordinate: Option<[TextureCoordinate; 3]>,
        texture_map: &mut Option<TextureMap>,
    ) {
        let a = triangle[0];
        let b = triangle[1];
        let c = triangle[2];

        let min_x = a.0.x.min(b.0.x).min(c.0.x) as isize;
        let min_y = a.0.y.min(b.0.y).min(c.0.y) as isize;
        let max_x = a.0.x.max(b.0.x).max(c.0.x) as isize;
        let max_y = a.0.y.max(b.0.y).max(c.0.y) as isize;

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let p = Vertex2::new(x as f32, y as f32);
                if let Some(index) = self.get_index(p) {
                    let depth_weights = barycentric(a.0, b.0, c.0, p);
                    let depths = Vertex3::from((a.1, b.1, c.1));
                    let depth: f32 = depths.dot(depth_weights);

                    if depth < self.depth_buffer[index] && depth > 0.0 {
                        if !Self::point_in_triangle(a.0, b.0, c.0, p) {
                            continue;
                        }

                        let color = get_color(texture_coordinate, texture_map, a, b, c, p);
                        self.frame_buffer[index] = color;
                        self.depth_buffer[index] = depth;
                    }
                }
            }
        }
    }

    pub fn draw_model(&mut self, model: &mut Model, camera: Camera) {
        let texture_map = &mut model.texture_map;
        let vertices = &model.mesh.vertices;
        let vertex_indices = &model.mesh.vertex_indices;
        let texture_coordinates = &model.mesh.texture_coordinates;
        let texture_coordinate_indices = &model.mesh.texture_coordinate_indices;

        for ((v1, v2, v3), (vt1, vt2, vt3)) in vertex_indices.iter().zip(texture_coordinate_indices)
        {
            let triangle = [vertices[*v1], vertices[*v2], vertices[*v3]];
            let projected_triangle = self.project_triangle(triangle, camera);
            let tex_coords = [
                texture_coordinates[*vt1],
                texture_coordinates[*vt2],
                texture_coordinates[*vt3],
            ];
            self.draw_triangle(projected_triangle, Some(tex_coords), texture_map);
        }
    }

    pub fn rotate_model(&self, model: &mut Model, rotation: (usize, usize, usize), theta: f32) {
        let x_rotate = M3x3::x_rotation_matrix(theta);
        let y_rotate = M3x3::y_rotation_matrix(theta);
        let z_rotate = M3x3::z_rotation_matrix(theta);

        if rotation.0 != 0 {
            for vertex in &mut model.mesh.vertices {
                *vertex = *vertex * x_rotate;
            }
        }
        if rotation.1 != 0 {
            for vertex in &mut model.mesh.vertices {
                *vertex = *vertex * y_rotate;
            }
        }
        if rotation.2 != 0 {
            for vertex in &mut model.mesh.vertices {
                *vertex = *vertex * z_rotate;
            }
        }
    }
}

fn get_color(
    texture_coordinate: Option<[TextureCoordinate; 3]>,
    texture_map: &mut Option<TextureMap>,
    a: (Vertex2, f32),
    b: (Vertex2, f32),
    c: (Vertex2, f32),
    p: Vertex2,
) -> Color {
    if let (Some(tex_coords), Some(tex_map)) = (texture_coordinate, texture_map) {
        let weights = barycentric(a.0, b.0, c.0, p);
        if weights.x < 0.0 || weights.y < 0.0 || weights.z < 0.0 {
            return Color::MAX;
        }

        let z0 = a.1;
        let z1 = b.1;
        let z2 = c.1;

        let one_over_z0 = 1.0 / z0;
        let one_over_z1 = 1.0 / z1;
        let one_over_z2 = 1.0 / z2;

        let u_over_z = tex_coords[0].u * one_over_z0 * weights.x
            + tex_coords[1].u * one_over_z1 * weights.y
            + tex_coords[2].u * one_over_z2 * weights.z;

        let v_over_z = tex_coords[0].v * one_over_z0 * weights.x
            + tex_coords[1].v * one_over_z1 * weights.y
            + tex_coords[2].v * one_over_z2 * weights.z;

        let one_over_z =
            one_over_z0 * weights.x + one_over_z1 * weights.y + one_over_z2 * weights.z;

        let u = u_over_z / one_over_z;
        let v = v_over_z / one_over_z;

        let interpolated = TextureCoordinate::new(u, v);
        tex_map.get_pixel(interpolated).unwrap_or(Color::MAX)
    } else {
        Color::MAX
    }
}
