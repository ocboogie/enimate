use egui::{Pos2, Rect};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crate::{
    mesh::Mesh,
    object::{Material, Model, Object, ObjectId, ObjectKind, Transform},
};

use crate::motion::{Motion, MotionId};

pub struct World<'a> {
    pub objects: &'a mut ObjectTree,
    motions: &'a HashMap<MotionId, Box<dyn Motion>>,
    pub time: f32,
}

impl<'a> World<'a> {
    pub fn new(
        time: f32,
        objects: &'a mut ObjectTree,
        motions: &'a HashMap<MotionId, Box<dyn Motion>>,
    ) -> Self {
        Self {
            objects,
            motions,
            time,
        }
    }

    pub fn play(&mut self, motion: MotionId) {
        // TODO: Provide a warning here somehow if the motion doesn't exist.
        let motion = self.motions.get(&motion).unwrap();
        motion.animate(self);
    }

    pub fn play_at(&mut self, motion: MotionId, time: f32) {
        let motion = self.motions.get(&motion).unwrap();
        let current_time = self.time;
        self.time = time;
        motion.animate(self);
        self.time = current_time;
    }
}

#[derive(Clone, Debug)]
pub struct ObjectTree {
    pub root: ObjectId,
    objects: HashMap<ObjectId, Object>,
}

impl Deref for ObjectTree {
    type Target = HashMap<ObjectId, Object>;

    fn deref(&self) -> &Self::Target {
        &self.objects
    }
}

impl DerefMut for ObjectTree {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.objects
    }
}

pub struct RenderObject {
    pub id: ObjectId,
    pub mesh: Mesh,
    pub material: Material,
    pub transform: Transform,
}

impl ObjectTree {
    pub fn new() -> Self {
        let mut objects = HashMap::new();

        let root = 0;
        objects.insert(
            root,
            Object {
                object_kind: ObjectKind::Group(Vec::new()),
                transform: Transform::default(),
            },
        );

        Self { root, objects }
    }

    pub fn render_object(
        &self,
        id: ObjectId,
        transform: Transform,
        objects: &mut Vec<RenderObject>,
    ) {
        let object = self.objects.get(&id).unwrap();
        let transform = transform.and_then(&object.transform);

        match &object.object_kind {
            ObjectKind::Model(model) => {
                objects.push(RenderObject {
                    id,
                    mesh: model.mesh.clone(),
                    material: model.material.clone(),
                    transform,
                });
            }
            ObjectKind::Group(group) => {
                for child_id in group {
                    self.render_object(*child_id, transform, objects);
                }
            }
        }
    }

    pub fn render(&self) -> Vec<RenderObject> {
        let mut objects = Vec::new();

        self.render_object(self.root, Transform::default(), &mut objects);
        objects
    }

    pub fn add(&mut self, id: usize, object: Object, rooted: bool) {
        if rooted {
            let root = self.objects.get_mut(&self.root).expect("No root");

            match &mut root.object_kind {
                ObjectKind::Group(group) => {
                    group.push(id);
                }
                _ => panic!("Root object is not a group"),
            }
        }
        self.objects.insert(id, object);
    }

    // pub fn get(&self, id: ObjectId) -> Option<&Object> {
    //     self.objects.get(&id)
    // }
    //
    // pub fn get_mut(&mut self, id: ObjectId) -> Option<&mut Object> {
    //     self.objects.get_mut(&id)
    // }
    //
    pub fn bounds(&self, object_id: &ObjectId) -> Rect {
        let object = self.objects.get(object_id).unwrap();

        match &object.object_kind {
            ObjectKind::Model(model) => model.mesh.bounds(),
            ObjectKind::Group(group) => {
                let mut bounds = Rect::NOTHING;

                for child_id in group {
                    let child_bounds = self.bounds(child_id);
                    bounds = bounds.union(child_bounds);
                }

                bounds
            }
        }
    }
}
