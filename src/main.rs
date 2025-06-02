mod rasterizer;
use rasterizer::Image;
use std::io::Result;
use std::time;

fn rasterize() -> Result<()> {
    let width = 512;
    let height = 512;
    let start = time::Instant::now();
    let mut img = Image::new(width, height);

    img.draw_triangle2((100, 100), (100, 300), (300, 100), Image::BLUE);

    let duration = start.elapsed();
    println!("Render time : {:.2?}", duration);

    img.save_bmp("test.bmp")?;
    Ok(())
}

fn main() -> Result<()> {
    rasterize()?;
    Ok(())
}
