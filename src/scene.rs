use crate::{animation::Animation, world::World};

pub struct Scene {
    pub world: World,
    pub animation: Box<dyn Animation>,
}

impl Scene {
    pub fn new(world: World, animation: Box<dyn Animation>) -> Self {
        Self { world, animation }
    }

    pub fn update(&mut self, time: f32) {
        self.animation.animate(&mut self.world, time);
    }
}
