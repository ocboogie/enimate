use crate::{
    motion::{Motion, MotionId},
    object_tree::ObjectTree,
    world::World,
};

use std::collections::HashMap;

mod builder;
mod object;
mod scene;
mod spatial;
mod temporal;

pub use builder::*;
pub use object::*;
pub use scene::*;
pub use spatial::*;
pub use temporal::*;

pub struct BuilderState {
    pub motions: HashMap<MotionId, Box<dyn Motion>>,
    pub objects: ObjectTree,
    pub scene_length: f32,
}

impl BuilderState {
    pub fn new(scene_length: f32) -> Self {
        Self {
            motions: HashMap::new(),
            objects: ObjectTree::new(),
            scene_length,
        }
    }

    pub fn emulate_motion(&mut self, motion: &dyn Motion) {
        // Run the motion, so the state of objects is consistent with the end of the motion.
        let world = &mut World::new(&mut self.objects, &self.motions);
        motion.animate(world, 1.0);
    }

    pub fn normalize_time(&self, time: f32) -> f32 {
        time / self.scene_length
    }
}
