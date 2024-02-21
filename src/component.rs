use crate::builder::Builder;
use crate::object::ObjectId;

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
