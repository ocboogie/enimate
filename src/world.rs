use crate::object_tree::ObjectTree;
use std::collections::HashMap;

pub type Variable = usize;

pub struct World {
    pub objects: ObjectTree,
    render_size: (f32, f32),
    variables: HashMap<Variable, f32>,
}

impl World {
    pub fn new(
        objects: ObjectTree,
        render_size: (f32, f32),
        variables: HashMap<Variable, f32>,
    ) -> Self {
        Self {
            objects,
            render_size,
            variables,
        }
    }

    pub fn render_size(&self) -> (f32, f32) {
        self.render_size
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
