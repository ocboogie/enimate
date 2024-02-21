use crate::{
    mesh::{Mesh, Vertex},
    object::{Object, ObjectId, ObjectKind, Transform},
    utils::box2d_to_rect,
};
use egui::{pos2, Color32, Rect};
use lyon::{
    algorithms::aabb::bounding_box,
    lyon_tessellation::{
        BuffersBuilder, FillOptions, FillTessellator, FillVertex, StrokeOptions, StrokeTessellator,
        StrokeVertex, VertexBuffers,
    },
    path::Path,
};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

#[derive(Clone, Debug)]
pub struct ObjectTree {
    pub root: ObjectId,
    objects: HashMap<ObjectId, Object>,
    parent_map: HashMap<ObjectId, ObjectId>,
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

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum RenderObjectKind {
    Stroke,
    Fill,
}

pub struct RenderObject {
    pub id: ObjectId,
    pub mesh: Mesh,
    pub color: Color32,
    pub transform: Transform,
    pub kind: RenderObjectKind,
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

        Self {
            root,
            objects,
            parent_map: HashMap::new(),
        }
    }

    fn tessellate_fill(tessellator: &mut FillTessellator, path: &Path) -> Mesh {
        let mut geometry: VertexBuffers<Vertex, u32> = VertexBuffers::new();

        let mut buffers_builder = BuffersBuilder::new(&mut geometry, |vertex: FillVertex| {
            let pos = vertex.position();
            Vertex {
                pos: pos2(pos.x, pos.y),
            }
        });

        tessellator
            .tessellate_path(path, &FillOptions::default(), &mut buffers_builder)
            .unwrap();

        Mesh {
            vertices: geometry.vertices,
            indices: geometry.indices,
        }
    }

    fn tessellate_stroke(
        tessellator: &mut StrokeTessellator,
        line_width: f32,
        path: &Path,
    ) -> Mesh {
        let mut geometry: VertexBuffers<Vertex, u32> = VertexBuffers::new();

        let mut buffers_builder = BuffersBuilder::new(&mut geometry, |vertex: StrokeVertex| {
            let pos = vertex.position();
            Vertex {
                pos: pos2(pos.x, pos.y),
            }
        });

        tessellator
            .tessellate_path(
                path,
                &StrokeOptions::default().with_line_width(line_width),
                &mut buffers_builder,
            )
            .unwrap();

        Mesh {
            vertices: geometry.vertices,
            indices: geometry.indices,
        }
    }

    pub fn render_object(
        &self,
        id: ObjectId,
        transform: Transform,
        fill_tessellator: &mut FillTessellator,
        stroke_tessellator: &mut StrokeTessellator,
        objects: &mut Vec<RenderObject>,
    ) {
        let object = self.objects.get(&id).unwrap();
        let transform = transform.and_then(&object.transform);

        match &object.object_kind {
            ObjectKind::Model(model) => {
                if model.material.fill.is_none() && model.material.stroke.is_none() {
                    panic!("Tried to render object with no fill or stroke");
                }

                if let Some(fill) = &model.material.fill {
                    objects.push(RenderObject {
                        id,
                        mesh: Self::tessellate_fill(fill_tessellator, &model.path),
                        color: fill.color,
                        transform,
                        kind: RenderObjectKind::Fill,
                    });
                }

                if let Some(stroke) = &model.material.stroke {
                    objects.push(RenderObject {
                        id,
                        mesh: Self::tessellate_stroke(
                            stroke_tessellator,
                            stroke.width,
                            &model.path,
                        ),
                        color: stroke.color,
                        transform,
                        kind: RenderObjectKind::Stroke,
                    });
                }
            }
            ObjectKind::Group(group) => {
                for child_id in group {
                    self.render_object(
                        *child_id,
                        transform,
                        fill_tessellator,
                        stroke_tessellator,
                        objects,
                    );
                }
            }
        }
    }

    pub fn render(&self) -> Vec<RenderObject> {
        let mut objects = Vec::new();
        let mut fill_tessellator = FillTessellator::new();
        let mut stroke_tessellator = StrokeTessellator::new();

        self.render_object(
            self.root,
            Transform::default(),
            &mut fill_tessellator,
            &mut stroke_tessellator,
            &mut objects,
        );
        objects
    }

