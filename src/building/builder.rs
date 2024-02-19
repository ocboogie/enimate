use crate::{
    mesh::{Mesh, Vertex},
    motion::{
        AddObject, Concurrently, ConcurrentlyWithDurations, Motion, MotionId, Sequence, Wait,
    },
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

use super::{component::Component, BuilderState, GroupBuilder};

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

    fn add<C: Component>(&mut self, mut component: C) -> C::Handle {
        component.add(self)
    }

    // fn animate(&mut self, builder: fn(&mut AnimationBuilder<Self>) -> ()) {
    //     let mut animation_builder = AnimationBuilder {
    //         builder: self,
    //         motions: Vec::new(),
    //     };
    //
    //     builder(&mut animation_builder);
    //
    //     if animation_builder.motions.is_empty() {
    //         return;
    //     }
    //
    //     let duration = animation_builder
    //         .motions
    //         .iter()
    //         .max_by(|x, y| x.1.partial_cmp(&y.1).unwrap())
    //         .unwrap();
    //
    //     for (motion, _) in animation_builder.motions {
    //         self.play(motion, *duration);
    //     }
    // }
    //
    // fn build<C: Component>(&mut self, mut component: C) -> ObjectBuilder<C::Handle> {
    //     let handle = component.add(self);
    //     ObjectBuilder::new(handle.id(), self));
    // }
    //
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

    fn play_concurrently(&mut self, motions: Vec<(Box<dyn Motion>, f32)>) {
        let total_duration = motions
            .iter()
            .map(|(_, duration)| duration)
            .max_by(|x, y| x.partial_cmp(y).unwrap())
            .copied()
            .unwrap();

        let motion = Box::new(ConcurrentlyWithDurations(
            motions
                .into_iter()
                .map(|(motion, duration)| (self.add_motion(motion), duration / total_duration))
                .collect(),
        ));

        self.play(motion, total_duration);
    }

    fn wait(&mut self, duration: f32) {
        self.play(Box::new(Wait), duration);
    }

    fn sequence(&mut self, sequence: Vec<(Box<dyn Motion>, f32)>) -> (Box<dyn Motion>, f32) {
        let total_duration: f32 = sequence.iter().map(|(_, duration)| duration).sum();

        (
            Box::new(Sequence(
                sequence
                    .into_iter()
                    .map(|(motion, duration)| (self.add_motion(motion), duration / total_duration))
                    .collect(),
            )),
            total_duration,
        )
    }
}

pub struct CaptureMotion<'a, B: Builder> {
    builder: &'a mut B,
    state: &'a mut BuilderState,
    object_id: ObjectId,
}

//
// pub struct AnimationBuilder<'a, B: Builder> {
//     builder: &'a mut B,
//     motions: Vec<(Box<dyn Motion>, f32)>,
// }
//
// impl<'a, B: Builder> Builder for AnimationBuilder<'a, B> {
//     fn state(&mut self) -> &mut BuilderState {
//         self.builder.state()
//     }
//
//     fn play(&mut self, motion: Box<dyn Motion>, duration: f32) -> MotionId {
//         let motion_id = self.builder.add_motion(motion);
//
//         self.motions.push((motion_id, duration));
//
//         motion_id
//     }
// }
