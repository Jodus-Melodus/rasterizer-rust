use std::{
    fs::File,
    io::{Error, Read},
};

use image::{GenericImageView, Pixel};

use crate::renderer::types::{Color, Point2D, Point3D};

pub struct Model {
    pub vertices: Vec<Point3D>,
    pub faces: Vec<((usize, usize, usize), (usize, usize, usize))>,
    pub texture: Vec<Vec<Color>>,
    pub texture_coordinates: Vec<Point2D>,
}

impl Model {
    pub fn load_from_file(model_path: &str, texture_path: &str) -> Result<Self, Error> {
        // Loading objects
        let mut file = File::open(model_path)?;
        let mut vertices = Vec::new();
        let mut faces = Vec::new();
        let mut texture_coordinates = Vec::new();
        let mut buffer = String::new();

        file.read_to_string(&mut buffer)?;

        for line in buffer.split("\n") {
            if line.is_empty() || line.starts_with("#") {
                continue;
            }

            let tokens = line.split_ascii_whitespace().collect::<Vec<&str>>();
            if tokens.len() < 1 {
                panic!("Invalid object file");
            }

            match tokens[0] {
                "v" => {
                    vertices.push(Point3D::new(
                        tokens[1].parse::<f32>().unwrap_or_else(|e| panic!("{}", e)),
                        tokens[2].parse::<f32>().unwrap_or_else(|e| panic!("{}", e)),
                        tokens[3].parse::<f32>().unwrap_or_else(|e| panic!("{}", e)),
                    ));
                }
                "vt" => {
                    texture_coordinates.push(Point2D::new(
                        tokens[1].parse::<f32>().unwrap_or_else(|e| panic!("{}", e)),
                        tokens[2].parse::<f32>().unwrap_or_else(|e| panic!("{}", e)),
                    ));
                }
                "f" => {
                    let mut face_indices = Vec::new();
                    let mut texture_indices = Vec::new();
                    for i in 1..tokens.len() {
                        let indices = tokens[i].split("/").collect::<Vec<&str>>();
                        if indices.len() != 3 {
                            panic!("Invalid object file");
                        }

                        let (vertex_index, texture_index, _normal_index) = (
                            indices[0]
                                .parse::<usize>()
                                .unwrap_or_else(|e| panic!("{}", e))
                                - 1,
                            indices[1]
                                .parse::<usize>()
                                .unwrap_or_else(|e| panic!("{}", e))
                                - 1,
                            indices[2]
                                .parse::<usize>()
                                .unwrap_or_else(|e| panic!("{}", e))
                                - 1,
                        );

                        face_indices.push(vertex_index);
                        texture_indices.push(texture_index);
                    }

                    for i in 1..face_indices.len() - 1 {
                        faces.push((
                            (face_indices[0], face_indices[i], face_indices[i + 1]),
                            (
                                texture_indices[0],
                                texture_indices[i],
                                texture_indices[i + 1],
                            ),
                        ));
                    }
                }
                _ => {}
            }
        }

        // Loading texture
        let texture_file = image::open(texture_path)
            .expect(&format!("Failed to open texture file: {}", texture_path));
        let (width, height) = texture_file.dimensions();
        let mut texture = vec![vec![Color::BLACK; width as usize]; height as usize];

        for y in 0..height {
            for x in 0..width {
                let pixel = texture_file.get_pixel(x, y).to_rgba().0;
                texture[y as usize][x as usize] =
                    Color::new(pixel[0], pixel[1], pixel[2], pixel[3]);
            }
        }

        Ok(Model {
            vertices,
            faces,
            texture,
            texture_coordinates,
        })
    }
}
