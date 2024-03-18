use egui::Pos2;

use crate::builder::Builder;
use crate::dynamics::Dynamic;
use crate::motion::{Alpha, Motion, Move};
use crate::object::{Object, ObjectId};
use crate::world::World;

pub trait Handle {
    fn id(&self) -> ObjectId;
}

impl Handle for ObjectId {
    fn id(&self) -> ObjectId {
        *self
    }
}

pub trait Component {
    type Handle: Handle;

    fn build<B: Builder>(self, builder: &mut B) -> Self::Handle;
}

impl Component for Object {
    type Handle = ObjectId;

    fn build<B: Builder>(self, builder: &mut B) -> Self::Handle {
        builder.add_new_object(self)
    }
}
