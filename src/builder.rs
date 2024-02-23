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
        component.build(self)
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

    // Adds an object to the object tree, but does not add it to the root.
    fn register_object(&mut self, id: ObjectId, object: Object) {
        self.play(AddObject {
            object_id: id,
            object,
            rooted: false,
        });
    }

    fn register_new_object(&mut self, object: Object) -> ObjectId {
        let object_id = rand::random::<ObjectId>();
        self.play(AddObject {
            object_id,
            object,
            rooted: false,
        });
        object_id
    }

    // TODO: Potential bug: since we add the group at the end, the children
    // disconnect from the root, until after the group is added. This may
    // cause problems trying to get the bounding box of a child, for example.
    fn group(&mut self, group_builder: impl FnOnce(&mut GroupBuilder<Self>)) -> ObjectId {
        let group_id = rand::random::<ObjectId>();
        let mut group_build = GroupBuilder {
            builder: self,
            group_id,
            children: Vec::new(),
        };
        group_builder(&mut group_build);

        let children = group_build.children;

        self.add_object(group_id, Object::new_group(children));

        group_id
    }
}

pub struct GroupBuilder<'a, B: Builder> {
    builder: &'a mut B,
    group_id: ObjectId,
    children: Vec<ObjectId>,
}

impl<'a, B: Builder> Builder for GroupBuilder<'a, B> {
    fn state(&mut self) -> &mut BuilderState {
        self.builder.state()
    }
    fn play<A: Animation + 'static>(&mut self, animation: A) {
        self.builder.play(animation);
    }
    fn rooted(&mut self) -> bool {
        false
    }

    fn add_object(&mut self, id: ObjectId, object: Object) {
        self.children.push(id);
        self.play(AddObject {
            object_id: id,
            object,
            rooted: false,
        });
    }
}
