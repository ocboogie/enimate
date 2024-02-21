use std::collections::HashMap;

use crate::animation::Time;
use crate::motion::{Motion, MotionId};
use crate::object_tree::ObjectTree;
use crate::scene::Scene;

pub type Variable = usize;

pub struct World<'a> {
    pub objects: &'a mut ObjectTree,
    variables: HashMap<Variable, f32>,
}

impl<'a> World<'a> {
    pub fn new(objects: &'a mut ObjectTree, variables: HashMap<Variable, f32>) -> Self {
        Self { objects, variables }
    }

    pub fn update_variable(&mut self, variable: Variable, value: f32) {
        self.variables.insert(variable, value);
    }

    pub fn update_variables(&mut self, variables: &HashMap<Variable, f32>) {
        for (variable, value) in variables {
            self.update_variable(*variable, *value);
        }
    }

    pub fn get_variable(&self, variable: Variable) -> f32 {
        *self.variables.get(&variable).expect("Variable not found")
    }
}
