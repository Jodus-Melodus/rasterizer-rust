use std::{
    fs::File,
    io::{Error, Read},
};

use minmath::linear_algebra::vector::Vector;

pub struct Model {
    pub vertices: Vec<Vector<3>>,
    pub faces: Vec<(usize, usize, usize)>,
}

impl Model {
    pub fn new(vertices: Vec<Vector<3>>, faces: Vec<(usize, usize, usize)>) -> Self {
        Model { vertices, faces }
    }

    pub fn from_file(path: &str) -> Result<Self, Error> {
        let mut file = File::open(path)?;
        let mut vertices = Vec::new();
        let mut faces = Vec::new();
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
                    vertices.push(Vector::<3>::new([
                        tokens[1].parse::<f32>().unwrap_or_else(|e| panic!("{}", e)),
                        tokens[2].parse::<f32>().unwrap_or_else(|e| panic!("{}", e)),
                        tokens[3].parse::<f32>().unwrap_or_else(|e| panic!("{}", e)),
                    ]));
                }
                "f" => {
                    let mut face_indices = Vec::new();
                    for i in 1..tokens.len() {
                        let indices = tokens[i].split("/").collect::<Vec<&str>>();
                        if indices.len() != 3 {
                            panic!("Invalid object file");
                        }

                        let (vertex_index, _texture_index, _normal_index) = (
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
                    }

                    for i in 1..face_indices.len() - 1 {
                        faces.push((face_indices[0], face_indices[i], face_indices[i + 1]));
                    }
                }
                _ => {}
            }
        }

        Ok(Model { vertices, faces })
    }
}
