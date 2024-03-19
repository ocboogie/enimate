use dyn_clone::DynClone;
use egui::Pos2;

use crate::{
    dynamics::{Dynamic, DynamicType},
    interpolation::Interpolatable,
    motion::{Alpha, Motion},
    object::{ObjectId, Transform},
    trigger::Trigger,
    world::World,
};

pub trait PropertyUpdater<T: Interpolatable> {
    fn update(&self, world: &mut World, value: T);
}

pub trait PropertyType<T: Interpolatable>: DynClone + DynamicType<T> + PropertyUpdater<T> {}

impl<T: Interpolatable, U: DynClone + DynamicType<T> + PropertyUpdater<T>> PropertyType<T> for U {}

pub struct Property<T: Interpolatable>(Box<dyn PropertyType<T>>);

// pub struct OwnedProperty<T: Interpolatable> {
//     value: T,
//     update: fn(&mut World, T),
// }
//
// impl<T: Interpolatable> DynamicType<T> for OwnedProperty<T> {
//     fn get(&self, _world: &World) -> T {
//         self.value
//     }
// }
//
// impl<T: Interpolatable> PropertyType<T> for OwnedProperty<T> {
//     fn set(&self, _world: &mut World, value: T) {
//         self.value = value;
//     }
// }

impl<T: Interpolatable> Clone for Property<T> {
    fn clone(&self) -> Self {
        Self(dyn_clone::clone_box(&*self.0))
    }
}

impl<T: Interpolatable> Property<T> {
    pub fn new(property: impl PropertyType<T> + 'static) -> Self {
        Self(Box::new(property))
    }
}

struct PropertyFromTo<T: Interpolatable> {
    property: Property<T>,
    from: Dynamic<T>,
    to: Dynamic<T>,
}

impl<T: Interpolatable> Motion for PropertyFromTo<T> {
    fn animate(&self, world: &mut World, alpha: Alpha) {
        let from = self.from.get(world);
        let to = self.to.get(world);

        let value = from.interpolate(&to, alpha);
        self.property.0.update(world, value);
    }
}

struct PropertyTo<T: Interpolatable> {
    property: Property<T>,
    to: Dynamic<T>,
}

impl<T: Interpolatable> Motion for PropertyTo<T> {
    fn animate(&self, world: &mut World, alpha: Alpha) {
        let from = self.property.0.get(world);
        let to = self.to.get(world);

        let value = from.interpolate(&to, alpha);
        self.property.0.update(world, value);
    }
}

struct PropertySet<T: Interpolatable> {
    property: Property<T>,
    value: Dynamic<T>,
}

impl<T: Interpolatable> Trigger for PropertySet<T> {
    fn trigger(&self, world: &mut World) {
        self.property.0.update(world, self.value.get(world));
    }
}

impl<T: Interpolatable> Property<T> {
    pub fn set(&self, value: Dynamic<T>) -> impl Trigger {
        PropertySet {
            property: self.clone(),
            value,
        }
    }

    pub fn animate_from(&self, from: Dynamic<T>, to: Dynamic<T>) -> impl Motion {
        PropertyFromTo {
            property: self.clone(),
            from,
            to,
        }
    }

    pub fn animate(&self, to: Dynamic<T>) -> impl Motion {
        PropertyTo {
            property: self.clone(),
            to,
        }
    }
}

#[derive(Clone)]
pub struct Position(pub ObjectId);

impl DynamicType<Pos2> for Position {
    fn get(&self, world: &World) -> Pos2 {
        world.objects.get(&self.0).unwrap().transform.position
    }
}

impl PropertyUpdater<Pos2> for Position {
    fn update(&self, world: &mut World, value: Pos2) {
        world.objects.get_mut(&self.0).unwrap().transform.position = value;
    }
}

#[derive(Clone)]
pub struct Scale(pub ObjectId);

impl DynamicType<f32> for Scale {
    fn get(&self, world: &World) -> f32 {
        world.objects.get(&self.0).unwrap().transform.scale
    }
}

impl PropertyUpdater<f32> for Scale {
    fn update(&self, world: &mut World, value: f32) {
        world.objects.get_mut(&self.0).unwrap().transform.scale = value;
    }
}

#[derive(Clone)]
pub struct Rotation(pub ObjectId);

impl DynamicType<f32> for Rotation {
    fn get(&self, world: &World) -> f32 {
        world.objects.get(&self.0).unwrap().transform.rotation
    }
}

impl PropertyUpdater<f32> for Rotation {
    fn update(&self, world: &mut World, value: f32) {
        world.objects.get_mut(&self.0).unwrap().transform.rotation = value;
    }
}

#[derive(Clone)]
pub struct Anchor(pub ObjectId);

impl DynamicType<Pos2> for Anchor {
    fn get(&self, world: &World) -> Pos2 {
        world.objects.get(&self.0).unwrap().transform.anchor
    }
}

impl PropertyUpdater<Pos2> for Anchor {
    fn update(&self, world: &mut World, value: Pos2) {
        world.objects.get_mut(&self.0).unwrap().transform.anchor = value;
    }
}

#[derive(Clone)]
pub struct TransformProperty(pub ObjectId);

impl DynamicType<Transform> for TransformProperty {
    fn get(&self, world: &World) -> Transform {
        world.objects.get(&self.0).unwrap().transform
    }
}

impl PropertyUpdater<Transform> for TransformProperty {
    fn update(&self, world: &mut World, value: Transform) {
        world.objects.get_mut(&self.0).unwrap().transform = value;
    }
}

impl TransformProperty {
    pub fn position(&self) -> Property<Pos2> {
        Property::new(Position(self.0))
    }

    pub fn scale(&self) -> Property<f32> {
        Property::new(Scale(self.0))
    }

    pub fn rotation(&self) -> Property<f32> {
        Property::new(Rotation(self.0))
    }

    pub fn anchor(&self) -> Property<Pos2> {
        Property::new(Anchor(self.0))
    }
}
