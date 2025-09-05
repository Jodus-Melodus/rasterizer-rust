use minmath::linear_algebra::vector::Vector;

use crate::renderer::{screen::ScreenBuffer, types::Color};

pub mod renderer;

fn main() {
    let mut screen: ScreenBuffer<1280, 720> = ScreenBuffer::new();
    let a = Vector::new([-10.0, -10.0]);
    let b = Vector::new([10.0, -10.0]);
    let c = Vector::new([0.0, 10.0]);
    screen.draw_triangle(a, b, c, Color::WHITE);

    println!("{}", screen.ascii());
}
