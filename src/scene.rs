use crate::{animation::Animation, object::Object, world::World};

#[derive(Default)]
pub struct Scene {
    world: World,
    animations: Vec<Box<dyn Animation>>,
}

impl Scene {
    pub fn new(world: World, animations: Vec<Box<dyn Animation>>) -> Self {
        Self { world, animations }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        // for animation in &self.animations {
        //     animation.animate(&mut self.world);
        // }

        // self.world.update(ctx, frame);
    }
}
