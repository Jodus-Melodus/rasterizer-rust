use crate::renderer::{model::Model, screen::ScreenBuffer};
use crossterm::event::{self, Event, KeyCode};
use std::{thread::sleep, time::Duration};

pub mod renderer;

const WIDTH: usize = 208;
const HEIGHT: usize = 50;

fn main() {
    let mut screen: ScreenBuffer<WIDTH, HEIGHT> = ScreenBuffer::new();
    let model = Model::load_from_file("objects/torus.obj").unwrap();
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

        screen.clear();
        screen.draw_model(&model, focal_length);
        print!("\x1b[2J\x1b[H");
        println!("{}", screen.display());
        sleep(Duration::from_millis(100));
    }
}
