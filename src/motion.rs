use egui::Color32;

use crate::motion_ui::MotionUi;
use crate::object::{Object, ObjectKind};
use crate::scene::Scene;
use crate::world::World;
use crate::{object::Transform, world::ObjectTree};

pub type MotionId = usize;

/// A motion is the most basic momvement primitive. It is a function that takes a world and a time
/// and updates the world, adding objects, mutating them, or animating them. The time is a value
/// between 0 and 1.
pub trait Motion: MotionUi {
    fn animate(&self, world: &mut World);
}

pub struct Noop;

impl Motion for Noop {
    fn animate(&self, _world: &mut World) {}
}

pub struct Parallel {
    pub motions: Vec<MotionId>,
}

impl Parallel {
    /// Order matters!
    pub fn new(motions: Vec<MotionId>) -> Self {
        Self { motions }
    }
}

impl Motion for Parallel {
    fn animate(&self, world: &mut World) {
        for motion in &self.motions {
            world.play(*motion);
        }
    }
}

pub struct Trigger {
    pub time: f32,
    pub motion: MotionId,
}

impl Trigger {
    pub fn new(time: f32, motion: MotionId) -> Self {
        Self { time, motion }
    }
}

impl Motion for Trigger {
    fn animate(&self, world: &mut World) {
        if world.time >= self.time {
            world.play_at(self.motion, 1.0);
        }
    }
}

pub struct Keyframe {
    pub from_min: f32,
    pub from_max: f32,
    pub to_min: f32,
    pub to_max: f32,

    pub motion: MotionId,
}

impl Keyframe {
    pub fn new(from_min: f32, from_max: f32, to_min: f32, to_max: f32, motion: MotionId) -> Self {
        Self {
            from_min,
            from_max,
            to_min,
            to_max,
            motion,
        }
    }
}

impl Motion for Keyframe {
    fn animate(&self, world: &mut World) {
        let mut adjusted_time = (world.time - self.from_min) / (self.from_max - self.from_min);

        // We don't want to run the animate function because, for example, the AddObject motion
        // relies on not being run if the time is out of bounds.
        if adjusted_time < 0.0 {
            return;
        }

        // We want to clamp the time to 1.0, so that once we've pasted the end of the motion,
        // it still runs, ensuring AddObject is run, and it doesn't screw up animations like
        // AnimateTransform.
        adjusted_time = adjusted_time.min(1.0);

        adjusted_time *= (self.to_max - self.to_min) + self.to_min;

        world.play_at(self.motion, adjusted_time);
    }
}

pub struct AnimateTransform {
    pub object_id: usize,
    pub from: Transform,
    pub to: Transform,
}

impl AnimateTransform {
    pub fn new(object_id: usize, from: Transform, to: Transform) -> Self {
        Self {
            object_id,
            from,
            to,
        }
    }
}

impl Motion for AnimateTransform {
    fn animate(&self, world: &mut World) {
        let object = world.objects.get_mut(&self.object_id).unwrap();

        let time = world.time;

        object.transform = Transform {
            position: time * (self.to.position - self.from.position.to_vec2())
                + self.from.position.to_vec2(),
            scale: time * (self.to.scale - self.from.scale) + self.from.scale,
            rotation: time * (self.to.rotation - self.from.rotation) + self.from.rotation,
            anchor: time * (self.to.anchor - self.from.anchor.to_vec2())
                + self.from.anchor.to_vec2(),
        };
    }
}

pub struct AddObject {
    pub object_id: usize,
    pub object: Object,
    pub rooted: bool,
}

impl Motion for AddObject {
    fn animate(&self, world: &mut World) {
        world
            .objects
            .add(self.object_id, self.object.clone(), self.rooted);
    }
}

pub struct FadeIn {
    pub object_id: usize,
}

fn fade_in(world: &mut World, object_id: usize) {
    let mut object = world.objects.remove(&object_id).unwrap();

    match &mut object.object_kind {
        ObjectKind::Model(ref mut model) => {
            let c = model.material.color;
            model.material.color =
                Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), (world.time * 255.0) as u8);
        }
        ObjectKind::Group(group) => {
            for child_id in group.iter() {
                fade_in(world, *child_id);
            }
        }
    }

    world.objects.add(object_id, object, false);
}

impl Motion for FadeIn {
    fn animate(&self, world: &mut World) {
        // FIXME: We should be able to animate the alpha value of the color.
        fade_in(world, self.object_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ExpectTime(f32);

    impl Motion for ExpectTime {
        fn animate(&self, world: &mut ObjectTree, time: f32) {
            assert_eq!(self.0, time);
        }
    }

    #[test]
    fn test_keyframe() {
        let mut world = ObjectTree::default();

        Keyframe::new(0.0, 1.0, 0.0, 1.0, Box::new(ExpectTime(0.0))).animate(&mut world, 0.0);
        Keyframe::new(0.0, 1.0, 0.0, 1.0, Box::new(ExpectTime(0.5))).animate(&mut world, 0.5);
        Keyframe::new(0.0, 1.0, 1.0, 2.0, Box::new(ExpectTime(1.5))).animate(&mut world, 0.5);
        Keyframe::new(5.0, 10.0, 0.0, 1.0, Box::new(ExpectTime(0.5))).animate(&mut world, 7.0);
    }
}
