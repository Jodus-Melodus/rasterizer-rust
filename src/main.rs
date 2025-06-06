use minifb::{Key, Window, WindowOptions};
use std::io::{stdin, stdout, Write};
use vector_2d_3d::Vector2D;

use crate::render::{
    rasterizer::Screen,
    types::{Color, FrameBufferSize},
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

    while window.is_open() && !window.is_key_down(Key::Escape) {
        screen.draw_point(Vector2D::from_coord(0.0, 0.0), Color::new(255, 0, 0, 255));

        window
            .update_with_buffer(&screen.frame_buffer(), width, height)
            .unwrap();
    }
}
