use crate::{animation::Animation, world::ObjectTree};

pub struct Scene {
    pub world: ObjectTree,
    pub animation: Box<dyn Animation>,
}

impl Scene {
    pub fn new(world: ObjectTree, animation: Box<dyn Animation>) -> Self {
        Self { world, animation }
    }

    pub fn update(&mut self, time: f32) {
        self.animation.animate(&mut self.world, time);
    }
}
