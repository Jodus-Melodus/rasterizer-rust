use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};

use crate::renderer::{model::Model, screen::ScreenBuffer};

pub mod renderer;

const WIDTH: usize = 208;
const HEIGHT: usize = 50;

fn main() {
    let mut screen: ScreenBuffer<WIDTH, HEIGHT> = ScreenBuffer::new();
    let model = Model::load_from_file("objects/complex.obj").unwrap();
    let mut focal_length = 50.0;
    let mut running = true;

    while running {
        if event::poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Esc => running = false,
                    KeyCode::Char(c) => match c {
                        'w' => focal_length += 1.0,
                        's' => focal_length -= 1.0,
                        _ => {}
                    },
                    _ => {}
                }
            }
        }

        screen.draw_model(&model, focal_length);
        println!("{}", screen.display());
        screen.clear();
    }
}
