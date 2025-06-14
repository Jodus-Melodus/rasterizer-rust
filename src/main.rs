use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::{io::Result, time::Instant};

use crate::renderer::{
    model::Model,
    rasterizer::Screen,
    types::{vertices::Vertex3, Camera, FrameBufferSize},
};

mod renderer;

fn main() -> Result<()> {
    let (width, height) = (1024, 512);
    let frame_buffer_size = FrameBufferSize::new(width, height);
    let mut screen = Screen::new(frame_buffer_size);
    let mut window = Window::new("Rasterizer", width, height, WindowOptions::default()).unwrap();
    let mut frame_count = 0;
    let mut camera = Camera::new(Vertex3::new(0.0, 0.0, -10.0), 90.0_f32.to_radians());
    let mut shape = Model::load_from_file("objects/cube.obj", Some("objects/texture.png"))?;
    let start_time = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let pressed_keys = window.get_keys_pressed(KeyRepeat::Yes);
        if !pressed_keys.is_empty() {
            for key in pressed_keys {
                match key {
                    Key::W => camera.z += 1.0,
                    Key::S => camera.z -= 1.0,
                    _ => (),
                }
            }
        }

        screen.clear();
        let theta = 0.01;
        screen.rotate_model(&mut shape, (0, 1, 0), theta);
        screen.draw_model(&mut shape, camera);

        window
            .update_with_buffer(screen.frame_buffer(), width, height)
            .unwrap();

        frame_count += 1;
    }

    let duration = start_time.elapsed();
    let fps = frame_count as f32 / duration.as_secs_f32();
    println!("Fps: {}", fps);

    Ok(())
}
