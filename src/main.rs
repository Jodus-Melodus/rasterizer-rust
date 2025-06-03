mod rasterizer;
use minifb::{Key, Window, WindowOptions};
use rasterizer::Image;
use std::{
    io::Result,
    time::{Duration, Instant},
};

fn main() -> Result<()> {
    let width = 1024;
    let height = 1024;
    let mut img = Image::new(width, height);
    let mut window = Window::new("Rasterizer", width, height, WindowOptions::default()).unwrap();

    let points = [
        (356, 256),
        (327, 327),
        (256, 356),
        (185, 327),
        (156, 256),
        (185, 185),
        (256, 156),
        (327, 185),
    ];

    let start = Instant::now();
    for i in 1..points.len() - 1 {
        img.draw_triangle1(points[0], points[i], points[i + 1], Image::RED);
    }
    let duration = start.elapsed().as_secs_f32();
    let fps = 1.0 / duration;
    println!("{:.2?}", fps);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&img.to_u32_buffer(), width, height)
            .unwrap();
        std::thread::sleep(std::time::Duration::from_millis(30));
    }

    Ok(())
}
