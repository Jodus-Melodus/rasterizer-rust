use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::{
    io::{stdin, stdout, Write},
    time::Instant,
};

use crate::render::{
    rasterizer::Screen,
    types::{
        x_rotation_matrix, y_rotation_matrix, z_rotation_matrix, Camera, Color, FrameBufferSize,
        Vector3,
    },
};

mod render;

fn read_line(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    stdout().flush().unwrap();
    stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    // let width = read_line("Width: ").parse::<usize>().unwrap_or(1024);
    // let height = read_line("Height: ").parse::<usize>().unwrap_or(512);
    let (width, height) = (1024, 512);
    let frame_buffer_size = FrameBufferSize::new(width, height);
    let mut screen = Screen::new(frame_buffer_size);
    let mut window = Window::new("Rasterizer", width, height, WindowOptions::default()).unwrap();
    let mut frame_count = 0;
    let camera = Camera::new(Vector3::new(0.0, 0.0, -5.0), 90.0_f32.to_radians());

    let shape = (
        [
            (1.0, 1.0, 1.0),
            (1.0, 1.0, -1.0),
            (1.0, -1.0, 1.0),
            (1.0, -1.0, -1.0),
            (-1.0, 1.0, 1.0),
            (-1.0, 1.0, -1.0),
            (-1.0, -1.0, 1.0),
            (-1.0, -1.0, -1.0),
        ],
        [
            (0, 2, 4),
            (2, 6, 4),
            (1, 5, 3),
            (3, 5, 7),
            (0, 4, 1),
            (1, 4, 5),
            (2, 3, 6),
            (3, 7, 6),
            (0, 1, 2),
            (1, 3, 2),
            (4, 6, 5),
            (5, 6, 7),
        ],
    );
    let shape = (shape.0.map(|(x, y, z)| Vector3::new(x, y, z)), shape.1);
    let start_time = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let pressed_keys = window.get_keys_pressed(KeyRepeat::Yes);
        if !pressed_keys.is_empty() {
            for key in pressed_keys {
                match key {
                    Key::W => (),
                    Key::S => (),
                    _ => (),
                }
            }
        }

        let theta = start_time.elapsed().as_secs_f32() * 1.0;
        let x_rotation_matrix = x_rotation_matrix(theta);
        let y_rotation_matrix = y_rotation_matrix(theta);
        let z_rotation_matrix = z_rotation_matrix(theta);

        screen.clear();
        let rotated_points = shape.0.map(|v| v * y_rotation_matrix);

        let triangles = shape
            .1
            .iter()
            .map(|(i1, i2, i3)| {
                [
                    rotated_points[*i1],
                    rotated_points[*i2],
                    rotated_points[*i3],
                ]
            })
            .collect::<Vec<_>>();

        let mut projected_triangles = triangles
            .iter()
            .map(|triangle: &[Vector3; 3]| screen.project_triangle(*triangle, camera))
            .collect::<Vec<_>>();

        projected_triangles.sort_by(|a, b| {
            let avg_a = (a[0].1 + a[1].1 + a[2].1) / 3.0;
            let avg_b = (b[0].1 + b[1].1 + b[2].1) / 3.0;
            avg_a
                .partial_cmp(&avg_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        for triangle in projected_triangles.iter().rev() {
            screen.draw_triangle(*triangle, Color::random());
        }

        window
            .update_with_buffer(&screen.frame_buffer(), width, height)
            .unwrap();

        frame_count += 1;
    }

    let duration = start_time.elapsed();
    let fps = frame_count as f32 / duration.as_secs_f32();
    println!("Fps: {}", fps);
}

// fix right side coords
