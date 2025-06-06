use std::sync::{Arc, Mutex};

use rayon::prelude::*;
use vector_2d_3d::Vector2D;

use crate::render::types::{Color, FrameBufferSize};

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

    fn draw_point(&mut self, point: Vector2D, color: Color) {
        let offset = Vector2D::from_coord(
            self.frame_buffer_size.width as f32 / 2.0,
            self.frame_buffer_size.height as f32 / 2.0,
        );

        let offset_point = point + offset;
        let index =
            (self.frame_buffer_size.width as f32 * offset_point.y + offset_point.x) as usize;

        self.frame_buffer[index] = color.to_u32();
    }

    pub fn draw_triangle(&mut self, triangle_points: [Vector2D; 3], color: Color) {
        let min_x = triangle_points
            .iter()
            .map(|v| v.x)
            .fold(f32::INFINITY, f32::min) as isize;
        let max_x = triangle_points
            .iter()
            .map(|v| v.x)
            .fold(f32::NEG_INFINITY, f32::max) as isize;
        let min_y = triangle_points
            .iter()
            .map(|v| v.y)
            .fold(f32::INFINITY, f32::min) as isize;
        let max_y = triangle_points
            .iter()
            .map(|v| v.y)
            .fold(f32::NEG_INFINITY, f32::max) as isize;

        let width = self.frame_buffer_size.width as isize;
        let height = self.frame_buffer_size.height as isize;
        let offset = Vector2D::from_coord(
            self.frame_buffer_size.width as f32 / 2.0,
            self.frame_buffer_size.height as f32 / 2.0,
        );

        let pixel_writes: Vec<(usize, u32)> = (min_x..=max_x)
            .into_par_iter()
            .flat_map_iter(|x| {
                let mut writes = Vec::new();
                for y in min_y..=max_y {
                    let point = Vector2D::from_coord(x as f32, y as f32);
                    if point_in_triangle_1(point, triangle_points) {
                        let offset_point = point + offset;
                        let ix = offset_point.x as isize;
                        let iy = offset_point.y as isize;
                        if ix >= 0 && iy >= 0 && ix < width && iy < height {
                            let index = (width * iy + ix) as usize;
                            writes.push((index, color.to_u32()));
                        }
                    }
                }
                writes
            })
            .collect();

        for (index, value) in pixel_writes {
            self.frame_buffer[index] = value;
        }
    }
}

fn point_in_triangle_1(point: Vector2D, triangle_points: [Vector2D; 3]) -> bool {
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
