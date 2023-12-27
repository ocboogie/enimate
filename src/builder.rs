use crate::{
    animation::{self, Animation},
    component::{Component, ComponentBuilder},
    mesh::{Mesh, Vertex},
    object::{Material, Object, ObjectId, ObjectKind, Transform},
};
use egui::pos2;
use lyon::{
    lyon_tessellation::{
        BuffersBuilder, FillGeometryBuilder, FillVertex, StrokeGeometryBuilder, StrokeOptions,
        StrokeTessellator, StrokeVertex,
    },
    path::Path,
    tessellation::{FillOptions, FillTessellator, VertexBuffers},
};

#[must_use]
pub struct ObjectAnimationBuilder<'a, B: Builder> {
    animations: Vec<Box<dyn Animation>>,
    object_id: ObjectId,
    object: Object,
    builder: &'a mut B,
}

impl<'a, B: Builder> ObjectAnimationBuilder<'a, B> {
    pub fn new(object: Object, object_id: ObjectId, builder: &'a mut B) -> Self {
        Self {
            animations: Vec::new(),
            object_id,
            object,
            builder,
        }
    }

    pub fn translate(mut self, end: egui::Pos2) -> Self {
        self.animations
            .push(Box::new(crate::animation::AnimateTransform::new(
                self.object_id,
                self.object.transform,
                self.object.transform.with_position(end),
            )));
        self
    }

    pub fn rotate(mut self, rotation: f32) -> Self {
        self.animations
            .push(Box::new(crate::animation::AnimateTransform::new(
                self.object_id,
                self.object.transform,
                self.object.transform.with_rotation(rotation),
            )));
        self
    }

    pub fn scale(mut self, scale: f32) -> Self {
        self.animations
            .push(Box::new(crate::animation::AnimateTransform::new(
                self.object_id,
                self.object.transform,
                self.object.transform.with_scale(scale),
            )));
        self
    }

    pub fn add(self) -> ObjectId {
        self.builder.add_object(self.object_id, self.object, true);
        self.builder
            .add_animation(Box::new(animation::Parallel::new(self.animations)));
        self.object_id
    }
}

#[must_use]
pub struct ObjectBuilder<'a, B: Builder> {
    object_id: ObjectId,
    object: Object,
    builder: &'a mut B,
}

impl<'a, B: Builder> ObjectBuilder<'a, B> {
    fn new(object: Object, builder: &'a mut B) -> Self {
        let object_id = rand::random::<usize>();

        Self {
            object_id,
            object,
            builder,
        }
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.object.transform = transform;
        self
    }

    pub fn with_scale(mut self, scale: f32) -> Self {
        self.object.transform.scale = scale;
        self
    }

    pub fn with_rotation(mut self, rotation: f32) -> Self {
        self.object.transform.rotation = rotation;
        self
    }

    pub fn with_position(mut self, position: egui::Pos2) -> Self {
        self.object.transform.position = position;
        self
    }

    pub fn with_anchor(mut self, anchor: egui::Pos2) -> Self {
        self.object.transform.anchor = anchor;
        self
    }

    pub fn with_animations(self) -> ObjectAnimationBuilder<'a, B> {
        ObjectAnimationBuilder::new(self.object, self.object_id, self.builder)
    }

    pub fn add(self) -> ObjectId {
        self.builder.add_object(self.object_id, self.object, true);
        self.object_id
    }

    pub fn center_anchor(mut self) -> Self {
        match self.object.object_kind {
            ObjectKind::Model(ref model) => {
                self.object.transform.position -= model.mesh.bounds().center().to_vec2();
                self.object.transform.anchor = model.mesh.bounds().center();
                self
            }
            ObjectKind::Group(_) => todo!(),
        }
    }
}

// Something that can be added to a Builder
pub trait Block {
    fn root(self, builder: &mut impl Builder) -> Object;
}

impl Block for Object {
    fn root(self, _builder: &mut impl Builder) -> Object {
        self
    }
}

impl<C: Component> Block for C {
    fn root(self, builder: &mut impl Builder) -> Object {
        let mut component_builder = ComponentBuilder::new();
        self.build(&mut component_builder);

        let ComponentBuilder {
            rooted_objects,
            objects,
            animations,
        } = component_builder;

        for animation in animations {
            builder.add_animation(animation);
        }
        let object_ids = objects
            .into_iter()
            .map(|(id, object)| (id, object, true))
            .chain(
                rooted_objects
                    .into_iter()
                    .map(|(id, object)| (id, object, false)),
            )
            .map(|(id, object, rooted)| {
                builder.add_object(id, object, rooted);
                id
            })
            .collect();

        Object::new_group(object_ids)
    }
}

