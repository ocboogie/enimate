use crate::{animation::Animation, object::Object, scene::Scene, world::World};

pub struct SceneBuilder {
    pub world: World,
}

impl SceneBuilder {
    pub fn new() -> Self {
        Self {
            world: World::default(),
        }
    }

    pub fn build(self) -> Scene {
        Scene::new(self.world, Box::new(crate::animation::Noop))
    }

    pub fn add_object(mut self, object: Object) -> Self {
        let id = rand::random::<usize>();
        self.world.objects.insert(id, object);
        self
    }
}
