use std::collections::HashMap;

use crate::{
    animation::{Animation, GenericAnimation, MotionAnimation, Time},
    building::{Builder, Component},
    motion::{Alpha, Motion, MotionId, Sequence, Wait},
    object::ObjectId,
    object_tree::ObjectTree,
    world::{Variable, World},
};

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
