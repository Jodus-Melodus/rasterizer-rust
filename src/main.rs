use crossterm::event::{read, Event, KeyCode, KeyEvent};

use crate::renderer::{screen::ScreenBuffer, types::Color, vector::Vector2};

pub mod renderer;

const WIDTH: usize = 208;
const HEIGHT: usize = 50;

fn main() {
    let mut screen: ScreenBuffer<WIDTH, HEIGHT> = ScreenBuffer::new();
    let mut running = true;

    while running {
        match read().unwrap() {
            Event::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Esc => {
                    running = false;
                }
                _ => {}
            },
            _ => {}
        }

        let a = Vector2::new(-10.0, -10.0);
        let b = Vector2::new(10.0, -10.0);
        let c = Vector2::new(0.0, 10.0);
        screen.draw_triangle(a, b, c, Color::WHITE);
        println!("{}", screen.ascii());
        screen.clear();
    }
}
