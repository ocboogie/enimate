use egui::{pos2, Pos2};

use crate::{motion::Alpha, object::Transform};

pub trait Interpolatable: 'static {
    fn interpolate(&self, other: &Self, alpha: Alpha) -> Self;
}

impl Interpolatable for f32 {
    fn interpolate(&self, other: &Self, alpha: Alpha) -> Self {
        self * (1.0 - alpha) + other * alpha
    }
}

impl Interpolatable for Pos2 {
    fn interpolate(&self, other: &Self, alpha: Alpha) -> Self {
        pos2(
            self.x.interpolate(&other.x, alpha),
            self.y.interpolate(&other.y, alpha),
        )
    }
}

impl Interpolatable for Transform {
    fn interpolate(&self, other: &Self, alpha: Alpha) -> Self {
        Transform {
            position: self.position.interpolate(&other.position, alpha),
            rotation: self.rotation.interpolate(&other.rotation, alpha),
            scale: self.scale.interpolate(&other.scale, alpha),
            anchor: self.anchor.interpolate(&other.anchor, alpha),
        }
    }
}
