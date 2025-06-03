mod rasterizer;
use minifb::{Key, Window, WindowOptions};
use rasterizer::Image;
use std::io::Result;

fn rasterize() -> Result<()> {
    let width = 512;
    let height = 512;
    let mut img = Image::new(width, height);
    let mut window = Window::new("Rasterizer", width, height, WindowOptions::default()).unwrap();

    img.draw_triangle2((100, 100), (100, 300), (300, 100), Image::BLUE);

    window
        .update_with_buffer(&img.to_u32_buffer(), width, height)
        .unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update();
        std::thread::sleep(std::time::Duration::from_millis(30));
    }
    Ok(())
}

fn main() -> Result<()> {
    rasterize()?;
    Ok(())
}
