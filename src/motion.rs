use egui::Color32;

use crate::dynamics::{DynamicTransform, DynamicValue, DynamicVector};
use crate::object::{Object, ObjectId, ObjectKind};

use crate::motion_ui::MotionUi;
use crate::object::Transform as ObjectTransform;
use crate::utils::rotate_vec_around_vector;
use crate::world::{Variable, World};

pub type MotionId = usize;

/// A motion is the most basic momvement primitive. It is a function that takes a world
/// and updates the world, adding objects, mutating them, or animating them. The time is a value
/// between 0 and 1.
pub trait Motion: MotionUi {
    fn animate(&self, world: &mut World, time: f32);
}

pub struct NoOp;

impl Motion for NoOp {
    fn animate(&self, _world: &mut World, _time: f32) {}
}

pub struct Sequence {
    /// All the durations must add up to 1.0.
    pub motions: Vec<(f32, MotionId)>,
}

impl Motion for Sequence {
    fn animate(&self, world: &mut World, time: f32) {
        let mut current_time = 0.0;

        for (duration, motion) in &self.motions {
            let adusted_time = (time - current_time) / duration;

            if adusted_time < 0.0 {
                return;
            }

            world.play_at(*motion, adusted_time.min(1.0));
            current_time += duration;
        }
    }
}

pub struct Parallel {
    /// Order matters!
    pub motions: Vec<MotionId>,
}

impl Motion for Parallel {
    fn animate(&self, world: &mut World, time: f32) {
        for motion in &self.motions {
            world.play_at(*motion, time);
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
    fn animate(&self, world: &mut World, time: f32) {
        if time >= self.time {
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
    fn animate(&self, world: &mut World, time: f32) {
        let mut adjusted_time = (time - self.from_min) / (self.from_max - self.from_min);

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
    pub from: DynamicTransform,
    pub to: DynamicTransform,
}

impl Motion for AnimateTransform {
    fn animate(&self, world: &mut World, time: f32) {
        let from = self.from.to_transform(world, time);
        let to = self.to.to_transform(world, time);

        let object = world.objects.get_mut(&self.object_id).unwrap();

        object.transform = ObjectTransform {
            position: time * (to.position - from.position.to_vec2()) + from.position.to_vec2(),
            scale: time * (to.scale - from.scale) + from.scale,
            rotation: time * (to.rotation - from.rotation) + from.rotation,
            anchor: time * (to.anchor - from.anchor.to_vec2()) + from.anchor.to_vec2(),
        };
    }
}

pub struct SetTransform {
    pub object_id: usize,
    pub transform: DynamicTransform,
}

impl Motion for SetTransform {
    fn animate(&self, world: &mut World, time: f32) {
        let transform = self.transform.to_transform(world, time);
        let object = world.objects.get_mut(&self.object_id).unwrap();

        object.transform = transform;
    }
}

pub struct Rotate {
    pub object_id: usize,
    pub around: DynamicVector,
    pub from: DynamicValue,
    pub to: DynamicValue,
}

impl Motion for Rotate {
    fn animate(&self, world: &mut World, time: f32) {
        let from = self.from.value(world);
        let to = self.to.value(world);

        let around = self.around.to_pos2(world);

        let object = world.objects.get_mut(&self.object_id).unwrap();

        let rotation = time * (to - from) + from;

        object.transform.position =
            rotate_vec_around_vector(object.transform.position, rotation, around);
    }
}

// pub struct Transform {
//     pub object_id: ObjectId,
//     pub transform: ObjectTransform,
// }
//
// impl Motion for Transform {
//     fn animate(&self, world: &mut World, time: f32) {
//         let object = world.objects.get_mut(&self.object_id).unwrap();
//
//         object.transform = self.transform;
//     }
// }

pub struct AddObject {
    pub object_id: usize,
    pub object: Object,
    pub rooted: bool,
}

impl Motion for AddObject {
    fn animate(&self, world: &mut World, time: f32) {
        world
            .objects
            .add(self.object_id, self.object.clone(), self.rooted);
    }
}

// Set variable to current time,
pub struct SetVariable {
    pub var: Variable,
}

impl Motion for SetVariable {
    fn animate(&self, world: &mut World, time: f32) {
        world.update_variable(self.var, time);
    }
}

// pub struct VariableSetter<M: Fn(&World, f32) -> f32> {
//     pub ident: Variable,
//     pub motion: M,
// }
//
// impl<M: Fn(&World, f32) -> f32> Motion for VariableSetter<M> {
//     fn animate(&self, world: &mut World, time: f32) {
//         let value = (self.motion)(world, time);
//
//         world.update_variable(self.ident, value);
//     }
// }

pub struct FadeIn {
    pub object_id: usize,
}

fn fade_in(world: &mut World, time: f32, object_id: usize) {
    let mut object = world.objects.remove(&object_id).unwrap();

    match &mut object.object_kind {
        ObjectKind::Model(ref mut model) => {
            if let Some(ref mut fill) = model.material.fill {
                let c = fill.color;
                fill.color =
                    Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), (time * 255.0) as u8);
            }

            if let Some(ref mut stroke) = model.material.stroke {
                let c = stroke.color;
                stroke.color =
                    Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), (time * 255.0) as u8);
            }
        }
        ObjectKind::Group(group) => {
            for child_id in group.iter() {
                fade_in(world, time, *child_id);
            }
        }
    }

    world.objects.add(object_id, object, false);
}

impl Motion for FadeIn {
    fn animate(&self, world: &mut World, time: f32) {
        // FIXME: We should be able to animate the alpha value of the color.
        fade_in(world, time, self.object_id);
    }
}

#[cfg(test)]
mod tests {
    use crate::object_tree::ObjectTree;
    use crate::scene::Scene;
    use std::collections::HashMap;

    use super::*;

    struct ExpectTime(f32);

    // impl MotionUi for ExpectTime {
    //     fn ui(&mut self, _ui: &mut egui::Ui, _scene: &mut Scene) {}
    // }

    impl Motion for ExpectTime {
        fn animate(&self, world: &mut World, time: f32) {
            assert_eq!(self.0, time);
        }
    }

    fn add_motion(
        motions: &mut HashMap<MotionId, Box<dyn Motion>>,
        motion: impl Motion + 'static,
    ) -> MotionId {
        let id = rand::random::<usize>();
        motions.insert(id, Box::new(motion));
        id
    }

    #[test]
    fn test_sequence() {
        let motions = &mut HashMap::new();
        let expect_time = add_motion(motions, ExpectTime(1.0));
        let expect_time_2 = add_motion(motions, ExpectTime(1.0));
        let expect_time_3 = add_motion(motions, ExpectTime(0.5));
        let seq = add_motion(
            motions,
            Sequence {
                motions: (vec![
                    (0.5, expect_time),
                    (0.25, expect_time_2),
                    (0.25, expect_time_3),
                ]),
            },
        );

        let mut tree = ObjectTree::new();

        let mut world = World::new(&mut tree, &motions);

        world.play_at(seq, 0.875);
    }
}
