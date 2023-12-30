use std::collections::HashMap;

use crate::{
    builder::{Builder, BuilderState},
    motion::{self, AddObject, Keyframe, Motion, MotionId, Parallel, Trigger},
    object::{Object, ObjectId},
    object_tree::ObjectTree,
    scene::Scene,
    world::World,
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

    pub fn finish(mut self) -> Scene {
        let parallel_id = rand::random::<usize>();
        let parallel = Parallel::new(self.state.root_motions.clone());
        self.state.motions.insert(parallel_id, Box::new(parallel));

        // let keyframe_id = rand::random::<usize>();
        // let keyframe = Keyframe {
        //     from_min: 0.0,
        //     from_max: self.state.scene_length,
        //     to_min: 0.0,
        //     to_max: 1.0,
        //     motion: parallel_id,
        // };
        // self.state.motions.insert(keyframe_id, Box::new(keyframe));

        Scene::new(self.state.motions, parallel_id, self.state.scene_length)
    }
}

impl Builder for SceneBuilder {
    fn state(&mut self) -> &mut BuilderState {
        &mut self.state
    }
}
