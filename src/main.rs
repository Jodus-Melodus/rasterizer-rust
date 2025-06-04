mod rasterizer;
use minifb::{Key, Window, WindowOptions};
use rasterizer::Image;
use serde_json::from_reader;
use std::{
    fs::File,
    io::{stdin, stdout, BufReader, Result, Write},
    time::Instant,
};
use vector_2d_3d::{Vector2D, Vector3D};

use crate::render_objects::{Object, Text};
mod render_objects;

fn read_line(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    stdout().flush().unwrap();
    stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn read_render_file() -> Result<(Vec<Object>, Vec<Text>)> {
    let object_file = File::open("objects.json")?;
    let text_file = File::open("text.json")?;
    let object_reader = BufReader::new(object_file);
    let text_reader = BufReader::new(text_file);
    let objects: Vec<Object> = from_reader(object_reader)?;
    let text: Vec<Text> = from_reader(text_reader)?;
    Ok((objects, text))
}

fn objects_to_points(objects: Vec<Object>) -> Vec<(Vec<Vector3D>, (u8, u8, u8))> {
    let mut objs = Vec::new();

    for object in objects {
        let points = object
            .points
            .iter()
            .map(|pts| Vector3D::from_coord(pts[0], pts[1], pts[2]))
            .collect::<Vec<Vector3D>>();
        objs.push((points, Into::<(u8, u8, u8)>::into(object.color)));
    }

    objs
}

fn text_to_points(text: Vec<Text>) -> Vec<(String, Vector2D, (u8, u8, u8))> {
    let mut txt = Vec::new();

    for t in text {
        txt.push((
            t.text,
            Vector2D::from_coord(t.origin[0], t.origin[1]),
            Into::<(u8, u8, u8)>::into(t.color),
        ));
    }

    txt
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

    let (objects, text) = read_render_file().unwrap();
    println!("{:#?}", objects);

    let objects = objects_to_points(objects);
    let text = text_to_points(text);

    while window.is_open() && !should_close {
        img.clear();

        // key binds
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

        // draw objects
        for object in &objects {
            let (points, color) = object;

            let projected_points = points
                .iter()
                .map(|point| img.project_3d_to_2d(*point, camera, focal_length))
                .collect::<Vec<Option<Vector2D>>>();

            for i in 1..projected_points.len() - 1 {
                if let (Some(p0), Some(p1), Some(p2)) = (
                    projected_points[0],
                    projected_points[i],
                    projected_points[i + 1],
                ) {
                    img.draw_triangle4(p0, p1, p2, *color);
                }
            }
        }

        // draw text
        for txt in &text {
            let (t, origin, color) = txt;
            img.draw_text(t, *origin, *color);
        }

        let frame_time = last_frame_time.elapsed().as_secs_f32();
        last_frame_time = Instant::now();
        frame_count += 1;
        println!("Fps: {:.2}", 1.0 / frame_time);

        window
            .update_with_buffer(&img.to_u32_buffer(), width, height)
            .unwrap();
    }

    println!(
        "Average fps: {:.2}",
        frame_count as f32 / start_time.elapsed().as_secs_f32()
    );

    read_line("Press Enter to close");
    Ok(())
}
