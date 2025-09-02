use crate::renderer::types::Color;

pub const WIDTH: usize = 1280;
pub const HEIGHT: usize = 720;
pub type ScreenBuffer = [[Color; WIDTH]; HEIGHT];
