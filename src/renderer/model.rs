use image::{GenericImageView, ImageReader};
use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind, Result},
};

use crate::renderer::types::{
    vertices::{Vertex2, Vertex3},
    Color,
};

pub type TextureCoordinate = Vertex2;

#[derive(Clone)]
pub struct Model {
    pub mesh: Mesh,
    pub texture_map: Option<TextureMap>,
}

impl Model {
    pub fn new(mesh: Mesh, texture_path: Option<&str>) -> Self {
        if let Some(texture_path) = texture_path {
            let texture_map = TextureMap::load_from_file(texture_path).ok();
            Model { mesh, texture_map }
        } else {
            Model {
                mesh,
                texture_map: None,
            }
        }
    }

    pub fn load_from_file(object_path: &str, texture_path: Option<&str>) -> Result<Self> {
        let mesh = Mesh::load_from_file(object_path)?;
        let texture_map = if let Some(texture_path) = texture_path {
            TextureMap::load_from_file(texture_path).ok()
        } else {
            None
        };
        Ok(Model { mesh, texture_map })
    }
}

#[derive(Clone)]
pub struct TextureMap {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>,
}

impl TextureMap {
    pub fn new(width: usize, height: usize, data: Vec<u8>) -> Self {
        TextureMap {
            width,
            height,
            data,
        }
    }

    pub fn load_from_file(path: &str) -> Result<Self> {
        // Read image data
        let img = ImageReader::open(path)?
            .decode()
            .map_err(|e| Error::new(ErrorKind::Other, e))?;
        let (width, height) = img.dimensions();
        let data = img.to_rgba8().into_raw();
        Ok(TextureMap {
            width: width as usize,
            height: height as usize,
            data,
        })
    }

    pub fn get_pixel(&self, texture_coordinate: TextureCoordinate) -> Option<Color> {
        // Clamp UVs to [0, 1]
        let u = texture_coordinate.x.clamp(0.0, 1.0);
        let v = texture_coordinate.y.clamp(0.0, 1.0);
        // Convert to pixel coordinates
        let x = (u * (self.width as f32 - 1.0)).round() as usize;
        // Flip v for image coordinate system
        let y = ((1.0 - v) * (self.height as f32 - 1.0)).round() as usize;
        let index = (y * self.width + x) * 4;
        if index + 3 < self.data.len() {
            Some(Color::new(
                self.data[index],
                self.data[index + 1],
                self.data[index + 2],
                self.data[index + 3],
            ))
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex3>,
    pub texture_coordinates: Vec<TextureCoordinate>,
    pub vertex_normals: Vec<Vertex3>,
    pub vertex_indices: Vec<(usize, usize, usize)>,
    pub texture_coordinate_indices: Vec<(usize, usize, usize)>,
    pub vertex_normal_indices: Vec<(usize, usize, usize)>,
}

impl Mesh {
    pub fn new(
        vertices: Vec<Vertex3>,
        vertex_indices: Vec<(usize, usize, usize)>,
        texture_coordinate_indices: Vec<(usize, usize, usize)>,
        vertex_normal_indices: Vec<(usize, usize, usize)>,
        texture_coordinates: Vec<TextureCoordinate>,
        vertex_normals: Vec<Vertex3>,
    ) -> Self {
        Mesh {
            vertices,
            vertex_indices,
            texture_coordinate_indices,
            vertex_normal_indices,
            texture_coordinates,
            vertex_normals,
        }
    }

    pub fn load_from_file(path: &str) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut vertices = Vec::new();
        let mut vertex_indices = Vec::new();
        let mut texture_coordinate_indices = Vec::new();
        let mut vertex_normal_indices = Vec::new();
        let mut texture_coordinates = Vec::new();
        let mut vertex_normals = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let mut parts = line.split_ascii_whitespace();
            if let Some(line_type) = parts.next() {
                match line_type {
                    "v" => {
                        let coords: Vec<f32> = parts.map(|p| p.parse().unwrap()).collect();
                        if coords.len() == 3 {
                            vertices.push(Vertex3::new(coords[0], coords[1], coords[2]));
                        } else {
                            eprintln!("Unknown vertex syntax: {}", line);
                        }
                    }
                    "f" => {
                        let face_indices: Vec<Vec<Option<usize>>> = parts
                            .map(|f| {
                                let mut indices = f.split('/');
                                let v = indices
                                    .next()
                                    .and_then(|s| s.parse().ok())
                                    .map(|i: usize| i - 1);
                                let vt = indices.next().and_then(|s| {
                                    if !s.is_empty() {
                                        s.parse().ok().map(|i: usize| i - 1)
                                    } else {
                                        None
                                    }
                                });
                                let vn = indices.next().and_then(|s| {
                                    if !s.is_empty() {
                                        s.parse().ok().map(|i: usize| i - 1)
                                    } else {
                                        None
                                    }
                                });
                                vec![v, vt, vn]
                            })
                            .collect();

                        for i in 1..face_indices.len() - 1 {
                            if let (Some(v0), Some(v1), Some(v2)) = (
                                face_indices[0][0],
                                face_indices[i][0],
                                face_indices[i + 1][0],
                            ) {
                                vertex_indices.push((v0, v1, v2));
                            }
                            if let (Some(t0), Some(t1), Some(t2)) = (
                                face_indices[0][1],
                                face_indices[i][1],
                                face_indices[i + 1][1],
                            ) {
                                texture_coordinate_indices.push((t0, t1, t2));
                            }
                            if let (Some(n0), Some(n1), Some(n2)) = (
                                face_indices[0][2],
                                face_indices[i][2],
                                face_indices[i + 1][2],
                            ) {
                                vertex_normal_indices.push((n0, n1, n2));
                            }
                        }
                    }
                    "vt" => {
                        let coords: Vec<f32> = parts.map(|p| p.parse().unwrap()).collect();
                        if coords.len() == 2 {
                            texture_coordinates.push(TextureCoordinate::new(coords[0], coords[1]));
                        } else {
                            eprintln!("Unknown texture coordinate syntax: {}", line);
                        }
                    }
                    "vn" => {
                        let normal: Vec<f32> = parts.map(|p| p.parse().unwrap()).collect();
                        if normal.len() == 3 {
                            vertex_normals.push(Vertex3::new(normal[0], normal[1], normal[2]));
                        } else {
                            eprintln!("Unknown vertex syntax: {}", line);
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(Mesh::new(
            vertices,
            vertex_indices,
            texture_coordinate_indices,
            vertex_normal_indices,
            texture_coordinates,
            vertex_normals,
        ))
    }
}
