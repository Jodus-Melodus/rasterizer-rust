use minifb::{Key, KeyRepeat, Window, WindowOptions};

use crate::renderer::{
    screen::{project_coordinate, ScreenBuffer},
    types::Color,
    vector::{Vector2, Vector3},
};

pub mod renderer;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

fn main() {
    let mut window =
        Window::new("rasterizer-rust", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    let mut screen: ScreenBuffer<WIDTH, HEIGHT> = ScreenBuffer::new();
    let mut z = 0.0;
    let mut focal_length = -10.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for k in window.get_keys() {
            match k {
                Key::W => z += 1.0,
                Key::S => z -= 1.0,
                _ => (),
            }
        }

        println!("{}", z);

        let a = project_coordinate(Vector3::new(-10.0, -10.0, z), focal_length);
        let b = project_coordinate(Vector3::new(10.0, -10.0, z), focal_length);
        let c = project_coordinate(Vector3::new(0.0, 10.0, z), focal_length);
        screen.draw_triangle(a, b, c, Color::WHITE);

        window
            .update_with_buffer(&screen.pixels(), WIDTH, HEIGHT)
            .unwrap();

        screen.clear();
    }
}
