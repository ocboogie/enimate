use std::collections::HashMap;

use crate::motion::{Motion, MotionId};
use crate::object_tree::ObjectTree;

pub struct World<'a> {
    pub objects: &'a mut ObjectTree,
    motions: &'a HashMap<MotionId, Box<dyn Motion>>,
    pub time: f32,
}

impl<'a> World<'a> {
    pub fn new(
        time: f32,
        objects: &'a mut ObjectTree,
        motions: &'a HashMap<MotionId, Box<dyn Motion>>,
    ) -> Self {
        Self {
            objects,
            motions,
            time,
        }
    }

    pub fn play(&mut self, motion: MotionId) {
        // TODO: Provide a warning here somehow if the motion doesn't exist.
        let motion = self.motions.get(&motion).unwrap();
        motion.animate(self);
    }

    pub fn play_at(&mut self, motion: MotionId, time: f32) {
        let motion = self.motions.get(&motion).unwrap();
        let current_time = self.time;
        self.time = time;
        motion.animate(self);
        self.time = current_time;
    }
}
