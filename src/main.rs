mod rasterizer;
use minifb::{Key, Window, WindowOptions};
use rasterizer::Image;
use std::{
    io::{stdin, stdout, Result, Write},
    time::Instant,
};
use vector_2d_3d::{Vector2D, Vector3D};

use crate::rasterizer::{COLORS, TRIANGLE};

fn read_line(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    stdout().flush().unwrap();
    stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn main() -> Result<()> {
    let width = read_line("Width: ").parse::<usize>().unwrap_or(512);
    let height = read_line("Height: ").parse::<usize>().unwrap_or(512);
    let mut focal_length = read_line("Focal Length: ").parse::<f32>().unwrap_or(200.0);
    let camera_movement_speed = read_line("Camera Movement Speed: ")
        .parse::<f32>()
        .unwrap_or(0.1);
    let scroll_sensitivity = read_line("Scroll Sensitivity: ")
        .parse::<f32>()
        .unwrap_or(5.0);

    let mut camera = Vector3D::from_coord(0.0, 0.0, 5.0);
    let start_time = Instant::now();
    let mut last_frame_time = Instant::now();
    let mut frame_count = 0;
    let mut should_close = false;
    let mut img = Image::new(width, height);
    let mut window = Window::new("Rasterizer", width, height, WindowOptions::default()).unwrap();
    let points = TRIANGLE;

    let points = points
        .iter()
        .map(|(x, y, z)| Vector3D::from_coord(*x, *y, *z))
        .collect::<Vec<Vector3D>>();

    while window.is_open() && !should_close {
        img.clear();

        for key in window.get_keys_pressed(minifb::KeyRepeat::Yes) {
            match key {
                Key::S => camera.z += camera_movement_speed,
                Key::W => camera.z -= camera_movement_speed,
                Key::A => camera.x -= camera_movement_speed,
                Key::D => camera.x += camera_movement_speed,
                Key::Q => camera.y -= camera_movement_speed,
                Key::E => camera.y += camera_movement_speed,
                Key::Escape => {
                    should_close = true;
                }
                _ => (),
            }
        }

        let (_scroll_x, scroll_y) = window.get_scroll_wheel().unwrap_or((0.0, 0.0));
        if scroll_y != 0.0 {
            focal_length = (focal_length + -scroll_y * scroll_sensitivity).clamp(1.0, 500.0);
        }

        // draw(focal_length, camera, start_time, &mut img, &points);

        let frame_time = last_frame_time.elapsed().as_secs_f32();
        last_frame_time = Instant::now();
        frame_count += 1;
        let text = format!("Fps: {:.2}", 1.0 / frame_time);
        img.draw_text(
            &text,
            Vector2D::from_coord(width as f32 / -2.0, height as f32 / 2.0 - 5.0),
            COLORS[0],
        );
        println!("fps: {:.2}", 1.0 / frame_time);

        window
            .update_with_buffer(&img.to_u32_buffer(), width, height)
            .unwrap();
    }

    println!(
        "Average fps: {:.2}",
        frame_count as f32 / start_time.elapsed().as_secs_f32()
    );

    img.save_bmp("test.bmp")?;

    read_line("Press Enter to close");
    Ok(())
}

fn draw(
    focal_length: f32,
    camera: Vector3D,
    start_time: Instant,
    img: &mut Image,
    points: &Vec<Vector3D>,
) {
    let angle = start_time.elapsed().as_secs_f32() * 0.5;
    let cos_a = angle.cos();
    let sin_a = angle.sin();

    let points: Vec<Vector3D> = points
        .iter()
        .map(|p| Vector3D::from_coord(p.x * cos_a - p.z * sin_a, p.y, p.x * sin_a + p.z * cos_a))
        .collect();

    let projected_points = points
        .iter()
        .map(|point| Image::project_3d_to_2d(&*img, *point, camera, focal_length))
        .collect::<Vec<Option<Vector2D>>>();

    for i in 1..projected_points.len() - 1 {
        if let (Some(p0), Some(p1), Some(p2)) = (
            projected_points[0],
            projected_points[i],
            projected_points[i + 1],
        ) {
            img.draw_triangle4(p0, p1, p2, COLORS[i % COLORS.len()]);
        }
    }
}
