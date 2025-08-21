use egui::{pos2, Pos2};
use lyon::{
    math::point,
    path::{Path, Winding},
};

use crate::{
    builder::Builder,
    component::{Component, Handle},
    motion::Motion,
    object::{Material, Object, ObjectId, ObjectKind, Transform},
    world::World,
};

pub struct Circle {
    pub radius: f32,
    pub material: Material,
}

pub struct CircleHandle {
    radius: f32,
}

impl Component for Circle {
    type Handle = Handle<Object>;

    fn build<B: Builder>(self, builder: &mut B) -> Self::Handle {
        let mut path_builder = Path::builder();
        path_builder.add_circle(point(0.0, 0.0), self.radius, Winding::Positive);

        let object = Object::new_model(path_builder.build(), self.material);

        builder.add(object)
    }
}

pub struct Line {
    pub start: Pos2,
    pub end: Pos2,
    pub material: Material,
}

#[derive(Clone)]
pub struct LineHandle {
    pub line: Handle<Object>,
    pub start: Pos2,
    pub end: Pos2,
}

impl Line {
    pub fn build_path(start: Pos2, end: Pos2) -> Path {
        let mut path_builder = Path::builder();
        path_builder.begin(point(start.x, start.y));
        path_builder.line_to(point(end.x, end.y));
        path_builder.end(false);

        path_builder.build()
    }
}

impl Component for Line {
    type Handle = LineHandle;

    fn build<B: Builder>(self, builder: &mut B) -> Self::Handle {
        let path = Line::build_path(self.start, self.end);

        let object = Object::new_model(path, self.material);

        LineHandle {
            line: builder.add(object),
            start: self.start,
            end: self.end,
        }
    }
}

impl Handle<Line> {
    pub fn animate(&self, start: Option<Pos2>, end: Option<Pos2>) -> impl Motion {
        struct LineAnimation {
            object_id: Handle<Object>,
            from_start: Pos2,
            from_end: Pos2,
            to_start: Option<Pos2>,
            to_end: Option<Pos2>,
        }

        impl Motion for LineAnimation {
            fn animate(&self, world: &mut World, alpha: f32) {
                let to_start = self.to_start.unwrap_or(self.from_start);
                let to_end = self.to_end.unwrap_or(self.from_end);

                let animated_start = self.from_start + (to_start - self.from_start) * alpha;
                let animated_end = self.from_end + (to_end - self.from_end) * alpha;

                let path = Line::build_path(animated_start, animated_end);

                match &mut world.objects.get_mut(&self.object_id).unwrap().object_kind {
                    ObjectKind::Model(ref mut model) => {
                        model.update_path(path);
                    }
                    _ => unreachable!(),
                }
            }
        }

        LineAnimation {
            object_id: self.line.clone(),
            from_start: self.start,
            from_end: self.end,
            to_start: start,
            to_end: end,
        }
    }
}
