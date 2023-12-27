use std::collections::HashMap;

use crate::{
    builder::{Builder, BuilderState},
    motion::Motion,
    object::{Object, ObjectId},
    scene_builder::SceneBuilder,
    world::ObjectTree,
};

pub trait Component {
    fn build(&self, builder: &mut impl Builder);
}
