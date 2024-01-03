use crate::object::ObjectId;
use egui::{vec2, Pos2};

use super::BuilderState;

pub trait Positioner {
    fn position(&self, source: ObjectId, state: &BuilderState) -> Pos2;
}

impl Positioner for Pos2 {
    fn position(&self, _source: ObjectId, _state: &BuilderState) -> Pos2 {
        *self
    }
}

pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}

pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
}

pub struct Alignment {
    pub target: ObjectId,
    pub horizontal: HorizontalAlignment,
    pub vertical: VerticalAlignment,
}

impl Alignment {
    pub fn new(target: ObjectId) -> Self {
        Self {
            target,
            horizontal: HorizontalAlignment::Center,
            vertical: VerticalAlignment::Center,
        }
    }

    pub fn left(mut self) -> Self {
        self.horizontal = HorizontalAlignment::Left;
        self
    }

    pub fn center(mut self) -> Self {
        self.horizontal = HorizontalAlignment::Center;
        self
    }

    pub fn right(mut self) -> Self {
        self.horizontal = HorizontalAlignment::Right;
        self
    }

    pub fn top(mut self) -> Self {
        self.vertical = VerticalAlignment::Top;
        self
    }

    pub fn bottom(mut self) -> Self {
        self.vertical = VerticalAlignment::Bottom;
        self
    }
}

impl Positioner for Alignment {
    fn position(&self, source: ObjectId, state: &BuilderState) -> Pos2 {
        let source_bb = state.objects.local_bounding_box(source);
        let target_bb = state.objects.local_bounding_box(self.target);

        let x = match self.horizontal {
            HorizontalAlignment::Left => target_bb.left(),
            HorizontalAlignment::Center => target_bb.center().x,
            HorizontalAlignment::Right => target_bb.right(),
        };

        let y = match self.vertical {
            VerticalAlignment::Top => target_bb.top(),
            VerticalAlignment::Center => target_bb.center().y,
            VerticalAlignment::Bottom => target_bb.bottom(),
        };

        Pos2::new(x, y) - source_bb.center().to_vec2()
    }
}
