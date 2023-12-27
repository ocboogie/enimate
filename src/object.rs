use crate::mesh::Mesh;
use egui::{pos2, Color32, Pos2};

#[derive(Clone, Default, Debug)]
pub struct Material {
    pub color: Color32,
}

impl From<Color32> for Material {
    fn from(color: Color32) -> Self {
        Self { color }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Transform {
    pub position: Pos2,
    pub rotation: f32,
    pub scale: f32,
    pub anchor: Pos2,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Pos2::ZERO,
            rotation: 0.0,
            scale: 1.0,
            anchor: Pos2::ZERO,
        }
    }
}

impl Transform {
    fn rotate_vec_around_0(vector: Pos2, angle: f32) -> Pos2 {
        let (sin, cos) = angle.sin_cos();
        pos2(
            vector.x * cos - vector.y * sin,
            vector.x * sin + vector.y * cos,
        )
    }

    pub fn apply(&self, position: Pos2) -> Pos2 {
        let position = position - self.anchor.to_vec2();
        let position = Self::rotate_vec_around_0(position, self.rotation);
        let position = position * self.scale;
        position + self.position.to_vec2()
    }

    pub fn and_then(self, other: &Transform) -> Self {
        Transform {
            position: self.apply(other.position),
            scale: self.scale * other.scale,
            rotation: self.rotation + other.rotation,
            anchor: other.anchor,
        }
    }

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

    pub fn with_anchor(&self, anchor: Pos2) -> Self {
        let mut new = self.clone();
        new.anchor = anchor;
        new
    }
}

// TODO: At some point, we don't want Model to actually store the mesh, but rather a reference to
// it.
#[derive(Clone, Debug)]
pub struct Model {
    pub mesh: Mesh,
    pub material: Material,
}

pub type ObjectId = usize;

#[derive(Clone, Debug)]
pub enum ObjectKind {
    Model(Model),
    Group(Vec<ObjectId>),
    // Scene,
}

#[derive(Clone, Debug)]
pub struct Object {
    pub object_kind: ObjectKind,
    pub transform: Transform,
}

impl Object {
    pub fn new_model(mesh: Mesh, material: Material) -> Self {
        Self {
            object_kind: ObjectKind::Model(Model { mesh, material }),
            transform: Transform::default(),
        }
    }

    pub fn new_group(group: Vec<ObjectId>) -> Self {
        Self {
            object_kind: ObjectKind::Group(group),
            transform: Transform::default(),
        }
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }
}
