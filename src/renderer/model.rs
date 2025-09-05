use minmath::linear_algebra::vector::Vector;

pub struct Model {
    vertices: Vec<Vector<3>>,
    edges: Vec<usize>,
}

impl Model {
    pub fn new(vertices: Vec<Vector<3>>, edges: Vec<usize>) -> Self {
        Model { vertices, edges }
    }

    pub fn from_file(path: &str) -> Self {
        todo!()
    }
}
