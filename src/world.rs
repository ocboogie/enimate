use std::collections::HashMap;

use crate::motion::{Motion, MotionId};
use crate::object_tree::ObjectTree;

pub type Variable = usize;

pub struct World<'a> {
    pub objects: &'a mut ObjectTree,
    variables: HashMap<Variable, f32>,
    variable_subscriptions: &'a HashMap<Variable, Vec<MotionId>>,
    motions: &'a HashMap<MotionId, Box<dyn Motion>>,
}

impl<'a> World<'a> {
    pub fn new(
        objects: &'a mut ObjectTree,
        motions: &'a HashMap<MotionId, Box<dyn Motion>>,
        variable_subscriptions: &'a HashMap<Variable, Vec<MotionId>>,
    ) -> Self {
        Self {
            objects,
            variables: HashMap::new(),
            variable_subscriptions,
            motions,
        }
    }

    pub fn update_variable(&mut self, variable: Variable, value: f32) {
        self.variables.insert(variable, value);

        // TODO: cloned() is probably not necessary here.
        // Although, then the alternative is to remove the subscriptions from the list.
        if let Some(subscriptions) = self.variable_subscriptions.get(&variable).cloned() {
            for motion in subscriptions {
                self.play_at(motion, value);
            }
        }
    }

    pub fn update_variables(&mut self, variables: &HashMap<Variable, f32>) {
        for (variable, value) in variables {
            self.update_variable(*variable, *value);
        }
    }

    pub fn play(&mut self, motion: MotionId) {
        self.play_at(motion, 1.0);
    }

    pub fn play_at(&mut self, motion: MotionId, time: f32) {
        // TODO: Provide a warning here somehow if the motion doesn't exist.
        let motion = self.motions.get(&motion).unwrap();

        motion.animate(self, time);
    }
}
