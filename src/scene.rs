use crate::{
    animation::{Animation, MotionAnimation},
    builder::{Builder, BuilderState},
    motion::{Alpha, Motion},
    object_tree::ObjectTree,
    timing::{Sequence, Time, Wait},
    world::{Variable, World},
};
use std::collections::HashMap;

pub struct Scene(pub Sequence);

impl Scene {
    pub fn null() -> Self {
        Self(Sequence(vec![Box::new(MotionAnimation {
            duration: 0.0,
            motion: Wait,
        })]))
    }

    pub fn length(&self) -> Time {
        self.0.duration()
    }

    pub fn time_to_alpha(&self, time: Time) -> Alpha {
        time / self.length()
    }

    pub fn render_at(&self, time: Time) -> ObjectTree {
        let mut objects = ObjectTree::new();
        let mut world = World::new(&mut objects, HashMap::new());

        self.0.animate(&mut world, self.time_to_alpha(time));

        objects
    }

    pub fn render_with_input(&mut self, time: f32, input: HashMap<Variable, f32>) -> ObjectTree {
        let mut objects = ObjectTree::new();
        let mut world = World::new(&mut objects, input);

        self.0.animate(&mut world, self.time_to_alpha(time));

        objects
    }
}

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
