use egui::{pos2, Color32, Pos2, Rect};
use lyon::path::Path;

#[derive(Clone, Debug)]
pub struct FillMaterial {
    pub color: Color32,
}

impl FillMaterial {
    pub fn new(color: Color32) -> Self {
        Self { color }
    }
}

#[derive(Clone, Debug)]
pub struct StrokeMaterial {
    pub color: Color32,
    pub width: f32,
}

impl StrokeMaterial {
    pub fn new(color: Color32, width: f32) -> Self {
        Self { color, width }
    }
}

#[derive(Clone, Default, Debug)]
pub struct Material {
    pub fill: Option<FillMaterial>,
    pub stroke: Option<StrokeMaterial>,
}

impl From<FillMaterial> for Material {
    fn from(fill: FillMaterial) -> Self {
        Self {
            fill: Some(fill),
            ..Default::default()
        }
    }
}

impl From<StrokeMaterial> for Material {
    fn from(stroke: StrokeMaterial) -> Self {
        Self {
            stroke: Some(stroke),
            ..Default::default()
        }
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

    pub fn map_aabb(&self, rect: Rect) -> Rect {
        let top_left = self.apply(rect.min);
        let top_right = self.apply(pos2(rect.max.x, rect.min.y));
        let bottom_left = self.apply(pos2(rect.min.x, rect.max.y));
        let bottom_right = self.apply(rect.max);

        // TODO: There is probably a better way to do this.
        let min_x = top_left
            .x
            .min(top_right.x)
            .min(bottom_left.x)
            .min(bottom_right.x);
        let min_y = top_left
            .y
            .min(top_right.y)
            .min(bottom_left.y)
            .min(bottom_right.y);

        let max_x = top_left
            .x
            .max(top_right.x)
            .max(bottom_left.x)
            .max(bottom_right.x);
        let max_y = top_left
            .y
            .max(top_right.y)
            .max(bottom_left.y)
            .max(bottom_right.y);

        Rect::from_min_max(pos2(min_x, min_y), pos2(max_x, max_y))
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
    pub path: Path,
    pub material: Material,
}

pub type ObjectId = usize;

#[derive(Clone, Debug)]
pub enum ObjectKind {
    Model(Model),
    Group(Vec<ObjectId>),
}

#[derive(Clone, Debug)]
pub struct Object {
    pub object_kind: ObjectKind,
    pub transform: Transform,
}

impl Object {
    pub fn new_model(path: Path, material: Material) -> Self {
        Self {
            object_kind: ObjectKind::Model(Model { path, material }),
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
