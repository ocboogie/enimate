use std::collections::HashMap;

use crate::{
    motion::{Motion, MotionId, NoOp},
    object_tree::ObjectTree,
    world::{Variable, World},
};

pub struct Scene {
    pub root: MotionId,
    pub motions: HashMap<MotionId, Box<dyn Motion>>,
    pub variable_subscriptions: HashMap<Variable, Vec<MotionId>>,
    pub length: f32,
}

impl Scene {
    pub fn null() -> Self {
        let mut motions: HashMap<MotionId, Box<dyn Motion>> = HashMap::new();
        let root: MotionId = rand::random::<usize>();
        motions.insert(root, Box::new(NoOp));

        Self {
            root,
            motions,
            variable_subscriptions: HashMap::new(),
            length: 0.0,
        }
    }

    pub fn new(
        motions: HashMap<MotionId, Box<dyn Motion>>,
        root: MotionId,
        variable_subscriptions: HashMap<usize, Vec<MotionId>>,
        length: f32,
    ) -> Self {
        Self {
            root,
            motions,
            variable_subscriptions,
            length,
        }
    }

    pub fn root(&self) -> &Box<dyn Motion> {
        &self.motions[&self.root]
    }

    pub fn root_mut(&mut self) -> &mut Box<dyn Motion> {
        self.motions.get_mut(&self.root).unwrap()
    }

    pub fn render_at(&mut self, time: f32) -> ObjectTree {
        let mut objects = ObjectTree::new();
        let mut world = World::new(&mut objects, &self.motions, &self.variable_subscriptions);

        self.motions[&self.root].animate(&mut world, time);

        objects
    }

    pub fn render_with_input(&mut self, time: f32, input: HashMap<Variable, f32>) -> ObjectTree {
        let mut objects = ObjectTree::new();
        let mut world = World::new(&mut objects, &self.motions, &self.variable_subscriptions);

        self.motions[&self.root].animate(&mut world, time);

        world.update_variables(&input);

        objects
    }
}
