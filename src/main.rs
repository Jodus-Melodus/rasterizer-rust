use minifb::{Key, Window, WindowOptions};

use crate::renderer::{screen::ScreenBuffer, types::Color, vector::Vector2};

pub mod renderer;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

fn main() {
    let mut window =
        Window::new("rasterizer-rust", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    let mut screen: ScreenBuffer<WIDTH, HEIGHT> = ScreenBuffer::new();
    let a = Vector2::new(-10.0, -10.0);
    let b = Vector2::new(10.0, -10.0);
    let c = Vector2::new(0.0, 10.0);
    screen.draw_triangle(a, b, c, Color::WHITE);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&screen.pixels(), WIDTH, HEIGHT)
            .unwrap();
    }
}
