use std::collections::HashMap;

use crate::object::Object;

type ObjectID = usize;

#[derive(Default)]
pub struct World {
    objects: HashMap<ObjectID, Object>,
}

impl World {
    // pub fn new(objects: Vec<Object>) -> Self {
    //     Self { objects }
    // }

    pub fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // for object in &self.objects.values() {
        //
        // }
    }
}
