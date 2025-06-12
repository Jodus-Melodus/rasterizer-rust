use crate::renderer::{
    model::{Model, TextureCoordinate, TextureMap},
    types::{
        vertices::{Vertex2, Vertex3},
        Camera, Color, FrameBufferSize,
    },
};

pub struct Screen {
    frame_buffer: Vec<u32>,
    frame_buffer_size: FrameBufferSize,
    depth_buffer: Vec<f32>,
}

impl Screen {
    pub fn new(frame_buffer_size: FrameBufferSize) -> Self {
        Screen {
            frame_buffer: vec![0; frame_buffer_size.width * frame_buffer_size.height],
            frame_buffer_size,
            depth_buffer: Vec::new(),
        }
    }

    pub fn frame_buffer(&self) -> Vec<u32> {
        self.frame_buffer.clone()
    }

    pub fn clear(&mut self) {
        self.frame_buffer = vec![0; self.frame_buffer_size.width * self.frame_buffer_size.height];
        self.depth_buffer =
            vec![f32::INFINITY; self.frame_buffer_size.width * self.frame_buffer_size.height];
    }

    fn point_in_triangle(a: Vertex2, b: Vertex2, c: Vertex2, p: Vertex2) -> bool {
        let ab = a - b;
        let ab_perpendicular = ab.perpendicular();
        let cp = c - p;

        ab_perpendicular.dot(cp) >= 0.0
    }

    fn project_point(&self, p: Vertex3, camera: Camera) -> (Vertex2, f32) {
        let focal_length = self.frame_buffer_size.height as f32 / (2.0 * (camera.fov / 2.0).tan());

        (
            Vertex2::new(p.x * focal_length / p.z, p.y * focal_length / p.z),
            p.z,
        )
    }

    fn project_triangle(&self, triangle: [Vertex3; 3], camera: Camera) -> [(Vertex2, f32); 3] {
        triangle.map(|p| self.project_point(p, camera))
    }

    fn get_index(&mut self, p: Vertex2) -> Option<usize> {
        let x = p.x.round() as isize;
        let y = p.y.round() as isize;
        let width = self.frame_buffer_size.width as isize;
        let height = self.frame_buffer_size.height as isize;

        if x >= 0 && x < width && y >= 0 && y < height {
            Some((y * width + x) as usize)
        } else {
            None
        }
    }

    fn draw_point(&mut self, p: Vertex2, color: Color) {
        if let Some(index) = self.get_index(p) {
            self.frame_buffer[index] = color.to_u32();
        }
    }

    fn draw_triangle(
        &mut self,
        triangle: [(Vertex2, f32); 3],
        texture_coordinate: Option<[TextureCoordinate; 3]>,
        texture_map: &Option<TextureMap>,
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

                if !Self::point_in_triangle(a.0, b.0, c.0, p) {
                    continue;
                }

                if let (Some(texture_coord), Some(texture_map)) = (texture_coordinate, texture_map)
                {
                    // calculate texture coordinate for point

                    if let Some(color) = texture_map.get_pixel(texture_coord) {
                        if let Some(index) = self.get_index(p) {
                            // calculate point depth
                            if depth < self.depth_buffer[index] {
                                self.draw_point(p, color);
                                self.depth_buffer[index] = depth;
                            }
                        }
                    }
                }
            }
        }
    }

    fn draw_model(&mut self, model: Model, camera: Camera) {
        let texture_map = model.texture_map;
        let vertices = model.mesh.vertices;
        let vertex_indices = model.mesh.vertex_indices;
        let texture_coordinates = model.mesh.texture_coordinates;
        let texture_coordinate_indices = model.mesh.texture_coordinate_indices;

        let triangles = vertex_indices
            .iter()
            .map(|(v1, v2, v3)| [vertices[*v1], vertices[*v2], vertices[*v3]])
            .collect::<Vec<_>>();

        let texture_coordinates = texture_coordinate_indices
            .iter()
            .map(|(vt1, vt2, vt3)| {
                [
                    texture_coordinates[*vt1],
                    texture_coordinates[*vt2],
                    texture_coordinates[*vt3],
                ]
            })
            .collect::<Vec<_>>();

        let projected_triangles = triangles
            .iter()
            .map(|triangle| self.project_triangle(*triangle, camera))
            .collect::<Vec<_>>();

        let triangles = projected_triangles
            .iter()
            .zip(texture_coordinates)
            .collect::<Vec<_>>();

        for (triangle, texture_coordinates) in triangles {
            self.draw_triangle(*triangle, Some(texture_coordinates), &texture_map);
        }
    }
}
