use std::ops::Deref;

use egui::Pos2;

use crate::builder::Builder;
use crate::dynamics::Dynamic;
use crate::motion::MoveTo;
use crate::object::{Object, ObjectId, Transform};
use crate::properties::TransformProperty;

pub struct Handle<C: Component> {
    pub inner: C::Handle,
    pub transform: TransformProperty,
    pub object_id: ObjectId,
}

impl<C: Component> Deref for Handle<C> {
    type Target = C::Handle;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<H: Clone, C: Component<Handle = H>> Clone for Handle<C> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            transform: self.transform.clone(),
            object_id: self.object_id,
        }
    }
}

pub trait Component {
    type Handle;

    fn build<B: Builder>(self, builder: &mut B) -> Self::Handle;

    fn transform(&self) -> Transform {
        Transform::default()
    }
}

pub trait ComponentExt: Component + Sized {
    fn with_transform(self, transform: Transform) -> ComponentWrapper<Self> {
        ComponentWrapper {
            component: self,
            transform,
        }
    }
    fn with_position(self, position: Pos2) -> ComponentWrapper<Self> {
        self.with_transform(Transform::default().with_position(position))
    }
    fn with_rotation(self, rotation: f32) -> ComponentWrapper<Self> {
        self.with_transform(Transform::default().with_rotation(rotation))
    }
    fn with_scale(self, scale: f32) -> ComponentWrapper<Self> {
        self.with_transform(Transform::default().with_scale(scale))
    }
    fn with_anchor(self, anchor: Pos2) -> ComponentWrapper<Self> {
        self.with_transform(Transform::default().with_anchor(anchor))
    }
}

impl<C: Component> ComponentExt for C {}

pub struct ComponentWrapper<C: Component> {
    component: C,
    transform: Transform,
}

impl<C: Component> Component for ComponentWrapper<C> {
    type Handle = C::Handle;

    fn build<B: Builder>(self, builder: &mut B) -> Self::Handle {
        self.component.build(builder)
    }

    fn transform(&self) -> Transform {
        self.transform
    }
}

impl<C: Component> ComponentWrapper<C> {
    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }
    pub fn with_position(mut self, position: Pos2) -> Self {
        self.transform.position = position;
        self
    }
    pub fn with_rotation(mut self, rotation: f32) -> Self {
        self.transform.rotation = rotation;
        self
    }
    pub fn with_scale(mut self, scale: f32) -> Self {
        self.transform.scale = scale;
        self
    }
}

pub struct ObjectHandler {
    transform: TransformProperty,
}

impl<C: Component> Handle<C> {
    pub fn move_to(&self, pos: Dynamic<Pos2>) -> MoveTo {
        MoveTo {
            to: pos,
            object_id: self.object_id,
        }
    }
}

impl Component for Object {
    type Handle = ObjectId;

    fn build<B: Builder>(self, builder: &mut B) -> Self::Handle {
        builder.add_object(self)
    }
}
