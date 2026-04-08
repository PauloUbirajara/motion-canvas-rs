use crate::engine::animation::Node;
use vello::Scene;

pub trait Scene2D {
    fn render(&self, scene: &mut Scene);
    fn update(&mut self, dt: std::time::Duration);
    fn state_hash(&self) -> u64;
}

pub struct BaseScene {
    pub nodes: Vec<Box<dyn Node>>,
    pub timeline: crate::engine::animation::Timeline,
}

impl BaseScene {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            timeline: crate::engine::animation::Timeline::new(),
        }
    }

    pub fn add(&mut self, node: Box<dyn Node>) {
        self.nodes.push(node);
    }
}

impl Scene2D for BaseScene {
    fn render(&self, scene: &mut Scene) {
        for node in &self.nodes {
            node.render(scene);
        }
    }

    fn update(&mut self, dt: std::time::Duration) {
        self.timeline.update(dt);
        for node in &mut self.nodes {
            node.update(dt);
        }
    }

    fn state_hash(&self) -> u64 {
        let mut hash = 0u64;
        for node in &self.nodes {
            hash ^= node.state_hash();
        }
        hash
    }
}
