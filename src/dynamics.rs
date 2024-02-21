use egui::Pos2;

use crate::{object::Transform, world::World};

pub trait WorldValue {
    fn get(&self, world: &World) -> f32;
}

pub enum DynamicValue {
    Literal(f32),
    Dynamic(Box<dyn WorldValue>),
}

impl DynamicValue {
    pub fn get(&self, world: &World) -> f32 {
        match self {
            DynamicValue::Literal(value) => *value,
            DynamicValue::Dynamic(value) => value.get(world),
        }
    }
}

pub trait WorldPos {
    fn get(&self, world: &World) -> Pos2;
}

impl WorldPos for (DynamicValue, DynamicValue) {
    fn get(&self, world: &World) -> Pos2 {
        Pos2::new(self.0.get(world), self.1.get(world))
    }
}

pub enum DynamicPos {
    Literal(Pos2),
    Dynamic(Box<dyn WorldPos>),
}

impl DynamicPos {
    pub fn get(&self, world: &World) -> Pos2 {
        match self {
            DynamicPos::Literal(pos) => *pos,
            DynamicPos::Dynamic(pos) => pos.get(world),
        }
    }
}

impl From<Pos2> for DynamicPos {
    fn from(pos: Pos2) -> Self {
        DynamicPos::Literal(pos)
    }
}

pub trait WorldTransform {
    fn get(&self, world: &World) -> Transform;
}

pub struct DynamicTransform {
    pub position: DynamicPos,
    pub scale: DynamicValue,
    pub rotation: DynamicValue,
    pub anchor: DynamicPos,
}

impl DynamicTransform {
    pub fn get(&self, world: &World) -> Transform {
        Transform {
            position: self.position.get(world),
            scale: self.scale.get(world),
            rotation: self.rotation.get(world),
            anchor: self.anchor.get(world),
        }
    }
}

impl From<Transform> for DynamicTransform {
    fn from(transform: Transform) -> Self {
        DynamicTransform {
            position: DynamicPos::Literal(transform.position),
            scale: DynamicValue::Literal(transform.scale),
            rotation: DynamicValue::Literal(transform.rotation),
            anchor: DynamicPos::Literal(transform.anchor),
        }
    }
}
