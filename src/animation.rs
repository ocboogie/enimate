use crate::{
    motion::{Alpha, Motion},
    timing::{Sequence, Time},
    world::World,
};

pub trait Animation: Motion + 'static {
    fn duration(&self) -> Time;

    fn then<A: Animation>(self, other: A) -> Sequence
    where
        Self: Sized,
    {
        Sequence(vec![Box::new(self), Box::new(other)])
    }
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
