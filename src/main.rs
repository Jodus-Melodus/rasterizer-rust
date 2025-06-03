mod rasterizer;
use minifb::{Key, Window, WindowOptions};
use rasterizer::Image;
use std::io::Result;

fn main() -> Result<()> {
    let width = 512;
    let height = 512;
    let mut img = Image::new(width, height);
    let mut window = Window::new("Rasterizer", width, height, WindowOptions::default()).unwrap();

    let points = [(100, 100), (400, 100), (100, 400), (400, 400)];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&img.to_u32_buffer(), width, height)
            .unwrap();
        std::thread::sleep(std::time::Duration::from_millis(30));
    }

    Ok(())
}
