mod rasterizer;
use minifb::{Key, Window, WindowOptions};
use rasterizer::Image;
use std::{io::Result, thread::sleep, time::Duration};
use vector_2d_3d::{Vector2D, Vector3D};

use crate::rasterizer::{COLORS, CUBE, PYRAMID, TRIANGLE};

fn main() -> Result<()> {
    let width = 512;
    let height = 512;
    let mut img = Image::new(width, height);
    let mut window = Window::new("Rasterizer", width, height, WindowOptions::default()).unwrap();

    let points = PYRAMID;

    let points = points
        .iter()
        .map(|(x, y, z)| Vector3D::from_coord(*x, *y, *z))
        .collect::<Vec<Vector3D>>();

    let focal_length = 250.0;
    let start_time = std::time::Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        img.clear();
        let camera = Vector3D::from_coord(0.0, 0.0, 50.0);

        let angle = start_time.elapsed().as_secs_f32() * 0.5;
        let cos_a = angle.cos();
        let sin_a = angle.sin();

        let points: Vec<Vector3D> = points
            .iter()
            .map(|p| {
                Vector3D::from_coord(p.x * cos_a - p.z * sin_a, p.y, p.x * sin_a + p.z * cos_a)
            })
            .collect();

        let projected_points = points
            .iter()
            .map(|point| Image::project_3d_to_2d(&img, *point, camera, focal_length))
            .collect::<Vec<Option<Vector2D>>>();

        for i in 1..projected_points.len() - 1 {
            if let (Some(p0), Some(p1), Some(p2)) = (
                projected_points[0],
                projected_points[i],
                projected_points[i + 1],
            ) {
                img.draw_triangle4(p0, p1, p2, COLORS[i % COLORS.len()]);
            }
        }

        window
            .update_with_buffer(&img.to_u32_buffer(), width, height)
            .unwrap();

        sleep(Duration::from_millis(50));
    }

    Ok(())
}
