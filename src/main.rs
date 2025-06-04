mod rasterizer;
use minifb::{Key, Window, WindowOptions};
use rasterizer::Image;
use std::{io::Result, thread::sleep, time::Duration};
use vector_2d_3d::{Vector2D, Vector3D};

fn main() -> Result<()> {
    let width = 512;
    let height = 512;
    let mut img = Image::new(width, height);
    let mut window = Window::new("Rasterizer", width, height, WindowOptions::default()).unwrap();

    let points = [
        (-200.0, 200.0, -50.0),
        (250.0, 100.0, -50.0),
        (100.0, -250.0, -50.0),
    ];
    let points = points
        .iter()
        .map(|(x, y, z)| Vector3D::from_coord(*x, *y, *z))
        .collect::<Vec<Vector3D>>();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for x in 0..500 {
            img.clear();
            let camera = Vector3D::from_coord(x as f32, 0.0, 500.0);

            let projected_points = points
                .iter()
                .map(|point| Image::project_3d_to_2d(&img, *point, camera))
                .collect::<Vec<Option<Vector2D>>>();

            for i in 1..projected_points.len() - 1 {
                if let (Some(p0), Some(p1), Some(p2)) = (
                    projected_points[0],
                    projected_points[i],
                    projected_points[i + 1],
                ) {
                    img.draw_triangle4(p0, p1, p2, Image::BLUE)
                }
            }

            window
                .update_with_buffer(&img.to_u32_buffer(), width, height)
                .unwrap();

            sleep(Duration::from_millis(30));
        }
    }

    Ok(())
}
