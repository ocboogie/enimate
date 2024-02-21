use crate::{
    motion::{Motion, MotionId},
    object_tree::ObjectTree,
    scene::Scene,
    world::World,
};

use std::collections::HashMap;

mod builder;
mod component;
mod object;
mod scene;
mod spatial;
// mod temporal;

pub use builder::*;
pub use component::*;
pub use object::*;
pub use scene::*;
pub use spatial::*;
// pub use temporal::*;

pub struct BuilderState {
    pub scene: Scene,
    pub objects: ObjectTree,
}

impl BuilderState {
    pub fn new() -> Self {
        Self {
            scene: Scene::null(),
            objects: ObjectTree::new(),
        }
    }

    pub fn emulate_motion(&mut self, motion: &dyn Motion) {
        // Run the motion, so the state of objects is consistent with the end of the motion.
        let world = &mut World::new(&mut self.objects, HashMap::new());
        motion.animate(world, 1.0);
    }
}
