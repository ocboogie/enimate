use std::ops::Deref;

use egui::Pos2;

use crate::builder::Builder;
use crate::dynamics::{Dynamic, DynamicTransform, OwnedDynamic};
use crate::motion::{FadeIn, Move, MoveTo};
use crate::object::{Object, ObjectId, Transform};

pub struct Handle<C: Component> {
    pub inner: C::Handle,
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
            object_id: self.object_id,
        }
    }
}

pub trait Component {
    type Handle;

    fn build<B: Builder>(self, builder: &mut B) -> Self::Handle;

    fn transform(&self) -> DynamicTransform {
        Transform::default().into()
    }
}

pub trait ComponentExt: Component + Sized {
    fn with_transform(
        self,
        transform: impl Into<DynamicTransform>,
    ) -> ComponentWithTransform<Self> {
        ComponentWithTransform {
            component: self,
            transform: transform.into(),
        }
    }
    fn with_position(self, position: Pos2) -> ComponentWithTransform<Self> {
        self.with_transform(Transform::default().with_position(position))
    }
    fn with_rotation(self, rotation: f32) -> ComponentWithTransform<Self> {
        self.with_transform(Transform::default().with_rotation(rotation))
    }
    fn with_scale(self, scale: f32) -> ComponentWithTransform<Self> {
        self.with_transform(Transform::default().with_scale(scale))
    }
    fn with_anchor(self, anchor: Pos2) -> ComponentWithTransform<Self> {
        self.with_transform(Transform::default().with_anchor(anchor))
    }
}

impl<C: Component> ComponentExt for C {}

pub struct ComponentWithTransform<C: Component> {
    component: C,
    transform: DynamicTransform,
}

impl<C: Component> Component for ComponentWithTransform<C> {
    type Handle = C::Handle;

    fn build<B: Builder>(self, builder: &mut B) -> Self::Handle {
        self.component.build(builder)
    }

    fn transform(&self) -> DynamicTransform {
        self.transform.clone()
    }
}

impl<C: Component> ComponentWithTransform<C> {
    pub fn with_transform(mut self, transform: impl Into<DynamicTransform>) -> Self {
        self.transform = transform.into();
        self
    }
    pub fn with_position(mut self, position: impl Dynamic<Pos2>) -> Self {
        self.transform.position = OwnedDynamic::new(position);
        self
    }
    pub fn with_rotation(mut self, rotation: impl Dynamic<f32>) -> Self {
        self.transform.rotation = OwnedDynamic::new(rotation);
        self
    }
    pub fn with_scale(mut self, scale: impl Dynamic<f32>) -> Self {
        self.transform.scale = OwnedDynamic::new(scale);
        self
    }
}

impl<C: Component> Handle<C> {
    pub fn move_to(&self, pos: impl Dynamic<Pos2>) -> MoveTo {
        MoveTo {
            to: OwnedDynamic::new(pos),
            object_id: self.object_id,
        }
    }

    pub fn mv(&self, from: impl Dynamic<Pos2>, to: impl Dynamic<Pos2>) -> Move {
        Move {
            from: OwnedDynamic::new(from),
            to: OwnedDynamic::new(to),
            object_id: self.object_id,
        }
    }

    pub fn fade_in(&self) -> FadeIn {
        FadeIn {
            object_id: self.object_id,
        }
    }
}

impl Component for Object {
    type Handle = ObjectId;

    fn build<B: Builder>(self, builder: &mut B) -> Self::Handle {
        builder.add_object(self.into())
    }
}
