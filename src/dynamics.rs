use egui::{Pos2, Vec2};

use crate::{
    object::Transform,
    world::{Variable, World},
};

pub enum DynamicValue {
    Variable(Variable),
    Literal(f32),
}

impl DynamicValue {
    pub fn value(&self, world: &World) -> f32 {
        match self {
            DynamicValue::Variable(var) => world.get_variable(*var),
            DynamicValue::Literal(value) => *value,
        }
    }
}

impl From<f32> for DynamicValue {
    fn from(value: f32) -> Self {
        DynamicValue::Literal(value)
    }
}

pub struct DynamicVector {
    pub x: DynamicValue,
    pub y: DynamicValue,
}

impl From<Vec2> for DynamicVector {
    fn from(vec: Vec2) -> Self {
        DynamicVector {
            x: DynamicValue::Literal(vec.x),
            y: DynamicValue::Literal(vec.y),
        }
    }
}

impl From<Pos2> for DynamicVector {
    fn from(pos: Pos2) -> Self {
        DynamicVector {
            x: DynamicValue::Literal(pos.x),
            y: DynamicValue::Literal(pos.y),
        }
    }
}

impl DynamicVector {
    pub fn to_vec2(&self, world: &World) -> egui::Vec2 {
        egui::vec2(self.x.value(world), self.y.value(world))
    }

    pub fn to_pos2(&self, world: &World) -> egui::Pos2 {
        egui::pos2(self.x.value(world), self.y.value(world))
    }
}

pub struct DynamicTransform {
    pub position: DynamicVector,
    pub scale: DynamicValue,
    pub rotation: DynamicValue,
    pub anchor: DynamicVector,
}

impl From<Transform> for DynamicTransform {
    fn from(transform: Transform) -> Self {
        DynamicTransform {
            position: DynamicVector::from(transform.position),
            scale: DynamicValue::Literal(transform.scale),
            rotation: DynamicValue::Literal(transform.rotation),
            anchor: DynamicVector::from(transform.anchor),
        }
    }
}

impl DynamicTransform {
    pub fn to_transform(&self, world: &World, time: f32) -> Transform {
        Transform {
            position: self.position.to_pos2(world),
            scale: self.scale.value(world),
            rotation: self.rotation.value(world),
            anchor: self.anchor.to_pos2(world),
        }
    }
}
