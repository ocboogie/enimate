use egui::{pos2, Pos2};
use lyon::{
    math::point,
    path::{Path, Winding},
};

use crate::{
    builder::Builder,
    component::Component,
    object::{Material, Object, ObjectId, Transform},
};

pub struct Circle {
    pub center: Pos2,
    pub radius: f32,
    pub material: Material,
}

impl Component for Circle {
    type Handle = ObjectId;

    fn add<B: Builder>(self, builder: &mut B) -> Self::Handle {
        let mut path_builder = Path::builder();
        path_builder.add_circle(point(0.0, 0.0), self.radius, Winding::Positive);

        let object = Object::new_model(path_builder.build(), self.material)
            .with_transform(Transform::default().with_position(self.center));

        builder.add_new_object(object)
    }
}

pub struct Line {
    pub start: Pos2,
    pub end: Pos2,
    pub material: Material,
}

impl Component for Line {
    type Handle = ObjectId;

    fn add<B: Builder>(self, builder: &mut B) -> Self::Handle {
        let mut path_builder = Path::builder();
        path_builder.begin(point(self.start.x, self.start.y));
        path_builder.line_to(point(self.end.x, self.end.y));

        let object = Object::new_model(path_builder.build(), self.material);

        builder.add_new_object(object)
    }
}
