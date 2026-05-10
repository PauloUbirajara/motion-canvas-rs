use crate::core::animation::{Node, Signal};
use glam::Vec2;
use std::time::Duration;
use kurbo::Affine;
#[cfg(feature = "runtime")]
use vello::Scene;

/// A node that acts as a camera, applying its inverse transformation to all children.
///
/// `CameraNode` allows you to pan, zoom, and rotate a sub-section of your scene.
/// It works by calculating its own transform and then applying the inverse of that
/// transform to its children during the render pass.
///
/// ### Example
/// ```rust
/// # use motion_canvas_rs::prelude::*;
/// let camera = CameraNode::default()
///     .with_zoom(2.0)
///     .with_position(Vec2::new(50.0, 50.0));
/// ```
pub struct CameraNode {
    /// The list of child nodes affected by this camera.
    pub nodes: Vec<Box<dyn Node>>,
    /// The world-space position of the camera.
    pub position: Signal<Vec2>,
    /// The rotation of the camera in radians.
    pub rotation: Signal<f32>,
    /// The zoom level (1.0 is default, 2.0 is 2x zoom).
    pub zoom: Signal<f32>,
    /// The relative transformation origin. (-1,-1) is top-left, (0,0) is center, (1,1) is bottom-right.
    pub anchor: Signal<Vec2>,
    /// The opacity factor applied to the camera's viewport.
    pub opacity: Signal<f32>,
    /// The dimensions of the viewport (usually matches the project's size).
    pub size: Signal<Vec2>,
    /// If true, the world origin (0,0) is shifted to the center of the viewport.
    pub centered: Signal<bool>,
}

impl Default for CameraNode {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            position: Signal::new(Vec2::ZERO),
            rotation: Signal::new(0.0),
            zoom: Signal::new(1.0),
            anchor: Signal::new(Vec2::ZERO),
            opacity: Signal::new(1.0),
            size: Signal::new(Vec2::new(800.0, 600.0)),
            centered: Signal::new(true),
        }
    }
}

impl CameraNode {
    /// Creates a new camera containing the specified list of nodes.
    pub fn new(nodes: Vec<Box<dyn Node>>) -> Self {
        Self::default().with_nodes(nodes)
    }

    /// Sets the world-space position of the camera.
    pub fn with_position(mut self, pos: Vec2) -> Self {
        self.position = Signal::new(pos);
        self
    }

    /// Sets the rotation of the camera in radians.
    pub fn with_rotation(mut self, angle: f32) -> Self {
        self.rotation = Signal::new(angle);
        self
    }

    /// Sets the zoom level.
    pub fn with_zoom(mut self, zoom: f32) -> Self {
        self.zoom = Signal::new(zoom);
        self
    }

    /// Sets the relative transformation origin (anchor).
    pub fn with_anchor(mut self, anchor: Vec2) -> Self {
        self.anchor = Signal::new(anchor);
        self
    }

    /// Sets the viewport dimensions.
    pub fn with_size(mut self, size: Vec2) -> Self {
        self.size = Signal::new(size);
        self
    }

    /// Toggles world origin centering.
    pub fn with_centered(mut self, centered: bool) -> Self {
        self.centered = Signal::new(centered);
        self
    }

    /// Sets the child nodes.
    pub fn with_nodes(mut self, nodes: Vec<Box<dyn Node>>) -> Self {
        self.nodes = nodes;
        self
    }

    /// Adds a single child node to the camera.
    pub fn with_node(mut self, node: Box<dyn Node>) -> Self {
        self.nodes.push(node);
        self
    }

    /// Convenience method to add a static child node.
    pub fn with_child<N: Node + 'static>(mut self, node: N) -> Self {
        self.nodes.push(Box::new(node));
        self
    }

    /// Adds a node to an existing camera instance.
    pub fn add(&mut self, node: Box<dyn Node>) {
        self.nodes.push(node);
    }
}

impl Clone for CameraNode {
    fn clone(&self) -> Self {
        Self {
            nodes: self.nodes.iter().map(|n| n.clone_node()).collect(),
            position: self.position.clone(),
            rotation: self.rotation.clone(),
            zoom: self.zoom.clone(),
            anchor: self.anchor.clone(),
            opacity: Signal::new(1.0),
            size: self.size.clone(),
            centered: self.centered.clone(),
        }
    }
}

impl Node for CameraNode {
    #[cfg(feature = "runtime")]
    fn render(&self, scene: &mut Scene, parent_transform: Affine, parent_opacity: f32) {
        let opacity = self.opacity.get();
        let combined_opacity = parent_opacity * opacity;

        if combined_opacity <= 0.0 {
            return;
        }

        let pos = self.position.get();
        let rot = self.rotation.get();
        let zoom = self.zoom.get();
        let anchor = self.anchor.get();
        let size = self.size.get();
        let centered = self.centered.get();

        // The camera transform represents where the camera is in the world.
        // To render from the camera's perspective, we apply the INVERSE of its transform.

        // Offset for alignment (centering the camera)
        let viewport_center = if centered { size * 0.5 } else { Vec2::ZERO };
        let anchor_offset = anchor * size * 0.5;

        let view_transform =
            Affine::translate((viewport_center.x as f64, viewport_center.y as f64))
                * Affine::scale(zoom as f64)
                * Affine::rotate(-rot as f64)
                * Affine::translate((-pos.x as f64, -pos.y as f64))
                * Affine::translate((-anchor_offset.x as f64, -anchor_offset.y as f64));

        let combined_transform = parent_transform * view_transform;

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
        use crate::assets::hash::Hasher;
        let mut h = Hasher::new();
        h.update_u64(self.position.state_hash());
        h.update_u64(self.rotation.state_hash());
        h.update_u64(self.zoom.state_hash());
        h.update_u64(self.anchor.state_hash());
        h.update_u64(self.opacity.state_hash());
        h.update_u64(self.size.state_hash());
        h.update_u64(self.centered.state_hash());

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
        self.zoom.reset();
        self.anchor.reset();
        self.opacity.reset();
        self.size.reset();
        self.centered.reset();
        for node in &mut self.nodes {
            node.reset();
        }
    }
}
