use crate::{
    animation::Animation,
    builder::Builder,
    component::{Component, Handle},
    object::{Object, ObjectId},
};

#[derive(Default)]
pub struct Group {
    objects: Vec<Object>,
}

impl Group {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(mut self, object: Object) -> Self {
        self.objects.push(object);
        self
    }
}

pub struct GroupHandle {
    object_id: ObjectId,
    children: Vec<ObjectId>,
}

impl Handle for GroupHandle {
    fn id(&self) -> ObjectId {
        self.object_id
    }
}

impl Component for Group {
    type Handle = GroupHandle;

    fn build<B: Builder>(self, builder: &mut B) -> Self::Handle {
        let children: Vec<_> = self
            .objects
            .into_iter()
            .map(|object| builder.register_new_object(object))
            .collect();

        let object_id = builder.add_new_object(Object::new_group(children.clone()));

        GroupHandle {
            object_id,
            children,
        }
    }
}
