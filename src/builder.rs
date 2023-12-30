use std::collections::HashMap;

use crate::{
    component::Component,
    // component::{Component, ComponentBuilder},
    mesh::{Mesh, Vertex},
    motion::{
        self, AddObject, AnimateTransform, FadeIn, Keyframe, Motion, MotionId, NoOp, Parallel,
        Trigger,
    },
    object::{Material, Object, ObjectId, ObjectKind, Transform},
    object_tree::ObjectTree,
    scene_builder::SceneBuilder,
    world::World,
};
use egui::{pos2, vec2, Pos2, Rect};
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

pub struct SequenceBuilder<'a, B: Builder> {
    builder: &'a mut B,
    motions: Vec<(f32, MotionId)>,
}

impl<'a, B: Builder> SequenceBuilder<'a, B> {
    pub fn new(builder: &'a mut B) -> Self {
        Self {
            builder,
            motions: Vec::new(),
        }
    }
}

impl<'a, B: Builder> Builder for SequenceBuilder<'a, B> {
    fn state(&mut self) -> &mut BuilderState {
        self.builder.state()
    }
}

pub struct ParallelBuilder<'a, B: Builder> {
    builder: &'a mut B,
    motions: Vec<MotionId>,
    max_duration: f32,
}

impl<'a, B: Builder> ParallelBuilder<'a, B> {
    pub fn new(builder: &'a mut B) -> Self {
        Self {
            builder,
            motions: Vec::new(),
            max_duration: 0.0,
        }
    }

    pub fn sequence(&mut self, builder: impl FnOnce(&mut SequenceBuilder<B>)) {
        let start_time = self.builder.state().current_time;
        let mut sequence_builder = SequenceBuilder::new(self.builder);

        builder(&mut sequence_builder);

        self.builder.state().current_time = start_time;
    }
}

impl Builder for ParallelBuilder<'_, SceneBuilder> {
    fn state(&mut self) -> &mut BuilderState {
        self.builder.state()
    }

    fn add_motion(&mut self, motion: Box<dyn Motion>) -> MotionId {
        let motion_id = self.regester_motion(motion);

        self.motions.push(motion_id);

        motion_id
    }

    fn play(&mut self, motion: Box<dyn Motion>) -> MotionId {
        self.add_motion(motion)
    }

    fn play_for(&mut self, motion: Box<dyn Motion>, duration: f32) -> MotionId {
        let motion_id = self.regester_motion(motion);

        let current_time = self.state().current_time;
        let keyframe = Keyframe::new(current_time, current_time + duration, 0.0, 1.0, motion_id);

        self.max_duration = self.max_duration.max(duration);
        self.add_motion(Box::new(keyframe))
    }
}

pub trait Positioner {
    fn position(&self, source: ObjectId, state: &BuilderState) -> Pos2;
}

impl Positioner for Pos2 {
    fn position(&self, _source: ObjectId, _state: &BuilderState) -> Pos2 {
        *self
    }
}

pub struct Left(pub ObjectId);

impl Positioner for Left {
    fn position(&self, source: ObjectId, state: &BuilderState) -> Pos2 {
        let source_bb = state.objects.local_bounding_box(source);
        let target_bb = state.objects.local_bounding_box(self.0);

        target_bb.left_center() - vec2(source_bb.width() / 2.0, 0.0)
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

    // pub fn delay(self, duration: f32) -> Self {
    //     self.builder.play_for(Box::new(NoOp), duration);
    //     self
    // }
    //
    pub fn move_to(mut self, pos: impl Positioner) -> Self {
        let target_pos = pos.position(self.object_id, self.builder.state());
        let a = AnimateTransform::new(
            self.object_id,
            self.object().transform,
            self.object().transform.with_position(target_pos),
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

    fn bounding_box(&mut self) -> Rect {
        self.builder
            .state()
            .objects
            .local_bounding_box_obj(&self.object)
    }

    pub fn with_centered_anchor(mut self) -> Self {
        let bounding_box = self.bounding_box();

        self.object.transform.position -= bounding_box.center().to_vec2();
        self.object.transform.anchor = bounding_box.center();
        self
    }

    pub fn add(self) -> ObjectId {
        self.builder.add_object(self.object_id, self.object, true);
        self.object_id
    }

    pub fn animate(
        self,
        duration: f32,
        animation_creator: impl FnOnce(AnimationBuilder<B>) -> AnimationBuilder<B>,
    ) -> ObjectId {
        self.builder.add_object(self.object_id, self.object, true);

        let _ = animation_creator(AnimationBuilder::new(
            self.object_id,
            duration,
            self.builder,
        ));

        self.object_id
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

    fn regester_motion(&mut self, motion: Box<dyn Motion>) -> MotionId {
        let motion_id = rand::random::<usize>();

        self.state().motions.insert(motion_id, motion);
        motion_id
    }

    fn add_motion(&mut self, motion: Box<dyn Motion>) -> MotionId {
        let motion_id = self.regester_motion(motion);

        self.state().root_motions.push(motion_id);

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
        let keyframe = Keyframe::new(*current_time, *current_time + duration, 0.0, 1.0, motion_id);

        *current_time += duration;
        self.add_motion(Box::new(keyframe))
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

    fn parallel(&mut self, builder: impl FnOnce(&mut ParallelBuilder<Self>)) {
        let mut parallel_builder = ParallelBuilder::new(self);

        builder(&mut parallel_builder);

        let parallel = Parallel::new(parallel_builder.motions);

        if parallel_builder.max_duration == 0.0 {
            self.play(Box::new(parallel));
        } else {
            let max_duration = parallel_builder.max_duration;
            self.play_for(Box::new(parallel), max_duration);
        }
    }

    fn delay(&mut self, duration: f32) {
        self.play_for(Box::new(NoOp), duration);
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

    fn circle(&mut self, radius: f32, material: Material) -> ObjectBuilder<Self> {
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

        let object = Object::new_model(mesh, material);

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

    fn rect(&mut self, width: f32, height: f32, material: Material) -> ObjectBuilder<Self> {
        let mesh = self.tessellate(|mut tessellator, buffers_builder| {
            tessellator
                .tessellate_rectangle(
                    &lyon::math::Box2D::new(
                        lyon::math::point(-width / 2.0, -height / 2.0),
                        lyon::math::point(width / 2.0, height / 2.0),
                    ),
                    &FillOptions::default(),
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
