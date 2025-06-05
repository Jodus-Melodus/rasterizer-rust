mod rasterizer;
use minifb::{Key, Window, WindowOptions};
use rasterizer::Image;
use serde_json::from_reader;
use std::{
    fs::{self, File},
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

fn read_render_file() -> Result<(Vec<Vec<Object>>, Vec<Text>)> {
    let directory_name = "objects";
    let mut objects = Vec::new();

    match fs::read_dir(directory_name) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        let object_file = File::open(path)?;
                        let object_reader = BufReader::new(object_file);
                        let object: Vec<Object> = from_reader(object_reader)?;
                        objects.push(object);
                    }
                }
            }
        }
        Err(e) => eprintln!("Failed to read directory: {}", e),
    }

    let text_file = File::open("text.json")?;
    let text_reader = BufReader::new(text_file);
    let text: Vec<Text> = from_reader(text_reader)?;
    Ok((objects, text))
}

fn objects_to_points(
    object_array: Vec<Vec<Object>>,
) -> Vec<(Vec<Vector3D>, u32, (bool, bool, bool))> {
    let mut objs = Vec::new();

    for objects in object_array {
        for object in objects {
            let points = object
                .points
                .iter()
                .map(|pts| Vector3D::from_coord(pts[0], pts[1], pts[2]))
                .collect::<Vec<Vector3D>>();
            if object.draw {
                objs.push((
                    points,
                    object.color,
                    Into::<(bool, bool, bool)>::into(object.rotation),
                ));
            }
        }
    }

    objs
}

fn text_to_points(text: Vec<Text>) -> Vec<(String, Vector2D, u32)> {
    let mut txt = Vec::new();

    for t in text {
        if t.draw {
            txt.push((
                t.text,
                Vector2D::from_coord(t.origin[0], t.origin[1]),
                t.color,
            ));
        }
    }

    txt
}

fn main() -> Result<()> {
    let width = read_line("Width: ").parse::<usize>().unwrap_or(1_280);
    let height = read_line("Height: ").parse::<usize>().unwrap_or(640);
    let mut focal_length = read_line("Focal Length: ").parse::<f32>().unwrap_or(200.0);
    let movement_speed = read_line("Camera Movement Speed: ")
        .parse::<f32>()
        .unwrap_or(0.1);
    let scroll_sensitivity = read_line("Scroll Sensitivity: ")
        .parse::<f32>()
        .unwrap_or(5.0);

    let camera = Vector3D::from_coord(0.0, 0.0, 5.0);
    let start_time = Instant::now();
    let mut last_frame_time = Instant::now();
    let mut frame_count = 0;
    let mut should_close = false;
    let mut img = Image::new(width, height);
    let mut window = Window::new("Rasterizer", width, height, WindowOptions::default()).unwrap();

    let (objects, text) = read_render_file().unwrap();
    let objects = objects_to_points(objects);
    let text = text_to_points(text);

    let mut x_movement = 0.0;
    let mut y_movement = 0.0;
    let mut z_movement = 0.0;

    while window.is_open() && !should_close {
        img.clear();

        // key binds
        for key in window.get_keys_pressed(minifb::KeyRepeat::Yes) {
            match key {
                Key::S => z_movement += movement_speed,
                Key::W => z_movement -= movement_speed,
                Key::A => x_movement -= movement_speed,
                Key::D => x_movement += movement_speed,
                Key::Q => y_movement -= movement_speed,
                Key::E => y_movement += movement_speed,
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

        let translate_vector = Vector3D::from_coord(x_movement, y_movement, z_movement);

        let mut objects_with_depth: Vec<_> = objects
            .iter()
            .map(|object| {
                let (points, color, rotation) = object;
                let rotated_points: Vec<_> = points
                    .iter()
                    .map(|point| Image::rotate(*point, start_time, *rotation))
                    .collect();
                let translated_points: Vec<_> = rotated_points
                    .iter()
                    .map(|point| Image::translate(*point, translate_vector))
                    .collect();
                // Compute average z in camera space
                let avg_z = translated_points.iter().map(|p| p.z).sum::<f32>()
                    / translated_points.len() as f32;
                (avg_z, translated_points, color)
            })
            .collect();

        // Sort by avg_z descending (furthest first)
        objects_with_depth
            .sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        for (_avg_z, rotated_points, color) in objects_with_depth {
            let projected_points: Vec<_> = rotated_points
                .iter()
                .map(|point| img.project_3d_to_2d(*point, camera, focal_length))
                .collect();

            for i in 1..projected_points.len() - 1 {
                if let (Some(p0), Some(p1), Some(p2)) = (
                    projected_points[0],
                    projected_points[i],
                    projected_points[i + 1],
                ) {
                    img.draw_triangle2(p0, p1, p2, *color);
                }
            }
        }

        // draw text
        for txt in &text {
            let (t, origin, color) = txt;
            img.draw_text(t, *origin, *color, 3);
        }

        let frame_time = last_frame_time.elapsed().as_secs_f32();
        last_frame_time = Instant::now();
        frame_count += 1;
        let fps = 1.0 / frame_time;
        img.draw_text(
            &format!("Fps: {:.2}", fps),
            Vector2D::from_coord(
                (width as isize / -2) as f32 + 5.0,
                (height as isize / 2) as f32 - 5.0,
            ),
            u32::MAX,
            2,
        );
        // println!("Fps: {:.2}", fps);

        window
            .update_with_buffer(&img.get_pixels(), width, height)
            .unwrap();
    }

    println!(
        "Average fps: {:.2}",
        frame_count as f32 / start_time.elapsed().as_secs_f32()
    );

    // read_line("Press Enter to close");
    Ok(())
}
