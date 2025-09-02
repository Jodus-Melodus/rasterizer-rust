use crate::renderer::screen::ScreenBuffer;

pub mod renderer;

fn main() {
    let screen = ScreenBuffer::<1280, 720>::new();
}
