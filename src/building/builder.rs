use crate::{
    animation::Animation,
    mesh::{Mesh, Vertex},
    motion::{AddObject, Concurrently, Motion, MotionId, Sequence, Wait},
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

use super::{component::Component, BuilderState};

pub trait Builder: Sized {
    fn state(&mut self) -> &mut BuilderState;
    fn play<A: Animation + 'static>(&mut self, animation: A);
    // Wheather new objects should be added to the root of the object tree.
    fn rooted(&mut self) -> bool {
        true
    }

    fn add<C: Component>(&mut self, component: C) -> C::Handle {
        component.add(self)
    }

    fn add_object(&mut self, id: ObjectId, object: Object) {
        let rooted = self.rooted();
        self.play(AddObject {
            object_id: id,
            object,
            rooted,
        });
    }

    // Same as add_object, but generates a random id.
    fn add_new_object(&mut self, object: Object) -> ObjectId {
        let object_id = rand::random::<ObjectId>();
        self.add_object(object_id, object);
        object_id
    }
}
