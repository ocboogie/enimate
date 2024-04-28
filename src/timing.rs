// use crate::{
//     animation::{Animation, GenericAnimation},
//     motion::{Alpha, Motion},
//     world::World,
// };

pub type Time = f32;

// pub struct Wait;
//
// impl Motion for Wait {
//     fn animate(&self, _world: &mut World, _alpha: Alpha) {}
// }
//
// #[derive(Default)]
// pub struct Sequence(pub Vec<GenericAnimation>);
//
// impl Sequence {
//     pub fn add<A: Animation + 'static>(&mut self, animation: A) {
//         self.0.push(Box::new(animation));
//     }
// }
//
// impl Motion for Sequence {
//     fn animate(&self, world: &mut World, alpha: Alpha) {
//         let total_duration: f32 = self.0.iter().map(|a| a.duration()).sum();
//         let mut current_alpha = 0.0;
//
//         for animation in &self.0 {
//             let normalized_alpha = animation.duration() / total_duration;
//
//             let adusted_alpha = (alpha - current_alpha) / normalized_alpha;
//
//             if adusted_alpha < 0.0 {
//                 return;
//             }
//
//             animation.animate(world, adusted_alpha);
//             current_alpha += normalized_alpha;
//         }
//     }
// }
//
// impl From<Vec<GenericAnimation>> for Sequence {
//     fn from(animations: Vec<GenericAnimation>) -> Self {
//         Self(animations)
//     }
// }
//
// impl Animation for Sequence {
//     fn duration(&self) -> Time {
//         self.0.iter().map(|a| a.duration()).sum()
//     }
// }
//
// #[derive(Default)]
// pub struct Concurrently(
//     /// Order matters!
//     pub Vec<GenericAnimation>,
// );
//
// impl Concurrently {
//     pub fn add<A: Animation + 'static>(&mut self, animation: A) {
//         self.0.push(Box::new(animation));
//     }
// }
//
// impl FromIterator<GenericAnimation> for Concurrently {
//     fn from_iter<T: IntoIterator<Item = GenericAnimation>>(iter: T) -> Self {
//         Self(iter.into_iter().collect())
//     }
// }
//
// impl Motion for Concurrently {
//     fn animate(&self, world: &mut World, alpha: Alpha) {
//         let total_duration: f32 = self.duration();
//
//         for animation in &self.0 {
//             animation.animate(
//                 world,
//                 ((total_duration * alpha) / animation.duration()).min(1.0),
//             );
//         }
//     }
// }
//
// impl Animation for Concurrently {
//     fn duration(&self) -> Time {
//         self.0
//             .iter()
//             .map(|a| a.duration())
//             .max_by(|a, b| a.partial_cmp(b).unwrap())
//             .unwrap()
//     }
// }
