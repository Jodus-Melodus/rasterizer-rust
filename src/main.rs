use crate::renderer::{screen::project_coordinate, vector::Vector3};

pub mod renderer;

fn main() {
    // let mut screen: ScreenBuffer<1280, 720> = ScreenBuffer::new();
    // let a = Vector2::new(-10.0, -10.0);
    // let b = Vector2::new(10.0, -10.0);
    // let c = Vector2::new(0.0, 10.0);
    // screen.draw_triangle(a, b, c, Color::WHITE);
    // println!("{}", screen.ascii());

    let point = Vector3::new(0.0, 0.0, -5.0);
    println!("{:?}", point);
    let projected_point = project_coordinate(point, 10.0);
    println!("{:?}", projected_point);
}
