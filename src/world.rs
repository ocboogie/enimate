use egui::Pos2;
use std::collections::HashMap;

use crate::{
    mesh::Mesh,
    object::{Material, Model, Object, ObjectId, ObjectKind, Transform},
};

#[derive(Clone, Debug)]
pub struct ObjectTree {
    pub root: ObjectId,
    objects: HashMap<ObjectId, Object>,
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

        let root = rand::random::<usize>();
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

    pub fn add_object(&mut self, id: usize, object: Object, rooted: bool) {
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

    pub fn get_object(&self, id: ObjectId) -> Option<&Object> {
        self.objects.get(&id)
    }

    pub fn get_object_mut(&mut self, id: ObjectId) -> Option<&mut Object> {
        self.objects.get_mut(&id)
    }
}
