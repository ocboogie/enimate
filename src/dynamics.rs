use dyn_clone::DynClone;
use egui::Pos2;

use crate::{
    object::{Object, ObjectId, ObjectKind, Transform},
    world::{Variable, World},
};

pub trait DynamicType<T>: DynClone {
    fn get(&self, world: &World) -> T;

    fn d(self) -> Dynamic<T>
    where
        Self: Sized + 'static,
    {
        Dynamic(Box::new(self))
    }
}

pub struct Dynamic<T>(Box<dyn DynamicType<T>>);

impl<T> Dynamic<T> {
    pub fn new(dynamic: impl DynamicType<T> + 'static) -> Self {
        Self(Box::new(dynamic))
    }
}

impl<T> Clone for Dynamic<T> {
    fn clone(&self) -> Self {
        Dynamic(dyn_clone::clone_box(&*self.0))
    }
}

impl<T> DynamicType<T> for Dynamic<T> {
    fn get(&self, world: &World) -> T {
        self.0.get(world)
    }
}

impl DynamicType<f32> for Variable {
    fn get(&self, world: &World) -> f32 {
        world.get_variable(*self)
    }
}

// impl<T> From<DynamicType<T>> for Dynamic<T> {
//     fn from(value: Box<dyn DynamicType<T>>) -> Self {
//         Dynamic::Dynamic(value)
//     }
// }

// impl<T> Dynamic<T> for DynamicValue<T> {
//     fn get(&self, world: &World) -> T {
//         match self {
//             DynamicValue::Literal(value) => *value,
//             DynamicValue::Dynamic(dynamic) => dynamic.get(world),
//         }
//     }
// }
//
// impl<T> From<T> for DynamicValue<T> {
//     fn from(value: T) -> Self {
//         DynamicValue::Literal(value)
//     }
// }

// impl<T, D: Dynamic<T>> From<D> for DynamicValue<T> {
//     fn from(value: D) -> Self {
//         DynamicValue::Dynamic(Box::new(value))
//     }
// }

impl DynamicType<f32> for f32 {
    fn get(&self, _: &World) -> f32 {
        *self
    }
}

impl DynamicType<Pos2> for (Dynamic<f32>, Dynamic<f32>) {
    fn get(&self, world: &World) -> Pos2 {
        Pos2::new(self.0.get(world), self.1.get(world))
    }
}

impl DynamicType<Pos2> for Pos2 {
    fn get(&self, _: &World) -> Pos2 {
        *self
    }
}

#[derive(Clone)]
pub struct DynamicTransform {
    pub position: Dynamic<Pos2>,
    pub scale: Dynamic<f32>,
    pub rotation: Dynamic<f32>,
    pub anchor: Dynamic<Pos2>,
}

impl DynamicType<Transform> for DynamicTransform {
    fn get(&self, world: &World) -> Transform {
        Transform {
            position: self.position.get(world),
            scale: self.scale.get(world),
            rotation: self.rotation.get(world),
            anchor: self.anchor.get(world),
        }
    }
}

impl DynamicType<Transform> for Transform {
    fn get(&self, _: &World) -> Transform {
        *self
    }
}

impl From<Transform> for DynamicTransform {
    fn from(transform: Transform) -> Self {
        Self {
            position: transform.position.d(),
            scale: transform.scale.d(),
            rotation: transform.rotation.d(),
            anchor: transform.anchor.d(),
        }
    }
}

#[derive(Clone)]
pub struct DynamicObject {
    object_kind: ObjectKind,
    transform: DynamicTransform,
}

impl DynamicType<Object> for DynamicObject {
    fn get(&self, world: &World) -> Object {
        Object {
            object_kind: self.object_kind.clone(),
            transform: self.transform.get(world),
        }
    }
}

impl From<Object> for DynamicObject {
    fn from(object: Object) -> Self {
        Self {
            object_kind: object.object_kind,
            transform: object.transform.into(),
        }
    }
}

impl DynamicObject {
    pub fn new_group(objects: Vec<ObjectId>) -> Self {
        Self {
            object_kind: ObjectKind::Group(objects),
            transform: Transform::default().into(),
        }
    }

    pub fn with_transform(mut self, transform: impl Into<DynamicTransform>) -> Self {
        self.transform = transform.into();
        self
    }
}