    pub fn add(&mut self, id: usize, object: Object, rooted: bool) {
        if let ObjectKind::Group(children) = &object.object_kind {
            for child_id in children {
                self.parent_map.insert(*child_id, id);
            }
        }

        if rooted {
            let root = self.objects.get_mut(&self.root).expect("No root");

            match &mut root.object_kind {
                ObjectKind::Group(group) => {
                    self.parent_map.insert(id, self.root);
                    group.push(id);
                }
                _ => panic!("Root object is not a group"),
            }
        }

        self.objects.insert(id, object);
    }

    pub fn merge(&mut self, mut other: ObjectTree, root_id: ObjectId) -> Vec<ObjectId> {
        let mut rooted = Vec::new();
        self.objects.extend(
            other
                .objects
                .into_iter()
                .map(|(id, object)| (if id == other.root { root_id } else { id }, object)),
        );
        self.parent_map
            .extend(other.parent_map.into_iter().map(|(child, parent)| {
                (
                    child,
                    if parent == other.root {
                        rooted.push(child);
                        root_id
                    } else {
                        parent
                    },
                )
            }));

        rooted
    }

    fn flattened_transform(&self, id: ObjectId) -> Transform {
        let mut curr_id = id;
        let mut transforms = Vec::new();

        while let Some(parent_id) = self.parent_map.get(&curr_id) {
            let parent = self.objects.get(parent_id).unwrap();
            transforms.push(parent.transform);
            curr_id = *parent_id;
        }

        transforms.reverse();

        transforms
            .iter()
            .fold(Transform::default(), |acc, transform| {
                acc.and_then(transform)
            })
    }

    fn bounding_box_with_transform(&self, object: &Object, transform: Transform) -> Rect {
        let transform = transform.and_then(&object.transform);

        match &object.object_kind {
            ObjectKind::Model(model) => {
                transform.map_aabb(box2d_to_rect(bounding_box(&model.path)))
            }
            ObjectKind::Group(group) => {
                let mut bounding_box = Rect::NOTHING;

                for child_id in group {
                    let child = self.objects.get(child_id).unwrap();
                    let child_bounding_box = self.bounding_box_with_transform(child, transform);

                    bounding_box = bounding_box.union(child_bounding_box);
                }

                bounding_box
            }
        }
    }

    // pub fn bounding_box(&self, id: ObjectId) -> Box2D {
    //     let transform = self.flattened_transform(id);
    //     let object = self.objects.get(&id).unwrap();
    //     self.bounding_box_with_transform(object, transform)
    // }

    /// This is the bounding box of the object in its local coordinate system.
    /// i.e., the bounding box without any of its parents' transforms applied.
    pub fn local_bounding_box(&self, id: ObjectId) -> Rect {
        let object = self.objects.get(&id).unwrap();
        self.local_bounding_box_obj(object)
    }

    pub fn local_bounding_box_obj(&self, object: &Object) -> Rect {
        self.bounding_box_with_transform(object, Transform::default())
    }

    pub fn bounding_boxes_dp(
        &self,
        id: ObjectId,
        transform: Transform,
        boxes: &mut HashMap<ObjectId, Rect>,
    ) -> Rect {
        let object = self.objects.get(&id).unwrap();
        let transform = transform.and_then(&object.transform);

        let bb = match &object.object_kind {
            ObjectKind::Model(model) => {
                transform.map_aabb(box2d_to_rect(bounding_box(&model.path)))
            }
            ObjectKind::Group(group) => {
                let mut bounding_box = Rect::NOTHING;

                for child in group {
                    let child_bounding_box = self.bounding_boxes_dp(*child, transform, boxes);
                    bounding_box = bounding_box.union(child_bounding_box);
                }

                bounding_box
            }
        };

        boxes.insert(id, bb);
        bb
    }

    pub fn bounding_boxes(&self) -> HashMap<ObjectId, Rect> {
        let mut boxes = HashMap::new();
        self.bounding_boxes_dp(self.root, Transform::default(), &mut boxes);
        boxes
    }
}
