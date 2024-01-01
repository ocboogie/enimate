use std::collections::HashMap;

use crate::{
    component::Component,
    // component::{Component, ComponentBuilder},
    mesh::{Mesh, Vertex},
    motion::{
        self, AddObject, AnimateTransform, FadeIn, Keyframe, Motion, MotionId, NoOp, Parallel,
        Sequence, Trigger,
    },
    object::{Material, Object, ObjectId, ObjectKind, Transform},
    object_tree::ObjectTree,
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
    pub objects: ObjectTree,
    pub scene_length: f32,
}

impl BuilderState {
    pub fn new(scene_length: f32) -> Self {
        Self {
            motions: HashMap::new(),
            objects: ObjectTree::new(),
            scene_length,
        }
    }

    pub fn emulate_motion(&mut self, motion: &dyn Motion) {
        // Run the motion, so the state of objects is consistent with the end of the motion.
        let world = &mut World::new(1.0, &mut self.objects, &self.motions);
        motion.animate(world);
    }

    pub fn normalize_time(&self, time: f32) -> f32 {
        time / self.scene_length
    }
}

pub struct ParallelBuilder<'a> {
    state: &'a mut BuilderState,
    motions: Vec<MotionId>,
    duration: f32,
}

impl<'a> Builder for ParallelBuilder<'a> {
    fn state(&mut self) -> &mut BuilderState {
        &mut self.state
    }

    fn play(&mut self, motion: Box<dyn Motion>, duration: f32) -> MotionId {
        let motion_id = self.add_motion(motion);

        self.motions.push(motion_id);
        self.duration = self.duration.max(duration);

        motion_id
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
pub struct AnimationBuilder<'a> {
    object_id: ObjectId,
    animations: Vec<Box<dyn Motion>>,
    state: &'a BuilderState,
}

impl<'a> AnimationBuilder<'a> {
    fn new(object_id: ObjectId, state: &'a BuilderState) -> Self {
        Self {
            object_id,
            animations: Vec::new(),
            state,
        }
    }

    fn object(&mut self) -> &Object {
        self.state.objects.get(&self.object_id).unwrap()
    }

    pub fn translate(mut self, end: egui::Pos2) -> Self {
        let a = AnimateTransform::new(
            self.object_id,
            self.object().transform,
            self.object().transform.with_position(end),
        );
        self.animations.push(Box::new(a));
        self
    }

    // pub fn delay(self, duration: f32) -> Self {
    //     self.builder.play_for(Box::new(NoOp), duration);
    //     self
    // }

    pub fn move_to(mut self, pos: impl Positioner) -> Self {
        let target_pos = pos.position(self.object_id, self.state);
        let a = AnimateTransform::new(
            self.object_id,
            self.object().transform,
            self.object().transform.with_position(target_pos),
        );
        self.animations.push(Box::new(a));

        self
    }

    pub fn rotate(mut self, rotation: f32) -> Self {
        let a = Box::new(AnimateTransform::new(
            self.object_id,
            self.object().transform,
            self.object().transform.with_rotation(rotation),
        ));
        self.animations.push(a);
        self
    }

    pub fn scale(mut self, scale: f32) -> Self {
        let a = Box::new(AnimateTransform::new(
            self.object_id,
            self.object().transform,
            self.object().transform.with_scale(scale),
        ));
        self.animations.push(a);
        self
    }

    pub fn fade_in(mut self) -> Self {
        self.animations.push(Box::new(FadeIn {
            object_id: self.object_id,
        }));
        self
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
        self.builder.add_object(self.object_id, self.object);
        self.object_id
    }

