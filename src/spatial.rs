use crate::{component::Handle, dynamics::WorldPos, object::ObjectId, world::World};
use egui::Pos2;

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

impl WorldPos for Alignment {
    fn get(&self, world: &World) -> Pos2 {
        let bb = world.objects.local_bounding_box(self.target);

        let x = match self.horizontal {
            HorizontalAlignment::Left => bb.left(),
            HorizontalAlignment::Center => bb.center().x,
            HorizontalAlignment::Right => bb.right(),
        };

        let y = match self.vertical {
            VerticalAlignment::Top => bb.top(),
            VerticalAlignment::Center => bb.center().y,
            VerticalAlignment::Bottom => bb.bottom(),
        };

        Pos2::new(x, y)
    }
}
