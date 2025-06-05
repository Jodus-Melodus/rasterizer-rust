use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Object {
    pub name: String,
    pub points: Vec<[f32; 3]>,
    pub color: u32,
    pub rotation: [bool; 3],
    pub draw: bool,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Text {
    pub text: String,
    pub origin: [f32; 2],
    pub color: u32,
    pub draw: bool,
}
