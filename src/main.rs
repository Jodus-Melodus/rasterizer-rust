use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::{
    io::{stdin, stdout, Result, Write},
    time::Instant,
};

use crate::render::{
    rasterizer::Screen,
    types::{
        x_rotation_matrix, y_rotation_matrix, z_rotation_matrix, Camera, Color, FrameBufferSize,
        Mesh, Vector3,
    },
};

mod render;

fn read_line(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    stdout().flush().unwrap();
    stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn main() -> Result<()> {
    // let width = read_line("Width: ").parse::<usize>().unwrap_or(1024);
    // let height = read_line("Height: ").parse::<usize>().unwrap_or(512);
    let (width, height) = (1024, 512);
    let frame_buffer_size = FrameBufferSize::new(width, height);
    let mut screen = Screen::new(frame_buffer_size);
    let mut window = Window::new("Rasterizer", width, height, WindowOptions::default()).unwrap();
    let mut frame_count = 0;
    let mut camera = Camera::new(Vector3::new(0.0, 0.0, -2.0), 90.0_f32.to_radians());
    let shape = Mesh::load_from_file("sphere.obj")?;
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

        let theta = start_time.elapsed().as_secs_f32() * 1.0;
        screen.clear();
        screen.draw_shape(shape.clone(), theta, camera);

        window
            .update_with_buffer(&screen.frame_buffer(), width, height)
            .unwrap();

        frame_count += 1;
    }

    let duration = start_time.elapsed();
    let fps = frame_count as f32 / duration.as_secs_f32();
    println!("Fps: {}", fps);

    Ok(())
}

// fix right side coords
