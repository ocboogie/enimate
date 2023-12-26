use std::collections::HashMap;

use crate::object::Object;

pub type ObjectId = usize;

#[derive(Default, Clone)]
pub struct World {
    pub objects: HashMap<ObjectId, Object>,
}

impl World {}
