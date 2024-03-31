use crate::animation::{Animation, MotionAnimation};
use crate::dynamics::{DynamicObject, DynamicTransform, OwnedDynamic};
use crate::easing::Easing;
use crate::object::{Object, ObjectId, ObjectKind, Transform};
use crate::scene::Scene;
use crate::trigger::Trigger;
use crate::world::World;
use egui::{Color32, Pos2};

pub type Alpha = f32;

/// A motion is the most basic momvement primitive. It is a function that takes a world
/// and updates the world, adding objects, mutating them, or animating them. The time is a value
/// between 0 and 1.
#[must_use]
pub trait Motion: 'static {
    fn animate(&self, world: &mut World, alpha: Alpha);

    fn with_duration(self, duration: f32) -> MotionAnimation<Self>
    where
        Self: Sized,
    {
        MotionAnimation {
            motion: self,
            duration,
            easing: Easing::default(),
        }
    }
}

impl Motion for fn(&mut World, Alpha) {
    fn animate(&self, world: &mut World, alpha: Alpha) {
        self(world, alpha);
    }
}

impl Motion for Box<dyn Motion> {
    fn animate(&self, world: &mut World, alpha: Alpha) {
        self.as_ref().animate(world, alpha);
    }
}

pub struct EmbededScene {
    pub scene: Scene,
    pub transform: OwnedDynamic<Transform>,
    pub speed: f32,
    pub object_id: ObjectId,
    pub rooted: bool,
}

impl Motion for EmbededScene {
    fn animate(&self, world: &mut World, alpha: Alpha) {
        let transform = self.transform.get(world);

        let render_size = world.render_size();
        let adjusted_render_size = (
            render_size.0 / transform.scale,
            render_size.1 / transform.scale,
        );

        let objects = self.scene.render_at(
            self.scene.length() * alpha * self.speed,
            adjusted_render_size,
        );

        let children = world.objects.merge(objects, self.object_id);

        world.objects.add(
            self.object_id,
            Object {
                transform,
                object_kind: ObjectKind::Group(children),
            },
            self.rooted,
        );
    }
}

impl Animation for EmbededScene {
    fn duration(&self) -> f32 {
        self.scene.length() / self.speed
    }
}

pub struct AddObject {
    pub object_id: usize,
    pub object: OwnedDynamic<Object>,
    pub rooted: bool,
}

impl Trigger for AddObject {
    fn trigger(&self, world: &mut World) {
        let object = self.object.get(world);
        world.objects.add(self.object_id, object, self.rooted);
    }
}

pub struct Move {
    pub from: OwnedDynamic<Pos2>,
    pub to: OwnedDynamic<Pos2>,
    pub object_id: usize,
}

impl Motion for Move {
    fn animate(&self, world: &mut World, alpha: Alpha) {
        let from = self.from.get(world);
        let to = self.to.get(world);

        let pos = from + (to - from) * alpha;

        world
            .objects
            .get_mut(&self.object_id)
            .unwrap()
            .transform
            .position = pos;
    }
}

pub struct MoveTo {
    pub to: OwnedDynamic<Pos2>,
    pub object_id: usize,
}

impl Motion for MoveTo {
    fn animate(&self, world: &mut World, alpha: Alpha) {
        let to = self.to.get(world);
        let from = world
            .objects
            .get(&self.object_id)
            .unwrap()
            .transform
            .position;

        let pos = from + (to - from) * alpha;

        world
            .objects
            .get_mut(&self.object_id)
            .unwrap()
            .transform
            .position = pos;
    }
}

pub struct FadeIn {
    pub object_id: usize,
}

fn fade_in(world: &mut World, alpha: Alpha, object_id: usize) {
    let mut object = world.objects.remove(&object_id).unwrap();

    match &mut object.object_kind {
        ObjectKind::Model(ref mut model) => {
            if let Some(ref mut fill) = model.material.fill {
                let c = fill.color;
                fill.color =
                    Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), (alpha * 255.0) as u8);
            }

            if let Some(ref mut stroke) = model.material.stroke {
                let c = stroke.color;
                stroke.color =
                    Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), (alpha * 255.0) as u8);
            }
        }
        ObjectKind::Group(group) => {
            for child_id in group.iter() {
                fade_in(world, alpha, *child_id);
            }
        }
    }

    world.objects.add(object_id, object, false);
}

impl Motion for FadeIn {
    fn animate(&self, world: &mut World, alpha: f32) {
        // FIXME: We should be able to animate the alpha value of the color.
        fade_in(world, alpha, self.object_id);
    }
}
