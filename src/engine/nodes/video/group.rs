use crate::engine::animation::{Node, Signal};
use glam::Vec2;
use std::time::Duration;
use vello::kurbo::Affine;
use vello::Scene;

pub struct GroupNode {
    pub nodes: Vec<Box<dyn Node>>,
    pub position: Signal<Vec2>,
    pub rotation: Signal<f32>,
    pub scale: Signal<Vec2>,
    pub opacity: Signal<f32>,
}

impl Default for GroupNode {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            position: Signal::new(Vec2::ZERO),
            rotation: Signal::new(0.0),
            scale: Signal::new(Vec2::ONE),
            opacity: Signal::new(1.0),
        }
    }
}

impl GroupNode {
    pub fn new(nodes: Vec<Box<dyn Node>>) -> Self {
        Self::default().with_nodes(nodes)
    }

    pub fn with_position(mut self, pos: Vec2) -> Self {
        self.position = Signal::new(pos);
        self
    }

    pub fn with_opacity(mut self, a: f32) -> Self {
        self.opacity = Signal::new(a);
        self
    }

    pub fn with_rotation(mut self, angle: f32) -> Self {
        self.rotation = Signal::new(angle);
        self
    }

    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = Signal::new(Vec2::splat(scale));
        self
    }

    pub fn with_scale_xy(mut self, scale: Vec2) -> Self {
        self.scale = Signal::new(scale);
        self
    }

    pub fn with_nodes(mut self, nodes: Vec<Box<dyn Node>>) -> Self {
        self.nodes = nodes;
        self
    }
}

impl Clone for GroupNode {
    fn clone(&self) -> Self {
        Self {
            nodes: self.nodes.iter().map(|n| n.clone_node()).collect(),
            position: self.position.clone(),
            rotation: self.rotation.clone(),
            scale: self.scale.clone(),
            opacity: self.opacity.clone(),
        }
    }
}

impl Node for GroupNode {
    fn render(&self, scene: &mut Scene, parent_transform: Affine, parent_opacity: f32) {
        let opacity = self.opacity.get();

        let pos = self.position.get();
        let rot = self.rotation.get();
        let sc = self.scale.get();

        let local_transform = Affine::translate((pos.x as f64, pos.y as f64))
            * Affine::rotate(rot as f64)
            * Affine::scale_non_uniform(sc.x as f64, sc.y as f64);

        let combined_transform = parent_transform * local_transform;
        let combined_opacity = parent_opacity * opacity;

        if combined_opacity <= 0.0 {
            return;
        }

        for node in &self.nodes {
            node.render(scene, combined_transform, combined_opacity);
        }
    }

    fn update(&mut self, dt: Duration) {
        for node in &mut self.nodes {
            node.update(dt);
        }
    }

    fn state_hash(&self) -> u64 {
        let mut s = self.position.state_hash()
            ^ self.rotation.state_hash()
            ^ self.scale.state_hash()
            ^ self.opacity.state_hash();

        for node in &self.nodes {
            s ^= node.state_hash();
        }

        s
    }

    fn clone_node(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
