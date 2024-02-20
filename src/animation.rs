use crate::{
    motion::{Alpha, Concurrently, Motion, Sequence},
    world::World,
};

pub type Time = f32;

pub trait Animation: Motion {
    fn duration(&self) -> Time;
}

pub struct MotionAnimation<M: Motion> {
    pub duration: Time,
    pub motion: M,
}

impl<M: Motion> Motion for MotionAnimation<M> {
    fn animate(&self, world: &mut World, alpha: Alpha) {
        self.motion.animate(world, alpha);
    }
}

impl<M: Motion> Animation for MotionAnimation<M> {
    fn duration(&self) -> Time {
        self.duration
    }
}

pub type GenericAnimation = Box<dyn Animation>;

impl Motion for GenericAnimation {
    fn animate(&self, world: &mut World, alpha: Alpha) {
        self.as_ref().animate(world, alpha);
    }
}

impl Animation for GenericAnimation {
    fn duration(&self) -> Time {
        self.as_ref().duration()
    }
}

impl Animation for Sequence {
    fn duration(&self) -> Time {
        self.0.iter().map(|a| a.duration()).sum()
    }
}

impl Animation for Concurrently {
    fn duration(&self) -> Time {
        self.0
            .iter()
            .map(|a| a.duration())
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
    }
}
