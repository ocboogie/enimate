use std::collections::HashMap;

use crate::{
    animation::Animation,
    builder::Builder,
    object::{Object, ObjectId},
    scene_builder::SceneBuilder,
    world::ObjectTree,
};

pub struct ComponentBuilder {
    pub rooted_objects: HashMap<ObjectId, Object>,
    pub objects: HashMap<ObjectId, Object>,
    pub animations: Vec<Box<dyn Animation>>,
}

impl ComponentBuilder {
    pub fn new() -> Self {
        Self {
            rooted_objects: HashMap::new(),
            objects: HashMap::new(),
            animations: Vec::new(),
        }
    }
}

impl Builder for ComponentBuilder {
    fn add_object(&mut self, id: ObjectId, object: Object, rooted: bool) {
        if rooted {
            self.rooted_objects.insert(id, object);
        } else {
            self.objects.insert(id, object);
        }
    }

    fn add_animation(&mut self, animation: Box<dyn Animation>) {
        self.animations.push(animation);
    }
}

pub trait Component {
    fn build(&self, builder: &mut impl Builder);
}
