use std::collections::HashMap;

use crate::motion::{Motion, MotionId};
use crate::object_tree::ObjectTree;

pub type Variable = usize;

pub struct World<'a> {
    pub objects: &'a mut ObjectTree,
    pub variables: HashMap<Variable, f32>,
    // variable_trackers: &'a HashMap<Variable, Vec<MotionId>>,
    motions: &'a HashMap<MotionId, Box<dyn Motion>>,
}

impl<'a> World<'a> {
    pub fn new(
        objects: &'a mut ObjectTree,
        motions: &'a HashMap<MotionId, Box<dyn Motion>>,
        // variable_trackers: &'a HashMap<Variable, Vec<MotionId>>,
    ) -> Self {
        Self {
            objects,
            variables: HashMap::new(),
            // variable_trackers,
            motions,
        }
    }

    pub fn update_variable(&mut self, variable: Variable, value: f32) {
        self.variables.insert(variable, value);

        // if let Some(trackers) = self.variable_trackers.remove(&variable) {
        //     for motion in &trackers {
        //         self.play_at(*motion, value);
        //     }
        //
        //     self.variable_trackers.insert(variable, trackers);
        // }
    }

    pub fn update_variables(&mut self, variables: &HashMap<Variable, f32>) {
        for (variable, value) in variables {
            self.update_variable(*variable, *value);
        }
    }

    pub fn get_variable(&self, variable: Variable) -> f32 {
        *self.variables.get(&variable).expect("Variable not found")
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
