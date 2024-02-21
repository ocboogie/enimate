use crate::{
    animation::{Animation, GenericAnimation},
    building::{Builder, BuilderState},
    scene::Scene,
};

pub struct SceneBuilder {
    state: BuilderState,
}

impl SceneBuilder {
    pub fn new() -> Self {
        Self {
            state: BuilderState::new(),
        }
    }

    pub fn finish(self) -> Scene {
        self.state.scene
    }
}

impl Builder for SceneBuilder {
    fn state(&mut self) -> &mut BuilderState {
        &mut self.state
    }

    fn play<A: Animation + 'static>(&mut self, animation: A) {
        self.state.emulate_motion(&animation);
        (self.state.scene.0).0.push(Box::new(animation));
    }
}
