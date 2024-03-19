use std::ops::Deref;

use egui::Pos2;

use crate::builder::Builder;
use crate::dynamics::{Dynamic, DynamicType};
use crate::motion::MoveTo;
use crate::object::{Object, ObjectId};

pub struct Handle<T> {
    pub inner: T,
    pub object_id: ObjectId,
}

impl<T> Deref for Handle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: Clone> Clone for Handle<T> {
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

impl<T> Handle<T> {
    pub fn move_to(&self, pos: Dynamic<Pos2>) -> MoveTo {
        MoveTo {
            to: pos,
            object_id: self.object_id,
        }
    }
}
