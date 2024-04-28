use crate::{
    animation::Animation,
    builder::Builder,
    dynamics::{DynamicObject, OwnedDynamic},
    motion::{AddObject, Alpha, Motion},
    object::{Object, ObjectId},
    object_tree::ObjectTree,
    timing::{Sequence, Time},
    world::{Variable, World},
};
use std::collections::HashMap;

pub struct Scene(pub Sequence);

impl Scene {
    pub fn null() -> Self {
        Self(Sequence(vec![]))
    }

    pub fn length(&self) -> Time {
        self.0.duration()
    }

    pub fn time_to_alpha(&self, time: Time) -> Alpha {
        time / self.length()
    }

    pub fn render_at(&self, time: Time, render_size: (f32, f32)) -> ObjectTree {
        let mut world = World::new(ObjectTree::new(), render_size, HashMap::new());

        self.0.animate(&mut world, self.time_to_alpha(time));

        world.objects
    }

    pub fn render_with_input(
        &mut self,
        time: f32,
        render_size: (f32, f32),
        input: HashMap<Variable, f32>,
    ) -> ObjectTree {
        let mut world = World::new(ObjectTree::new(), render_size, input);

        self.0.animate(&mut world, self.time_to_alpha(time));

        world.objects
    }
}

pub struct SceneBuilder {
    scene: Scene,
}

impl SceneBuilder {
    pub fn new() -> Self {
        Self {
            scene: Scene::null(),
        }
    }

    pub fn finish(self) -> Scene {
        self.scene
    }
}

impl Builder for SceneBuilder {
    fn play<A: Animation + 'static>(&mut self, animation: A) {
        (self.scene.0).0.push(Box::new(animation));
    }

    fn add_object(&mut self, object: DynamicObject) -> ObjectId {
        let object_id = rand::random::<ObjectId>();

        self.play(AddObject {
            object_id,
            object: OwnedDynamic::new(object),
            rooted: true,
        });

        object_id
    }
}
