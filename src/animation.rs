use crate::world::World;

pub trait Animation {
    // Time is an f32 between 0 and 1.
    fn animate(&self, world: &mut World, time: f32);
}

pub struct Sequence {
    animations: Vec<Box<dyn Animation>>,
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

pub struct Parallel {
    animations: Vec<Box<dyn Animation>>,
}

impl Animation for Parallel {
    fn animate(&self, world: &mut World, time: f32) {
        for animation in &self.animations {
            animation.animate(world, time);
        }
    }
}

pub struct Keyframe {
    from_min: f32,
    from_max: f32,
    to_min: f32,
    to_max: f32,

    animation: Box<dyn Animation>,
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
