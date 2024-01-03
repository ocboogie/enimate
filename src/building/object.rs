use crate::{
    motion::{Motion, MotionId},
    object::{Object, ObjectId, Transform},
};
use egui::Rect;

use super::{AnimationBuilder, Builder, BuilderState};

#[must_use]
pub struct ObjectBuilder<'a, B: Builder> {
    object_id: ObjectId,
    object: Object,
    builder: &'a mut B,
}

impl<'a, B: Builder> ObjectBuilder<'a, B> {
    pub fn new(object: Object, builder: &'a mut B) -> Self {
        let object_id = rand::random::<usize>();

        Self {
            object_id,
            object,
            builder,
        }
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.object.transform = transform;
        self
    }

    pub fn with_scale(mut self, scale: f32) -> Self {
        self.object.transform.scale = scale;
        self
    }

    pub fn with_rotation(mut self, rotation: f32) -> Self {
        self.object.transform.rotation = rotation;
        self
    }

    pub fn with_position(mut self, position: egui::Pos2) -> Self {
        self.object.transform.position = position;
        self
    }

    pub fn with_anchor(mut self, anchor: egui::Pos2) -> Self {
        self.object.transform.anchor = anchor;
        self
    }

    fn bounding_box(&mut self) -> Rect {
        self.builder
            .state()
            .objects
            .local_bounding_box_obj(&self.object)
    }

    pub fn with_centered_anchor(mut self) -> Self {
        let bounding_box = self.bounding_box();

        self.object.transform.position -= bounding_box.center().to_vec2();
        self.object.transform.anchor = bounding_box.center();
        self
    }

    pub fn add(self) -> ObjectId {
        self.builder.add_object(self.object_id, self.object);
        self.object_id
    }

    pub fn animate(
        self,
        duration: f32,
        animation_creator: impl FnOnce(AnimationBuilder) -> AnimationBuilder,
    ) -> ObjectId {
        self.builder.add_object(self.object_id, self.object);

        self.builder
            .animate(self.object_id, duration, animation_creator);

        self.object_id
        //
        // let _ = animation_creator(AnimationBuilder::new(self.object_id, self.builder.state()));
        //
        // self.object_id
    }
}

#[must_use]
pub struct GroupBuilder<'a, B: Builder> {
    pub builder: &'a mut B,
    pub children: Vec<ObjectId>,
}

impl<'a, B: Builder> GroupBuilder<'a, B> {
    pub fn new(builder: &'a mut B, _rooted: bool) -> Self {
        Self {
            builder,
            children: Vec::new(),
        }
    }
}

impl<'a, B: Builder> Builder for GroupBuilder<'a, B> {
    fn state(&mut self) -> &mut BuilderState {
        self.builder.state()
    }

    fn play(&mut self, motion: Box<dyn Motion>, duration: f32) -> MotionId {
        self.builder.play(motion, duration)
    }

    fn rooted(&mut self) -> bool {
        false
    }
}
