use crate::{
    animation::Animation,
    component::{Component, Handle},
    motion::AddObject,
    object::{Object, ObjectId},
};

pub trait Builder: Sized {
    fn play<A: Animation + 'static>(&mut self, animation: A);
    fn add_object(&mut self, object: Object) -> ObjectId;

    fn add<C: Component>(&mut self, component: C) -> Handle<C> {
        let mut component_builder = ComponentBuilder {
            builder: self,
            objects: Vec::new(),
        };

        let handle = component.build(&mut component_builder);

        let object = Object::new_group(component_builder.objects);
        let object_id = self.add_object(object);

        Handle {
            inner: handle,
            object_id,
        }
    }
}

pub struct ComponentBuilder<'a, B: Builder> {
    builder: &'a mut B,
    objects: Vec<ObjectId>,
}

impl<'a, B: Builder> Builder for ComponentBuilder<'a, B> {
    fn play<A: Animation + 'static>(&mut self, animation: A) {
        self.builder.play(animation);
    }

    fn add_object(&mut self, object: Object) -> ObjectId {
        let object_id = rand::random::<ObjectId>();
        self.objects.push(object_id);
        self.play(AddObject {
            object_id,
            object,
            rooted: false,
        });
        object_id
    }
}
