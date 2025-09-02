use crate::renderer::{
    screen::{ScreenBuffer, HEIGHT, WIDTH},
    types::Color,
};

pub mod renderer;

fn main() {
    let screen: ScreenBuffer = [[Color::default(); WIDTH]; HEIGHT];
}
