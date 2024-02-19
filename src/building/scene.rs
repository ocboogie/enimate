use crate::{
    building::{Builder, BuilderState},
    motion::{Keyframe, Motion, MotionId, Sequence},
    object::ObjectId,
    scene::Scene,
};

pub struct SceneBuilder {
    state: BuilderState,
    root_motions: Vec<(MotionId, f32)>,
}

impl SceneBuilder {
    pub fn new(scene_length: f32) -> Self {
        Self {
            state: BuilderState::new(scene_length),
            root_motions: Vec::new(),
        }
    }

    pub fn finish(self) -> Scene {
        let mut motions = self.state.motions;
        let scene_length = self.state.scene_length;
        let sequence_id = rand::random::<ObjectId>();

        let sequence = Sequence(self.root_motions);

        let time_converter = Keyframe {
            from_min: 0.0,
            from_max: scene_length,
            to_min: 0.0,
            to_max: 1.0,
            motion: sequence_id,
        };
        let root_id = rand::random::<ObjectId>();

        motions.insert(sequence_id, Box::new(sequence));
        motions.insert(root_id, Box::new(time_converter));

        Scene::new(motions, root_id, scene_length)
    }
}

impl Builder for SceneBuilder {
    fn state(&mut self) -> &mut BuilderState {
        &mut self.state
    }

    fn play(&mut self, motion: Box<dyn Motion>, duration: f32) -> MotionId {
        self.state.emulate_motion(motion.as_ref());
        let motion_id = self.add_motion(motion);
        self.root_motions
            .push((motion_id, self.state.normalize_time(duration)));
        motion_id
    }
}
