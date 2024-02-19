use crate::object::ObjectId;

use super::Builder;

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

    fn add<B: Builder>(self, builder: &mut B) -> Self::Handle;
}
