use crate::{
    animation::{self, Animation},
    builder::Builder,
    object::{Object, ObjectId},
    scene::Scene,
    world::ObjectTree,
};

pub struct SceneBuilder {
    pub world: ObjectTree,
    pub animations: Vec<Box<dyn Animation>>,
    pub current_time: f32,
    pub scene_length: f32,
}

impl SceneBuilder {
    pub fn new(scene_length: f32) -> Self {
        Self {
            world: ObjectTree::new(),
            animations: Vec::new(),
            current_time: 0.0,
            scene_length,
        }
    }

    pub fn finish(self) -> Scene {
        Scene::new(
            self.world,
            Box::new(animation::Parallel::new(self.animations)),
        )
    }
}

impl Builder for SceneBuilder {
    fn add_object(&mut self, id: ObjectId, object: Object, rooted: bool) {
        self.world.add_object(id, object, rooted);
    }

    fn add_animation(&mut self, animation: Box<dyn Animation>) {
        // TODO:
        self.animations.push(animation);
    }
}
