use crate::{
    animation::Animation,
    motion::{Alpha, Motion},
    timing::Time,
    world::World,
};

pub trait Trigger: 'static {
    fn trigger(&self, world: &mut World);
}

impl<T: Trigger> Motion for T {
    fn animate(&self, world: &mut World, _time: Alpha) {
        self.trigger(world);
    }
}

impl<T: Trigger> Animation for T {
    fn duration(&self) -> Time {
        0.0
    }
}
