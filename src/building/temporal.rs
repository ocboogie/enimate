use crate::{
    motion::{AnimateTransform, FadeIn, Motion, MotionId},
    object::{Object, ObjectId},
};

use super::{Builder, BuilderState, Positioner};

#[must_use]
pub struct AnimationBuilder<'a> {
    object_id: ObjectId,
    pub animations: Vec<Box<dyn Motion>>,
    state: &'a BuilderState,
}

impl<'a> AnimationBuilder<'a> {
    pub fn new(object_id: ObjectId, state: &'a BuilderState) -> Self {
        Self {
            object_id,
            animations: Vec::new(),
            state,
        }
    }

    fn object(&mut self) -> &Object {
        self.state.objects.get(&self.object_id).unwrap()
    }

    pub fn translate(mut self, end: egui::Pos2) -> Self {
        let a = AnimateTransform {
            object_id: self.object_id,
            from: self.object().transform.into(),
            to: self.object().transform.with_position(end).into(),
        };
        self.animations.push(Box::new(a));
        self
    }

    // pub fn delay(self, duration: f32) -> Self {
    //     self.builder.play_for(Box::new(NoOp), duration);
    //     self
    // }

    pub fn move_to(mut self, pos: impl Positioner) -> Self {
        let target_pos = pos.position(self.object_id, self.state);
        let a = AnimateTransform {
            object_id: self.object_id,
            from: self.object().transform.into(),
            to: self.object().transform.with_position(target_pos).into(),
        };
        self.animations.push(Box::new(a));

        self
    }

    pub fn rotate(mut self, rotation: f32) -> Self {
        let a = AnimateTransform {
            object_id: self.object_id,
            from: self.object().transform.into(),
            to: self.object().transform.with_rotation(rotation).into(),
        };
        self.animations.push(Box::new(a));
        self
    }

    pub fn scale(mut self, scale: f32) -> Self {
        let a = AnimateTransform {
            object_id: self.object_id,
            from: self.object().transform.into(),
            to: self.object().transform.with_scale(scale).into(),
        };
        self.animations.push(Box::new(a));
        self
    }

    pub fn fade_in(mut self) -> Self {
        self.animations.push(Box::new(FadeIn {
            object_id: self.object_id,
        }));
        self
    }
}
