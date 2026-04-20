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
    pub size: Signal<Vec2>,
    pub anchor: Signal<Vec2>,
}

impl Default for GroupNode {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            position: Signal::new(Vec2::ZERO),
            rotation: Signal::new(0.0),
            scale: Signal::new(Vec2::ONE),
            opacity: Signal::new(1.0),
            size: Signal::new(Vec2::ZERO),
            anchor: Signal::new(Vec2::ZERO),
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

    pub fn with_size(mut self, size: Vec2) -> Self {
        self.size = Signal::new(size);
        self
    }

    pub fn with_anchor(mut self, anchor: Vec2) -> Self {
        self.anchor = Signal::new(anchor);
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
            size: self.size.clone(),
            anchor: self.anchor.clone(),
        }
    }
}

impl Node for GroupNode {
    fn render(&self, scene: &mut Scene, parent_transform: Affine, parent_opacity: f32) {
        let opacity = self.opacity.get();

        let pos = self.position.get();
        let rot = self.rotation.get();
        let sc = self.scale.get();
        let anchor = self.anchor.get();
        let size = self.size.get();

        let anchor_offset = anchor * size * 0.5;

        let local_transform = Affine::translate((pos.x as f64, pos.y as f64))
            * Affine::rotate(rot as f64)
            * Affine::scale_non_uniform(sc.x as f64, sc.y as f64)
            * Affine::translate((-anchor_offset.x as f64, -anchor_offset.y as f64));

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
        use crate::engine::util::hash::Hasher;
        let mut h = Hasher::new();
        h.update_u64(self.position.state_hash());
        h.update_u64(self.rotation.state_hash());
        h.update_u64(self.scale.state_hash());
        h.update_u64(self.opacity.state_hash());
        h.update_u64(self.size.state_hash());
        h.update_u64(self.anchor.state_hash());

        for node in &self.nodes {
            h.update_u64(node.state_hash());
        }

        h.finish()
    }

    fn clone_node(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn reset(&mut self) {
        self.position.reset();
        self.rotation.reset();
        self.scale.reset();
        self.opacity.reset();
        self.size.reset();
        self.anchor.reset();
        for node in &mut self.nodes {
            node.reset();
        }
    }
}
