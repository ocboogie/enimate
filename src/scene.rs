use std::collections::HashMap;

use crate::{
    motion::{Motion, MotionId},
    world::{ObjectTree, World},
};

pub struct Scene {
    pub root: MotionId,
    pub motions: HashMap<MotionId, Box<dyn Motion>>,
}

impl Scene {
    pub fn new(motions: HashMap<MotionId, Box<dyn Motion>>, root: MotionId) -> Self {
        Self { root, motions }
    }

    pub fn root(&self) -> &Box<dyn Motion> {
        &self.motions[&self.root]
    }

    pub fn root_mut(&mut self) -> &mut Box<dyn Motion> {
        self.motions.get_mut(&self.root).unwrap()
    }

    pub fn objects_at(&mut self, time: f32) -> ObjectTree {
        let mut objects = ObjectTree::new();
        let mut world = World::new(time, &mut objects, &self.motions);

        self.motions[&self.root].animate(&mut world);

        objects
    }
}
