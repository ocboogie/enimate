use egui::Pos2;

use crate::builder::Builder;
use crate::dynamics::{Dynamic, DynamicType};
use crate::motion::MoveTo;
use crate::object::{Object, ObjectId};

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

pub trait HandleExt: Handle {
    fn move_to(&self, pos: Dynamic<Pos2>) -> MoveTo {
        MoveTo {
            to: pos,
            object_id: self.id(),
        }
    }
}

impl<T: Handle> HandleExt for T {}
