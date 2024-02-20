use egui::Color32;

use crate::animation::{Animation, GenericAnimation, Time};
use crate::dynamics::{DynamicPos, DynamicTransform, DynamicValue};
use crate::object::{Object, ObjectId, ObjectKind};

use crate::object::Transform as ObjectTransform;
use crate::utils::rotate_vec_around_vector;
use crate::world::{Variable, World};

pub type MotionId = usize;

pub type Alpha = f32;

/// A motion is the most basic momvement primitive. It is a function that takes a world
/// and updates the world, adding objects, mutating them, or animating them. The time is a value
/// between 0 and 1.
pub trait Motion {
    fn animate(&self, world: &mut World, alpha: Alpha);
}

impl Motion for Box<dyn Motion> {
    fn animate(&self, world: &mut World, alpha: Alpha) {
        self.animate(world, alpha);
    }
}

pub struct Wait;

impl Motion for Wait {
    fn animate(&self, _world: &mut World, _alpha: Alpha) {}
}

pub struct Sequence(
    /// All the durations must add up to 1.0.
    pub Vec<GenericAnimation>,
);

impl Motion for Sequence {
    fn animate(&self, world: &mut World, alpha: Alpha) {
        // FIXME:
        let mut current_alpha = 0.0;

        for animation in &self.0 {
            let duration = animation.duration();

            let adusted_alpha = (alpha - current_alpha) / duration;

            if adusted_alpha < 0.0 {
                return;
            }

            animation.animate(world, adusted_alpha.min(1.0));
            current_alpha += duration;
        }
    }
}

pub struct Concurrently(
    /// Order matters!
    pub Vec<GenericAnimation>,
);

impl Motion for Concurrently {
    fn animate(&self, world: &mut World, alpha: Alpha) {
        for animation in &self.0 {
            animation.animate(world, alpha);
        }
    }
}

// pub struct Keyframe {
//     pub from_min: Time,
//     pub from_max: f32,
//     pub to_min: f32,
//     pub to_max: f32,
//
//     pub motion: MotionId,
// }
//
// impl Keyframe {
//     pub fn new(from_min: f32, from_max: f32, to_min: f32, to_max: f32, motion: MotionId) -> Self {
//         Self {
//             from_min,
//             from_max,
//             to_min,
//             to_max,
//             motion,
//         }
//     }
// }
//
// impl Motion for Keyframe {
//     fn animate(&self, world: &mut World, time: Alpha) {
//         let mut adjusted_time = (time - self.from_min) / (self.from_max - self.from_min);
//
//         // We don't want to run the animate function because, for example, the AddObject motion
//         // relies on not being run if the time is out of bounds.
//         if adjusted_time < 0.0 {
//             return;
//         }
//
//         // We want to clamp the time to 1.0, so that once we've pasted the end of the motion,
//         // it still runs, ensuring AddObject is run, and it doesn't screw up animations like
//         // AnimateTransform.
//         adjusted_time = adjusted_time.min(1.0);
//
//         adjusted_time *= (self.to_max - self.to_min) + self.to_min;
//
//         world.play_at(self.motion, adjusted_time);
//     }
// }

pub struct AddObject {
    pub object_id: usize,
    pub object: Object,
    pub rooted: bool,
}

impl Motion for AddObject {
    fn animate(&self, world: &mut World, _alpha: Alpha) {
        world
            .objects
            .add(self.object_id, self.object.clone(), self.rooted);
    }
}

pub struct FadeIn {
    pub object_id: usize,
}

fn fade_in(world: &mut World, alpha: Alpha, object_id: usize) {
    let mut object = world.objects.remove(&object_id).unwrap();

    match &mut object.object_kind {
        ObjectKind::Model(ref mut model) => {
            if let Some(ref mut fill) = model.material.fill {
                let c = fill.color;
                fill.color =
                    Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), (alpha * 255.0) as u8);
            }

            if let Some(ref mut stroke) = model.material.stroke {
                let c = stroke.color;
                stroke.color =
                    Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), (alpha * 255.0) as u8);
            }
        }
        ObjectKind::Group(group) => {
            for child_id in group.iter() {
                fade_in(world, alpha, *child_id);
            }
        }
    }

    world.objects.add(object_id, object, false);
}

impl Motion for FadeIn {
    fn animate(&self, world: &mut World, alpha: f32) {
        // FIXME: We should be able to animate the alpha value of the color.
        fade_in(world, alpha, self.object_id);
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
