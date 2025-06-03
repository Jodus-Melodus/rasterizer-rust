mod rasterizer;
use minifb::{Key, Window, WindowOptions};
use rasterizer::Image;
use std::io::Result;
use vector_2d_3d::Vector2D;

fn main() -> Result<()> {
    let width = 512;
    let height = 512;
    let mut img = Image::new(width, height);
    let mut window = Window::new("Rasterizer", width, height, WindowOptions::default()).unwrap();

    let points = [(-200, 200), (250, 100), (100, -250)];
    let points = points
        .iter()
        .map(|(x, y)| Vector2D::from_coord(*x as f32, *y as f32))
        .collect::<Vec<Vector2D>>();

    for i in 1..points.len() - 1 {
        img.draw_triangle2(points[0], points[i], points[i + 1], Image::BLUE);
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&img.to_u32_buffer(), width, height)
            .unwrap();
        std::thread::sleep(std::time::Duration::from_millis(30));
    }

    Ok(())
}
