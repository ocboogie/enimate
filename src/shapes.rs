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
    trigger::Trigger,
    world::World,
};

pub struct Circle {
    pub center: Pos2,
    pub radius: f32,
    pub material: Material,
}

impl Component for Circle {
    type Handle = ObjectId;

    fn build<B: Builder>(self, builder: &mut B) -> Self::Handle {
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

pub struct LineHandle {
    object_id: ObjectId,
    starting_start: Pos2,
    starting_end: Pos2,
}

impl Component for Line {
    type Handle = LineHandle;

    fn build<B: Builder>(self, builder: &mut B) -> Self::Handle {
        let mut path_builder = Path::builder();
        path_builder.begin(point(self.start.x, self.start.y));
        path_builder.line_to(point(self.end.x, self.end.y));
        path_builder.end(false);

        let object = Object::new_model(path_builder.build(), self.material);

        let object_id = builder.add_new_object(object);

        LineHandle {
            object_id,
            starting_start: self.start,
            starting_end: self.end,
        }
    }
}

impl Handle for LineHandle {
    fn id(&self) -> ObjectId {
        self.object_id
    }
}

impl LineHandle {
    pub fn change(&self, start: Pos2, end: Pos2) -> impl Trigger {
        struct LineChange {
            object_id: ObjectId,
            start: Pos2,
            end: Pos2,
        }

        impl Trigger for LineChange {
            fn trigger(&self, world: &mut World) {
                match &mut world.objects.get_mut(&self.object_id).unwrap().object_kind {
                    ObjectKind::Model(ref mut model) => {
                        let mut path_builder = Path::builder();
                        path_builder.begin(point(self.start.x, self.start.y));
                        path_builder.line_to(point(self.end.x, self.end.y));
                        path_builder.end(false);

                        model.update_path(path_builder.build());
                    }
                    _ => unreachable!(),
                }
            }
        }

        LineChange {
            object_id: self.object_id,
            start,
            end,
        }
    }

    pub fn animate(&self, start: Pos2, end: Pos2) -> impl Motion {
        struct LineAnimation {
            object_id: ObjectId,
            from_start: Pos2,
            from_end: Pos2,
            to_start: Pos2,
            to_end: Pos2,
        }

        impl Motion for LineAnimation {
            fn animate(&self, world: &mut World, alpha: f32) {
                let mut path_builder = Path::builder();
                let new_start = self.from_start + (self.to_start - self.from_start) * alpha;
                path_builder.begin(point(new_start.x, new_start.y));

                let new_end = self.from_end + (self.to_end - self.from_end) * alpha;
                path_builder.line_to(point(new_end.x, new_end.y));

                path_builder.end(false);

                let path = path_builder.build();

                match &mut world.objects.get_mut(&self.object_id).unwrap().object_kind {
                    ObjectKind::Model(ref mut model) => {
                        model.update_path(path);
                    }
                    _ => unreachable!(),
                }
            }
        }

        LineAnimation {
            object_id: self.object_id,
            from_start: self.starting_start,
            from_end: self.starting_end,
            to_start: start,
            to_end: end,
        }
    }
}
