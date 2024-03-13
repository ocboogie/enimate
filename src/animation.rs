use crate::{
    easing::Easing,
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
    pub easing: Easing,
}

impl<M: Motion> MotionAnimation<M> {
    pub fn with_easing(self, easing: Easing) -> Self {
        Self { easing, ..self }
    }
}

impl<M: Motion> Motion for MotionAnimation<M> {
    fn animate(&self, world: &mut World, alpha: Alpha) {
        let adjusted_alpha = self.easing.apply(alpha);

        self.motion.animate(world, adjusted_alpha);
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
