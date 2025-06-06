use rayon::prelude::*;

use crate::render::types::{Color, FrameBufferSize, Vector2};

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

    fn draw_point(&mut self, point: Vector2, color: Color) {
        let offset = Vector2::new(
            (self.frame_buffer_size.width / 2) as isize,
            (self.frame_buffer_size.height / 2) as isize,
        );

        let offset_point = point + offset;
        let index =
            (self.frame_buffer_size.width as isize * offset_point.y + offset_point.x) as usize;

        self.frame_buffer[index] = color.to_u32();
    }

    pub fn draw_triangle(&mut self, triangle_points: [Vector2; 3], color: Color) {
        let width = self.frame_buffer_size.width as isize;
        let height = self.frame_buffer_size.height as isize;
        let offset = Vector2::new(
            (self.frame_buffer_size.width / 2) as isize,
            (self.frame_buffer_size.height / 2) as isize,
        );
        let min_x = triangle_points
            .iter()
            .map(|v| v.x)
            .fold(isize::MAX, isize::min)
            .clamp(-offset.x, offset.x);
        let max_x = triangle_points
            .iter()
            .map(|v| v.x)
            .fold(isize::MIN, isize::max)
            .clamp(-offset.x, offset.x);
        let min_y = triangle_points
            .iter()
            .map(|v| v.y)
            .fold(isize::MAX, isize::min)
            .clamp(-offset.y, offset.y);
        let max_y = triangle_points
            .iter()
            .map(|v| v.y)
            .fold(isize::MIN, isize::max)
            .clamp(-offset.y, offset.y);

        let a = &triangle_points[0];
        let b = &triangle_points[1];
        let c = &triangle_points[2];
        let e0 = (b.y - a.y, a.x - b.x, b.x * a.y - a.x * b.y);
        let e1 = (c.y - b.y, b.x - c.x, c.x * b.y - b.x * c.y);
        let e2 = (a.y - c.y, c.x - a.x, a.x * c.y - c.x * a.y);
        let color = color.to_u32();

        let pixel_writes: Vec<(usize, u32)> = (min_y..=max_y)
            .into_par_iter()
            .flat_map_iter(|y| {
                let mut writes = Vec::new();
                for x in min_x..=max_x {
                    let point = Vector2::new(x, y);
                    let w0 = e0.0 * x + e0.1 * y + e0.2;
                    let w1 = e1.0 * x + e1.1 * y + e1.2;
                    let w2 = e2.0 * x + e2.1 * y + e2.2;

                    if w0 >= 0 && w1 >= 0 && w2 >= 0 {
                        let offset_point = point + offset;
                        let ix = offset_point.x as isize;
                        let iy = offset_point.y as isize;
                        if ix >= 0 && iy >= 0 && ix < width && iy < height {
                            let index = (width * iy + ix) as usize;
                            writes.push((index, color));
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
