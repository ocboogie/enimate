use crate::mesh::Mesh;
use egui::{Color32, Pos2};

#[derive(Clone, Default, Debug)]
pub struct Material {
    pub color: Color32,
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Transform {
    pub position: Pos2,
    pub rotation: f32,
    pub scale: f32,
}

impl Transform {
    pub fn with_position(&self, position: Pos2) -> Self {
        let mut new = self.clone();
        new.position = position;
        new
    }

    pub fn with_rotation(&self, rotation: f32) -> Self {
        let mut new = self.clone();
        new.rotation = rotation;
        new
    }

    pub fn with_scale(&self, scale: f32) -> Self {
        let mut new = self.clone();
        new.scale = scale;
        new
    }
}

#[derive(Default, Clone)]
pub struct Object {
    pub mesh: Mesh,
    pub material: Material,
    pub transform: Transform,
}
