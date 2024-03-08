use egui::Pos2;

use crate::{
    object::Transform,
    world::{Variable, World},
};

pub trait DynamicType<T> {
    fn get(&self, world: &World) -> T;

    fn d(self) -> Dynamic<T>
    where
        Self: Sized + 'static,
    {
        Dynamic(Box::new(self))
    }
}

pub struct Dynamic<T>(Box<dyn DynamicType<T>>);

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
