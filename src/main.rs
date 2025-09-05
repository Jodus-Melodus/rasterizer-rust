use crate::renderer::model::Model;

pub mod renderer;

fn main() {
    let model = Model::from_file("object/cube.obj").unwrap();

    println!("{:?}\n{:?}", model.vertices, model.faces);
}
