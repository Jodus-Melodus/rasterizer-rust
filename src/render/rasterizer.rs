use vector_2d_3d::Vector2D;

use crate::render::types::{Color, FrameBufferSize};

pub struct Screen {
    frame_buffer: Vec<Color>,
    frame_buffer_size: FrameBufferSize,
}

impl Screen {
    pub fn new(frame_buffer_size: FrameBufferSize) -> Self {
        Screen {
            frame_buffer: vec![Color::BLACK; frame_buffer_size.width * frame_buffer_size.height],
            frame_buffer_size,
        }
    }

    pub fn frame_buffer(&self) -> Vec<u32> {
        self.frame_buffer
            .iter()
            .copied()
            .map(|color| color.to_u32())
            .collect()
    }

    pub fn draw_point(&mut self, point: Vector2D, color: Color) {
        let offset = Vector2D::from_coord(
            self.frame_buffer_size.width as f32 / 2.0,
            self.frame_buffer_size.height as f32 / 2.0,
        );

        let offset_point = point + offset;
        self.frame_buffer
            [(self.frame_buffer_size.width as f32 * offset_point.y + offset_point.x) as usize] =
            color;
    }
}

pub fn point_in_triangle_1(point: Vector2D, triangle_points: [Vector2D; 3]) -> bool {
    let a = triangle_points[0];
    let b = triangle_points[1];
    let c = triangle_points[2];
    let p = point;

    let ab_perpendicular = (b - a).perpendicular_cw();
    let bc_perpendicular = (c - b).perpendicular_cw();
    let ca_perpendicular = (a - c).perpendicular_cw();

    let dot_product_ab_ap = ab_perpendicular.dot_product(&(p - a));
    let dot_product_bc_bp = bc_perpendicular.dot_product(&(p - b));
    let dot_product_ca_bp = ca_perpendicular.dot_product(&(p - c));

    ((dot_product_ab_ap > 0.0) == (dot_product_bc_bp > 0.0))
        && ((dot_product_bc_bp > 0.0) == (dot_product_ca_bp > 0.0))
}
