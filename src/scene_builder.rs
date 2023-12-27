use std::collections::HashMap;

use crate::{
    builder::{Builder, BuilderState},
    motion::{self, AddObject, Keyframe, Motion, MotionId, Parallel, Trigger},
    object::{Object, ObjectId},
    scene::Scene,
    world::{ObjectTree, World},
};

pub struct SceneBuilder {
    state: BuilderState,
}

impl SceneBuilder {
    pub fn new(scene_length: f32) -> Self {
        Self {
            state: BuilderState::new(scene_length),
        }
    }

    fn create_root_motion(&self) -> Box<dyn Motion> {
        Box::new(Parallel::new(self.state.root_motions.clone()))
    }

    pub fn finish(mut self) -> Scene {
        let root = self.create_root_motion();
        let root_id = rand::random::<usize>();

        self.state.motions.insert(root_id, root);

        Scene::new(self.state.motions, root_id)
    }
}

impl Builder for SceneBuilder {
    fn state(&mut self) -> &mut BuilderState {
        &mut self.state
    }
}