pub trait Builder: Sized {
    fn add_object(&mut self, id: ObjectId, object: Object, rooted: bool);
    fn add_animation(&mut self, animation: Box<dyn Animation>);

    fn build(&mut self, block: impl Block) -> ObjectBuilder<Self> {
        ObjectBuilder::new(block.root(self), self)
    }

    fn add(&mut self, block: impl Block) -> ObjectId {
        self.build(block).add()
    }

    fn group(&mut self) -> GroupBuilder<Self> {
        GroupBuilder {
            builder: self,
            children: Vec::new(),
        }
    }

    fn tessellate<B: FnOnce(FillTessellator, &mut dyn FillGeometryBuilder)>(
        &mut self,
        builder: B,
    ) -> Mesh {
        let mut geometry: VertexBuffers<Vertex, u32> = VertexBuffers::new();
        let tessellator = FillTessellator::new();

        let mut buffers_builder = BuffersBuilder::new(&mut geometry, |vertex: FillVertex| {
            let pos = vertex.position();
            Vertex {
                pos: pos2(pos.x, pos.y),
            }
        });

        builder(tessellator, &mut buffers_builder);

        Mesh {
            vertices: geometry.vertices,
            indices: geometry.indices,
        }
    }

    fn tessellate_stroke<B: FnOnce(StrokeTessellator, &mut dyn StrokeGeometryBuilder)>(
        &mut self,
        builder: B,
    ) -> Mesh {
        let mut geometry: VertexBuffers<Vertex, u32> = VertexBuffers::new();
        let tessellator = StrokeTessellator::new();

        let mut buffers_builder = BuffersBuilder::new(&mut geometry, |vertex: StrokeVertex| {
            let pos = vertex.position();
            Vertex {
                pos: pos2(pos.x, pos.y),
            }
        });

        builder(tessellator, &mut buffers_builder);

        Mesh {
            vertices: geometry.vertices,
            indices: geometry.indices,
        }
    }

    fn circle(&mut self, x: f32, y: f32, radius: f32, material: Material) -> ObjectBuilder<Self> {
        let mesh = self.tessellate(|mut tessellator, buffers_builder| {
            tessellator
                .tessellate_circle(
                    lyon::math::point(0.0, 0.0),
                    radius,
                    &FillOptions::default(),
                    buffers_builder,
                )
                .unwrap();
        });

        let object = Object::new_model(mesh, material).with_transform(Transform {
            position: pos2(x, y),
            ..Default::default()
        });

        ObjectBuilder::new(object, self)
    }

    fn line(
        &mut self,
        start: egui::Pos2,
        end: egui::Pos2,
        width: f32,
        material: Material,
    ) -> ObjectBuilder<Self> {
        let mut builder = Path::builder();
        builder.begin(lyon::math::point(start.x, start.y));
        builder.line_to(lyon::math::point(end.x, end.y));
        builder.close();

        let path = builder.build();

        let mesh = self.tessellate_stroke(|mut tessellator, buffers_builder| {
            tessellator
                .tessellate_path(
                    &path,
                    &StrokeOptions::default().with_line_width(width),
                    buffers_builder,
                )
                .unwrap();
        });

        let object = Object::new_model(mesh, material);

        ObjectBuilder::new(object, self)
    }

    fn path(&mut self, path: Path, width: f32, material: Material) -> ObjectBuilder<Self> {
        let mesh = self.tessellate_stroke(|mut tessellator, buffers_builder| {
            tessellator
                .tessellate_path(
                    &path,
                    &StrokeOptions::default().with_line_width(width),
                    buffers_builder,
                )
                .unwrap();
        });

        let object = Object::new_model(mesh, material);

        ObjectBuilder::new(object, self)
    }
}

pub struct GroupBuilder<'a, B: Builder> {
    builder: &'a mut B,
    children: Vec<ObjectId>,
}

impl<'a, B: Builder> Builder for GroupBuilder<'a, B> {
    fn add_object(&mut self, id: ObjectId, object: Object, rooted: bool) {
        if rooted {
            self.children.push(id);
        }
        self.builder.add_object(id, object, false);
    }

    fn add_animation(&mut self, animation: Box<dyn Animation>) {
        self.builder.add_animation(animation);
    }
}

impl<'a, B: Builder> GroupBuilder<'a, B> {
    pub fn finish(self) -> ObjectBuilder<'a, B> {
        self.builder.build(Object::new_group(self.children))
    }
}
