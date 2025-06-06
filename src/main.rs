use minifb::{Key, Window, WindowOptions};
use rand::random_range;
use std::{
    io::{stdin, stdout, Write},
    time::Instant,
};

use crate::render::{
    rasterizer::Screen,
    types::{Color, FrameBufferSize, Vector2},
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

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let scale = random_range(1..10);
        let triangle_points = [
            Vector2::new(
                random_range(-(100 * scale)..=(100 * scale)) as isize,
                random_range(-(100 * scale)..=(100 * scale)) as isize,
            ),
            Vector2::new(
                random_range(-(100 * scale)..=(100 * scale)) as isize,
                random_range(-(100 * scale)..=(100 * scale)) as isize,
            ),
            Vector2::new(
                random_range(-(100 * scale)..=(100 * scale)) as isize,
                random_range(-(100 * scale)..=(100 * scale)) as isize,
            ),
        ];
        screen.clear();
        screen.draw_triangle(
            triangle_points,
            Color::new(
                random_range(0..=255),
                random_range(0..=255),
                random_range(0..=255),
            ),
        );
        window
            .update_with_buffer(&screen.frame_buffer(), width, height)
            .unwrap();

        frame_count += 1;
    }

    let duration = start_time.elapsed();
    let fps = frame_count as f32 / duration.as_secs_f32();
    println!("Fps: {}", fps);
}

// 75 fps
// 220 fps
