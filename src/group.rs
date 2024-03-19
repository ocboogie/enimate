use std::ops::{Deref, DerefMut};

use crate::{
    builder::Builder,
    component::{Component, Handle},
    object::Transform,
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
    pub children: Vec<Handle<C>>,
}

impl<C: Component> Deref for GroupHandle<C> {
    type Target = [Handle<C>];

    fn deref(&self) -> &Self::Target {
        &self.children
    }
}

impl<C: Component> DerefMut for GroupHandle<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.children
    }
}

impl<H: Clone, C: Component<Handle = H>> Clone for GroupHandle<C> {
    fn clone(&self) -> Self {
        Self {
            children: self.children.clone(),
        }
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

        GroupHandle { children }
    }
}
