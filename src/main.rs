use crate::renderer::{
    model::Model,
    screen::{rotate_model, ScreenBuffer},
    types::Axis,
};
use crossterm::event::{self, Event, KeyCode};
use std::{
    thread::sleep,
    time::{Duration, Instant},
};

pub mod renderer;

const WIDTH: usize = 208;
const HEIGHT: usize = 50;

fn main() {
    let mut screen: ScreenBuffer<WIDTH, HEIGHT> = ScreenBuffer::new();
    let mut model = Model::load_from_file("objects/torus.obj").unwrap();
    let mut focal_length = 50.0;
    let mut running = true;
    let mut last_frame = Instant::now();
    let rotation_speed = 1.0;

    while running {
        let now = Instant::now();
        let delta_time = now.duration_since(last_frame).as_secs_f32();
        last_frame = now;

        if event::poll(Duration::from_millis(1)).unwrap() {
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

        rotate_model(&mut model, Axis::X, rotation_speed * delta_time);

        screen.clear();
        screen.draw_model(&model, focal_length);
        print!("\x1b[2J\x1b[H");
        println!("{}", screen.display());

        sleep(Duration::from_millis(16));
    }
}
