use crate::engine::animation::{Node, Signal};
use glam::Vec2;
use std::time::Duration;
use vello::kurbo::Affine;
use vello::Scene;

pub struct GroupNode {
    pub nodes: Vec<Box<dyn Node>>,
    pub transform: Signal<Affine>,
    pub opacity: Signal<f32>,
}

impl Default for GroupNode {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            transform: Signal::new(Affine::IDENTITY),
            opacity: Signal::new(1.0),
        }
    }
}

impl GroupNode {
    pub fn new(nodes: Vec<Box<dyn Node>>) -> Self {
        Self::default().with_nodes(nodes)
    }

    pub fn with_transform(mut self, transform: Affine) -> Self {
        self.transform = Signal::new(transform);
        self
    }

    pub fn with_position(mut self, pos: Vec2) -> Self {
        self.transform = Signal::new(Affine::translate((pos.x as f64, pos.y as f64)));
        self
    }

    pub fn with_opacity(mut self, a: f32) -> Self {
        self.opacity = Signal::new(a);
        self
    }

    pub fn with_rotation(mut self, angle: f32) -> Self {
        let current = self.transform.get();
        let coeffs = current.as_coeffs();
        let tx = coeffs[4];
        let ty = coeffs[5];
        self.transform = Signal::new(Affine::translate((tx, ty)) * Affine::rotate(angle as f64));
        self
    }

    pub fn with_scale(mut self, scale: f32) -> Self {
        let current = self.transform.get();
        let coeffs = current.as_coeffs();
        let tx = coeffs[4];
        let ty = coeffs[5];
        self.transform = Signal::new(Affine::translate((tx, ty)) * Affine::scale(scale as f64));
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
            transform: self.transform.clone(),
            opacity: self.opacity.clone(),
        }
    }
}

impl Node for GroupNode {
    fn render(&self, scene: &mut Scene, parent_transform: Affine, parent_opacity: f32) {
        let local_transform = self.transform.get();
        let opacity = self.opacity.get();

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
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut s = DefaultHasher::new();

        let coeffs = self.transform.get().as_coeffs();
        for c in coeffs {
            c.to_bits().hash(&mut s);
        }
        self.opacity.get().to_bits().hash(&mut s);

        for node in &self.nodes {
            node.state_hash().hash(&mut s);
        }

        s.finish()
    }

    fn clone_node(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
