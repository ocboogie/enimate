use crate::{
    mesh::{Mesh, Vertex},
    motion::{AddObject, Motion, MotionId, NoOp, Parallel, Sequence},
    object::{Material, Object, ObjectId},
};
use egui::pos2;
use lyon::{
    geom::Box2D,
    lyon_tessellation::{
        BuffersBuilder, FillGeometryBuilder, FillVertex, StrokeGeometryBuilder, StrokeOptions,
        StrokeTessellator, StrokeVertex,
    },
    math::point,
    path::{Path, Winding},
    tessellation::{FillOptions, FillTessellator, VertexBuffers},
};

use super::{
    AnimationBuilder, BuilderState, GroupBuilder, ObjectBuilder, ParallelBuilder, SequenceBuilder,
};

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
        let mut builder = Path::builder();
        builder.add_circle(point(0.0, 0.0), radius, Winding::Positive);

        let object = Object::new_model(builder.build(), material);

        ObjectBuilder::new(object, self)
    }

    fn line(
        &mut self,
        start: egui::Pos2,
        end: egui::Pos2,
        material: Material,
    ) -> ObjectBuilder<Self> {
        let mut builder = Path::builder();
        builder.begin(point(start.x, start.y));
        builder.line_to(point(end.x, end.y));
        builder.close();

        let path = builder.build();

        let object = Object::new_model(path, material);

        ObjectBuilder::new(object, self)
    }

    fn rect(&mut self, width: f32, height: f32, material: Material) -> ObjectBuilder<Self> {
        let mut builder = Path::builder();
        builder.add_rectangle(
            &Box2D::new(
                point(-width / 2.0, -height / 2.0),
                point(width / 2.0, height / 2.0),
            ),
            Winding::Positive,
        );

        let object = Object::new_model(builder.build(), material);

        ObjectBuilder::new(object, self)
    }

    fn path(&mut self, path: Path, material: Material) -> ObjectBuilder<Self> {
        let object = Object::new_model(path, material);

        ObjectBuilder::new(object, self)
    }
}
