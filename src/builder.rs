use std::collections::HashMap;

use crate::{
    component::Component,
    // component::{Component, ComponentBuilder},
    mesh::{Mesh, Vertex},
    motion::{
        self, AddObject, AnimateTransform, FadeIn, Keyframe, Motion, MotionId, Parallel, Trigger,
    },
    object::{Material, Object, ObjectId, ObjectKind, Transform},
    scene_builder::SceneBuilder,
    world::{ObjectTree, World},
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

pub struct BuilderState {
    pub motions: HashMap<MotionId, Box<dyn Motion>>,
    pub root_motions: Vec<MotionId>,
    pub objects: ObjectTree,
    pub current_time: f32,
    pub scene_length: f32,
}

impl BuilderState {
    pub fn new(scene_length: f32) -> Self {
        Self {
            motions: HashMap::new(),
            root_motions: Vec::new(),
            objects: ObjectTree::new(),
            current_time: 0.0,
            scene_length,
        }
    }
}

#[must_use]
pub struct AnimationBuilder<'a, B: Builder> {
    object_id: ObjectId,
    duration: f32,
    builder: &'a mut B,
}

impl<'a, B: Builder> AnimationBuilder<'a, B> {
    fn new(object_id: ObjectId, duration: f32, builder: &'a mut B) -> Self {
        Self {
            object_id,
            duration,
            builder,
        }
    }

    fn object(&mut self) -> &Object {
        self.builder.state().objects.get(&self.object_id).unwrap()
    }

    fn play(self, motion: impl Motion + 'static) -> Self {
        self.builder.play_for(Box::new(motion), self.duration);
        self
    }

    pub fn translate(mut self, end: egui::Pos2) -> Self {
        let a = AnimateTransform::new(
            self.object_id,
            self.object().transform,
            self.object().transform.with_position(end),
        );
        self.play(a)
    }

    pub fn rotate(mut self, rotation: f32) -> Self {
        let a = AnimateTransform::new(
            self.object_id,
            self.object().transform,
            self.object().transform.with_rotation(rotation),
        );
        self.play(a)
    }

    pub fn scale(mut self, scale: f32) -> Self {
        let a = AnimateTransform::new(
            self.object_id,
            self.object().transform,
            self.object().transform.with_scale(scale),
        );
        self.play(a)
    }

    pub fn fade_in(self) -> Self {
        let a = FadeIn {
            object_id: self.object_id,
        };
        self.play(a)
    }

    // pub fn add(self) -> ObjectId {
    //     self.builder.add_object(self.object_id, self.object, true);
    //     // self.builder
    //     //     .add_animation(Box::new(motion::Parallel::new(self.animations)));
    //     self.object_id
    // }
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

    // pub fn with_animations(mut self, animation_creator: impl FnOnce(AnimationBuilder)) -> Self {
    //     animation_creator(AnimationBuilder::new(
    //         self.object_id,
    //         self.builder.state().scene_length,
    //         self.builder,
    //     ));
    // }

    pub fn add(self) -> ObjectId {
        self.builder.add_object(self.object_id, self.object, true);
        self.object_id
    }

    pub fn center_anchor(mut self) -> Self {
        let bounds = match self.object.object_kind {
            ObjectKind::Model(ref model) => model.mesh.bounds(),
            ObjectKind::Group(_) => self.builder.state().objects.bounds(&self.object_id),
        };

        self.object.transform.position -= bounds.center().to_vec2();
        self.object.transform.anchor = bounds.center();
        self
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
        let mut group_builder = GroupBuilder {
            builder,
            children: Vec::new(),
        };

        self.build(&mut group_builder);

        Object::new_group(group_builder.children)
    }
}

pub trait Builder: Sized {
    fn state(&mut self) -> &mut BuilderState;

    fn add_motion(&mut self, motion: Box<dyn Motion>) -> MotionId {
        let motion_id = rand::random::<usize>();

        self.state().motions.insert(motion_id, motion);
        self.state().root_motions.push(motion_id);

        motion_id
    }

    fn regester_motion(&mut self, motion: Box<dyn Motion>) -> MotionId {
        let motion_id = rand::random::<usize>();

        self.state().motions.insert(motion_id, motion);
        motion_id
    }

    fn emulate_motion(&mut self, motion: &dyn Motion) {
        let state = self.state();
        // Run the motion, so the state of objects is consistent with the end of the motion.
        let world = &mut World::new(1.0, &mut state.objects, &state.motions);
        motion.animate(world);
    }

    fn play(&mut self, motion: Box<dyn Motion>) -> MotionId {
        self.emulate_motion(motion.as_ref());
        let motion_id = self.regester_motion(motion);

        let trigger = Trigger::new(self.state().current_time, motion_id);
        self.add_motion(Box::new(trigger))
    }

    fn play_for(&mut self, motion: Box<dyn Motion>, duration: f32) -> MotionId {
        self.emulate_motion(motion.as_ref());
        let motion_id = self.regester_motion(motion);

        let current_time = &mut self.state().current_time;
        *current_time += duration;

        let keyframe = Keyframe::new(0.0, 1.0, *current_time, *current_time + duration, motion_id);
        self.add_motion(Box::new(keyframe))
    }

    fn play_concurrently_for(&mut self, motions: Vec<Box<dyn Motion>>, duration: f32) -> MotionId {
        let parallel = Parallel::new(
            motions
                .into_iter()
                .map(|motion| self.regester_motion(motion))
                .collect(),
        );

        self.play_for(Box::new(parallel), duration)
    }

    fn add_object(&mut self, id: ObjectId, object: Object, rooted: bool) {
        self.play(Box::new(AddObject {
            object_id: id,
            object,
            rooted,
        }));
    }

    fn add_new_object(&mut self, object: Object) -> ObjectId {
        let id = rand::random::<usize>();
        self.add_object(id, object, true);
        id
    }

    fn animate(
        &mut self,
        object_id: ObjectId,
        duration: f32,
        animation_creator: impl FnOnce(AnimationBuilder<Self>) -> AnimationBuilder<Self>,
    ) {
        let _ = animation_creator(AnimationBuilder::new(object_id, duration, self));
    }

    fn build(&mut self, block: impl Block) -> ObjectBuilder<Self> {
        ObjectBuilder::new(block.root(self), self)
    }

    fn add(&mut self, block: impl Block) -> ObjectId {
        self.build(block).add()
    }

    fn group(&mut self, child_creator: impl FnOnce(GroupBuilder<Self>) -> GroupBuilder<Self>) {
        let group_builder = child_creator(GroupBuilder {
            builder: self,
            children: Vec::new(),
        });

        let children = group_builder.children;

        self.add_new_object(Object::new_group(children));
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
    pub children: Vec<ObjectId>,
}

impl<'a, B: Builder> Builder for GroupBuilder<'a, B> {
    fn state(&mut self) -> &mut BuilderState {
        self.builder.state()
    }

    fn add_object(&mut self, id: ObjectId, object: Object, rooted: bool) {
        if rooted {
            self.children.push(id);
        }
        self.builder.add_object(id, object, false);
    }
}
