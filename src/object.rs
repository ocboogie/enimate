use egui::{Color32, Mesh, Pos2};

pub struct Material {
    color: Color32,
}

pub struct Transform {
    position: Pos2,
    rotation: f32,
    scale: f32,
}

pub struct Object {
    mesh: Mesh,
    material: Material,
    transform: Transform,
}