    pub fn animate(
        self,
        duration: f32,
        animation_creator: impl FnOnce(AnimationBuilder) -> AnimationBuilder,
    ) -> ObjectId {
        self.builder.add_object(self.object_id, self.object);

        self.builder
            .animate(self.object_id, duration, animation_creator);

        self.object_id
        //
        // let _ = animation_creator(AnimationBuilder::new(self.object_id, self.builder.state()));
        //
        // self.object_id
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
//
// impl<C: Component> Block for C {
//     fn root(self, builder: &mut impl Builder) -> Object {
//         let mut group_builder = GroupBuilder {
//             builder,
//             children: Vec::new(),
//         };
//
//         self.build(&mut group_builder);
//
//         Object::new_group(group_builder.children)
//     }
// }

pub trait Builder: Sized {
    fn state(&mut self) -> &mut BuilderState;
    fn play(&mut self, motion: Box<dyn Motion>, duration: f32) -> MotionId;
    // Wheather new objects should be added to the root of the object tree.
    fn rooted(&mut self) -> bool {
        true
    }

    fn regester_motion(&mut self, id: usize, motion: Box<dyn Motion>) {
        self.state().motions.insert(id, motion);
    }

    fn add_motion(&mut self, motion: Box<dyn Motion>) -> MotionId {
        let motion_id = rand::random::<usize>();
        self.regester_motion(motion_id, motion);
        motion_id
    }

    fn add_object(&mut self, id: ObjectId, object: Object) {
        let rooted = self.rooted();
        let add_object = AddObject {
            object_id: id,
            object,
            rooted,
        };
        self.play(Box::new(add_object), 0.0);
    }

    // Same as add_object, but generates a random id.
    fn add_new_object(&mut self, object: Object) -> ObjectId {
        let object_id = rand::random::<ObjectId>();
        self.add_object(object_id, object);
        object_id
    }

    fn animate(
        &mut self,
        object_id: ObjectId,
        duration: f32,
        animation_creator: impl FnOnce(AnimationBuilder) -> AnimationBuilder,
    ) {
        let animation_builder = animation_creator(AnimationBuilder::new(object_id, self.state()));

        // self.play(Box::new(Parallel { motions: ids }), duration);

        if animation_builder.animations.len() == 1 {
            let animation = animation_builder.animations.into_iter().next().unwrap();
            self.play(animation, duration);
        } else {
            let ids = animation_builder
                .animations
                .into_iter()
                .map(|animation| self.add_motion(animation))
                .collect();

            self.play(Box::new(Parallel { motions: ids }), duration);
        }
    }

    fn sequence(&mut self, builder: impl FnOnce(&mut SequenceBuilder)) {
        let mut sequence_builder = SequenceBuilder {
            state: self.state(),
            motions: Vec::new(),
        };

        builder(&mut sequence_builder);

        let duration = sequence_builder
            .motions
            .iter()
            .map(|(duration, _)| duration)
            .sum();

        let sequence = Sequence {
            motions: sequence_builder.motions,
        };

        self.play(Box::new(sequence), duration);
    }

    fn parallel(&mut self, builder: impl FnOnce(&mut ParallelBuilder)) {
        let mut parallel_builder = ParallelBuilder {
            state: self.state(),
            motions: Vec::new(),
            duration: 0.0,
        };

        builder(&mut parallel_builder);

        let sequence = Parallel {
            motions: parallel_builder.motions,
        };

        let duration = parallel_builder.duration;

        self.play(Box::new(sequence), duration);
    }

    fn delay(&mut self, duration: f32) {
        self.play(Box::new(NoOp), duration);
    }

    // fn build(&mut self, block: impl Block) -> ObjectBuilder<Self> {
    //     ObjectBuilder::new(block.root(self), self)
    // }
    //
    // fn add(&mut self, block: impl Block) -> ObjectId {
    //     self.build(block).add()
    // }

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

pub struct SequenceBuilder<'a> {
    pub state: &'a mut BuilderState,
    pub motions: Vec<(f32, MotionId)>,
}

impl Builder for SequenceBuilder<'_> {
    fn state(&mut self) -> &mut BuilderState {
        self.state
    }

    fn play(&mut self, motion: Box<dyn Motion>, duration: f32) -> MotionId {
        let motion_id = self.add_motion(motion);

        self.motions.push((duration, motion_id));

        motion_id
    }
}

pub struct GroupBuilder<'a, B: Builder> {
    builder: &'a mut B,
    pub children: Vec<ObjectId>,
}

impl<'a, B: Builder> GroupBuilder<'a, B> {
    pub fn new(builder: &'a mut B, rooted: bool) -> Self {
        Self {
            builder,
            children: Vec::new(),
        }
    }
}

impl<'a, B: Builder> Builder for GroupBuilder<'a, B> {
    fn state(&mut self) -> &mut BuilderState {
        self.builder.state()
    }

    fn play(&mut self, motion: Box<dyn Motion>, duration: f32) -> MotionId {
        self.builder.play(motion, duration)
    }

    fn rooted(&mut self) -> bool {
        false
    }
}
