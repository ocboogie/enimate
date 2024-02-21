use crate::{
    animation::Animation,
    component::Component,
    motion::{AddObject, Motion},
    object::{Object, ObjectId},
    object_tree::ObjectTree,
    scene::Scene,
    world::World,
};
use std::collections::HashMap;

pub struct BuilderState {
    pub scene: Scene,
    pub objects: ObjectTree,
}

impl BuilderState {
    pub fn new() -> Self {
        Self {
            scene: Scene::null(),
            objects: ObjectTree::new(),
        }
    }

    pub fn emulate_motion(&mut self, motion: &dyn Motion) {
        // Run the motion, so the state of objects is consistent with the end of the motion.
        let world = &mut World::new(&mut self.objects, HashMap::new());
        motion.animate(world, 1.0);
    }
}

pub trait Builder: Sized {
    fn state(&mut self) -> &mut BuilderState;
    fn play<A: Animation + 'static>(&mut self, animation: A);
    // Wheather new objects should be added to the root of the object tree.
    fn rooted(&mut self) -> bool {
        true
    }

    fn add<C: Component>(&mut self, component: C) -> C::Handle {
        component.add(self)
    }

    fn add_object(&mut self, id: ObjectId, object: Object) {
        let rooted = self.rooted();
        self.play(AddObject {
            object_id: id,
            object,
            rooted,
        });
    }

    // Same as add_object, but generates a random id.
    fn add_new_object(&mut self, object: Object) -> ObjectId {
        let object_id = rand::random::<ObjectId>();
        self.add_object(object_id, object);
        object_id
    }
}
