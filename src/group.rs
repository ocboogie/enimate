use crate::{
    builder::Builder,
    component::{Component, Handle},
    object::{Object, ObjectId, Transform},
};

pub struct Group<C: Component> {
    children: Vec<C>,
    transform: Transform,
}

impl<C: Component> Group<C> {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            transform: Transform::default(),
        }
    }

    pub fn from_children(children: Vec<C>) -> Self {
        Self {
            children,
            transform: Transform::default(),
        }
    }

    pub fn add(&mut self, child: C) -> &mut Self {
        self.children.push(child);
        self
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }
}

pub struct GroupHandle<C: Component> {
    pub object_id: ObjectId,
    pub children: Vec<C::Handle>,
}

impl<H: Handle + Clone, C: Component<Handle = H>> Clone for GroupHandle<C> {
    fn clone(&self) -> Self {
        Self {
            object_id: self.object_id,
            children: self.children.clone(),
        }
    }
}

impl<C: Component> Handle for GroupHandle<C> {
    fn id(&self) -> ObjectId {
        self.object_id
    }
}

impl<C: Component> Component for Group<C> {
    type Handle = GroupHandle<C>;

    fn build<B: Builder>(self, builder: &mut B) -> Self::Handle {
        let children: Vec<_> = self
            .children
            .into_iter()
            .map(|object| builder.add(object))
            .collect();

        let children_ids: Vec<_> = children.iter().map(|child| child.id()).collect();

        let object_id = builder.add_new_object(Object::new_group(children_ids));

        GroupHandle {
            object_id,
            children,
        }
    }
}
