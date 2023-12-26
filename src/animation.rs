use crate::animation_ui::AnimationUI;
use crate::{object::Transform, world::World};

pub trait Animation: AnimationUI {
    // Time is an f32 between 0 and 1.
    fn animate(&self, world: &mut World, time: f32);
}

pub struct Sequence {
    pub animations: Vec<Box<dyn Animation>>,
}

impl Animation for Sequence {
    fn animate(&self, world: &mut World, time: f32) {
        let current_animation_index = (time * self.animations.len() as f32) as usize;
        let current_animation_time =
            time * self.animations.len() as f32 - current_animation_index as f32;
        let current_animation = &self.animations[current_animation_index];

        current_animation.animate(world, current_animation_time);
    }
}

pub struct Noop;

impl Animation for Noop {
    fn animate(&self, _world: &mut World, _time: f32) {}
}

pub struct Parallel {
    pub animations: Vec<Box<dyn Animation>>,
}

impl Parallel {
    pub fn new(animations: Vec<Box<dyn Animation>>) -> Self {
        Self { animations }
    }
}

impl Animation for Parallel {
    fn animate(&self, world: &mut World, time: f32) {
        for animation in &self.animations {
            animation.animate(world, time);
        }
    }
}

pub struct Keyframe {
    pub from_min: f32,
    pub from_max: f32,
    pub to_min: f32,
    pub to_max: f32,

    pub animation: Box<dyn Animation>,
}

impl Keyframe {
    pub fn new(
        from_min: f32,
        from_max: f32,
        to_min: f32,
        to_max: f32,
        animation: Box<dyn Animation>,
    ) -> Self {
        Self {
            from_min,
            from_max,
            to_min,
            to_max,
            animation,
        }
    }
}

impl Animation for Keyframe {
    fn animate(&self, world: &mut World, time: f32) {
        let adjusted_time = (time - self.from_min) / (self.from_max - self.from_min);

        self.animation.animate(
            world,
            adjusted_time * (self.to_max - self.to_min) + self.to_min,
        );
    }
}

pub struct AnimateTransform {
    pub object_id: usize,
    pub from: Transform,
    pub to: Transform,
}

impl AnimateTransform {
    pub fn new(object_id: usize, from: Transform, to: Transform) -> Self {
        Self {
            object_id,
            from,
            to,
        }
    }
}

impl Animation for AnimateTransform {
    fn animate(&self, world: &mut World, time: f32) {
        let object = world.objects.get_mut(&self.object_id).unwrap();

        object.transform = Transform {
            position: time * (self.to.position - self.from.position.to_vec2())
                + self.from.position.to_vec2(),
            scale: time * (self.to.scale - self.from.scale) + self.from.scale,
            rotation: time * (self.to.rotation - self.from.rotation) + self.from.rotation,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ExpectTime(f32);

    impl Animation for ExpectTime {
        fn animate(&self, world: &mut World, time: f32) {
            assert_eq!(self.0, time);
        }
    }

    #[test]
    fn test_keyframe() {
        let mut world = World::default();

        Keyframe::new(0.0, 1.0, 0.0, 1.0, Box::new(ExpectTime(0.0))).animate(&mut world, 0.0);
        Keyframe::new(0.0, 1.0, 0.0, 1.0, Box::new(ExpectTime(0.5))).animate(&mut world, 0.5);
        Keyframe::new(0.0, 1.0, 1.0, 2.0, Box::new(ExpectTime(1.5))).animate(&mut world, 0.5);
        Keyframe::new(5.0, 10.0, 0.0, 1.0, Box::new(ExpectTime(0.5))).animate(&mut world, 7.0);
    }
}
