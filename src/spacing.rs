use crate::{dynamics::DynamicType, object::ObjectId, renderer::UNIT_GRID_HEIGHT, world::World};
use egui::Pos2;

pub struct Pixels(f32);

impl DynamicType<f32> for Pixels {
    fn get(&self, world: &World) -> f32 {
        self.0 * (world.render_size().1 / UNIT_GRID_HEIGHT)
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
    target: ObjectId,
    horizontal: HorizontalAlignment,
    vertical: VerticalAlignment,
}

impl Alignment {
    pub fn new(target: ObjectId) -> Box<Self> {
        Box::new(Self {
            target,
            horizontal: HorizontalAlignment::Center,
            vertical: VerticalAlignment::Center,
        })
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

impl DynamicType<Pos2> for Alignment {
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
