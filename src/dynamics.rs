use dyn_clone::{clone_box, DynClone};
use egui::Pos2;

use crate::{
    object::{Object, ObjectId, ObjectKind, Transform},
    world::{Variable, World},
};

pub trait Dynamic<T: 'static>: DynClone + 'static {
    fn get(&self, world: &World) -> T;
}

pub struct OwnedDynamic<T: 'static>(Box<dyn Dynamic<T>>);

impl<T: 'static> OwnedDynamic<T> {
    pub fn new(dynamic: impl Dynamic<T>) -> Self {
        OwnedDynamic(Box::new(dynamic))
    }

    pub fn get(&self, world: &World) -> T {
        self.0.get(world)
    }
}

impl<T: 'static> Clone for OwnedDynamic<T> {
    fn clone(&self) -> Self {
        Self(clone_box(&*self.0))
    }
}

// impl<T> DynamicType<T> for Dynamic<T> {
//     fn get(&self, world: &World) -> T {
//         self.0.get(world)
//     }
// }

impl Dynamic<f32> for Variable {
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

impl Dynamic<f32> for f32 {
    fn get(&self, _: &World) -> f32 {
        *self
    }
}

impl Dynamic<Pos2> for Pos2 {
    fn get(&self, _: &World) -> Pos2 {
        *self
    }
}

#[derive(Clone)]
pub struct DynamicTransform {
    pub position: OwnedDynamic<Pos2>,
    pub scale: OwnedDynamic<f32>,
    pub rotation: OwnedDynamic<f32>,
    pub anchor: OwnedDynamic<Pos2>,
}

impl Dynamic<Transform> for DynamicTransform {
    fn get(&self, world: &World) -> Transform {
        Transform {
            position: self.position.get(world),
            scale: self.scale.get(world),
            rotation: self.rotation.get(world),
            anchor: self.anchor.get(world),
        }
    }
}

impl Dynamic<Transform> for Transform {
    fn get(&self, _: &World) -> Transform {
        *self
    }
}

impl From<Transform> for DynamicTransform {
    fn from(transform: Transform) -> Self {
        Self {
            position: OwnedDynamic::new(transform.position),
            scale: OwnedDynamic::new(transform.scale),
            rotation: OwnedDynamic::new(transform.rotation),
            anchor: OwnedDynamic::new(transform.anchor),
        }
    }
}

#[derive(Clone)]
pub struct DynamicObject {
    object_kind: ObjectKind,
    transform: OwnedDynamic<Transform>,
}

impl Dynamic<Object> for DynamicObject {
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
            transform: OwnedDynamic::new(object.transform),
        }
    }
}

impl DynamicObject {
    pub fn new_group(objects: Vec<ObjectId>) -> Self {
        Self {
            object_kind: ObjectKind::Group(objects),
            transform: OwnedDynamic::new(Transform::default()),
        }
    }

    pub fn with_transform(mut self, transform: impl Dynamic<Transform>) -> Self {
        self.transform = OwnedDynamic::new(transform);
        self
    }
}
