use minifb::{Key, KeyRepeat, Window, WindowOptions};
use rand::random_range;
use std::{
    io::{stdin, stdout, Write},
    time::Instant,
};

use crate::render::{
    rasterizer::Screen,
    types::{Color, FrameBufferSize, Vector2, Vector3},
};

mod render;

fn read_line(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    stdout().flush().unwrap();
    stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    let width = read_line("Width: ").parse::<usize>().unwrap_or(1024);
    let height = read_line("Height: ").parse::<usize>().unwrap_or(512);
    let frame_buffer_size = FrameBufferSize::new(width, height);
    let mut screen = Screen::new(frame_buffer_size);
    let mut window = Window::new("Rasterizer", width, height, WindowOptions::default()).unwrap();
    let start_time = Instant::now();
    let mut frame_count = 0;
    let camera = Vector3::new(0, 0, -10);
    let fov = 90.0_f32.to_radians();
    let triangle = [
        Vector3::new(2, 0, 3),
        Vector3::new(2, -2, 2),
        Vector3::new(-2, 2, 1),
    ];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let pressed_keys = window.get_keys_pressed(KeyRepeat::Yes);
        if !pressed_keys.is_empty() {
            for key in pressed_keys {
                match key {
                    Key::W => (),
                    Key::S => (),
                    _ => (),
                }
            }
        }

        screen.clear();
        let projected_points: [Vector2; 3] =
            triangle.map(|point| screen.project(point, camera, fov));
        screen.draw_triangle(projected_points, Color::BLUE);
        screen.draw_point(Vector2::new(0, 0), Color::RED);

        window
            .update_with_buffer(&screen.frame_buffer(), width, height)
            .unwrap();

        frame_count += 1;
    }

    let duration = start_time.elapsed();
    let fps = frame_count as f32 / duration.as_secs_f32();
    println!("Fps: {}", fps);
}
