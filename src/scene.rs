use std::collections::HashMap;

use crate::{
    animation::{Animation, GenericAnimation, MotionAnimation, Time},
    motion::{Motion, MotionId, Sequence, Wait},
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

    // pub fn sequence(animations: Vec<GenericAnimation>) -> Self {
    //     let duration = animations.iter().map(|a| a.duration).sum();
    // }

    pub fn render_at(&mut self, time: Time) -> ObjectTree {
        let mut objects = ObjectTree::new();
        let mut world = World::new(&mut objects, HashMap::new(), self.length());

        self.0.animate(&mut world, time);

        objects
    }

    pub fn render_with_input(&mut self, time: f32, input: HashMap<Variable, f32>) -> ObjectTree {
        let mut objects = ObjectTree::new();
        let mut world = World::new(&mut objects, input, self.length());

        self.0.animate(&mut world, time);

        objects
    }
}
