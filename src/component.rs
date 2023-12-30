use std::collections::HashMap;

use crate::{
    builder::{Builder, BuilderState},
    motion::Motion,
    object::{Object, ObjectId},
    object_tree::ObjectTree,
    scene_builder::SceneBuilder,
};

pub trait Component {
    type Handle;
    fn build(&self, builder: &mut impl Builder) -> Self::Handle;
}
