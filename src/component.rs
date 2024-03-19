use std::ops::Deref;

use egui::Pos2;

use crate::builder::Builder;
use crate::dynamics::{Dynamic, DynamicType};
use crate::motion::MoveTo;
use crate::object::{Object, ObjectId};

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
}

impl Component for Object {
    type Handle = ObjectId;

    fn build<B: Builder>(self, builder: &mut B) -> Self::Handle {
        builder.add_object(self)
    }
}

impl<C: Component> Handle<C> {
    pub fn move_to(&self, pos: Dynamic<Pos2>) -> MoveTo {
        MoveTo {
            to: pos,
            object_id: self.object_id,
        }
    }
}
