use crate::core::animation::{Node, Signal};
use crate::core::masks::{apply_mask, MaskMode};
use glam::Vec2;
use kurbo::Affine;
use std::time::Duration;

#[cfg(feature = "runtime")]
use vello::Scene;

/// A visual node that masks/clips a source node using a mask node.
///
/// `MaskNode` supports standard stencil masking and Boolean operations:
/// * `MaskMode::Intersect` (or `Alpha`): Renders only where the shapes overlap.
/// * `MaskMode::Subtract`: Subtracts the mask shape out of the source shape.
/// * `MaskMode::Exclude` (or `Xor` / `Difference`): Keeps non-overlapping regions.
/// * `MaskMode::Union`: Combines both shapes to make both fully visible.
pub struct MaskNode {
    /// The mask node defining the visible region/alpha.
    pub mask: Box<dyn Node>,
    /// The source node to be masked/clipped.
    pub source: Box<dyn Node>,
    /// The composition masking mode.
    pub mode: Signal<MaskMode>,
    /// The position of the mask container.
    pub position: Signal<Vec2>,
    /// Rotation in radians.
    pub rotation: Signal<f32>,
    /// Scaling factors.
    pub scale: Signal<Vec2>,
    /// Opacity factor.
    pub opacity: Signal<f32>,
    /// The nominal size for anchor computations.
    pub size: Signal<Vec2>,
    /// The transform anchor.
    pub anchor: Signal<Vec2>,
}

impl MaskNode {
    /// Creates a new `MaskNode` with standard defaults.
    pub fn new(mask: Box<dyn Node>, source: Box<dyn Node>) -> Self {
        Self {
            mask,
            source,
            mode: Signal::new(MaskMode::Alpha),
            position: Signal::new(Vec2::ZERO),
            rotation: Signal::new(0.0),
            scale: Signal::new(Vec2::ONE),
            opacity: Signal::new(1.0),
            size: Signal::new(Vec2::ZERO),
            anchor: Signal::new(Vec2::ZERO),
        }
    }

    /// Sets the position of the mask container.
    pub fn with_position(mut self, pos: Vec2) -> Self {
        self.position = Signal::new(pos);
        self
    }

    /// Sets the rotation of the mask container in radians.
    pub fn with_rotation(mut self, angle: f32) -> Self {
        self.rotation = Signal::new(angle);
        self
    }

    /// Sets the uniform scale factor.
    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = Signal::new(Vec2::splat(scale));
        self
    }

    /// Sets non-uniform scaling factors.
    pub fn with_scale_xy(mut self, scale: Vec2) -> Self {
        self.scale = Signal::new(scale);
        self
    }

    /// Sets the opacity of the mask container (0.0 to 1.0).
    pub fn with_opacity(mut self, a: f32) -> Self {
        self.opacity = Signal::new(a);
        self
    }

    /// Sets the size of the mask container for anchor computations.
    pub fn with_size(mut self, size: Vec2) -> Self {
        self.size = Signal::new(size);
        self
    }

    /// Sets the transform anchor origin.
    pub fn with_anchor(mut self, anchor: Vec2) -> Self {
        self.anchor = Signal::new(anchor);
        self
    }

    /// Sets the composition masking mode.
    pub fn with_mode(mut self, mode: MaskMode) -> Self {
        self.mode = Signal::new(mode);
        self
    }
}

impl Clone for MaskNode {
    fn clone(&self) -> Self {
        Self {
            mask: self.mask.clone_node(),
            source: self.source.clone_node(),
            mode: self.mode.clone(),
            position: self.position.clone(),
            rotation: self.rotation.clone(),
            scale: self.scale.clone(),
            opacity: self.opacity.clone(),
            size: self.size.clone(),
            anchor: self.anchor.clone(),
        }
    }
}

impl Node for MaskNode {
    #[cfg(feature = "runtime")]
    fn render(&self, scene: &mut Scene, parent_transform: Affine, parent_opacity: f32) {
        let opacity = self.opacity.get();
        let combined_opacity = parent_opacity * opacity;
        if combined_opacity <= 0.0 {
            return;
        }

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

        let mode = self.mode.get();

        apply_mask(
            scene,
            mode,
            combined_opacity,
            combined_transform,
            |s, t| self.mask.render(s, t, 1.0),
            |s, t| self.source.render(s, t, 1.0),
        );
    }

    fn update(&mut self, dt: Duration) {
        self.mask.update(dt);
        self.source.update(dt);
    }

    fn state_hash(&self) -> u64 {
        use crate::assets::hash::Hasher;
        let mut h = Hasher::new();
        h.update_u64(self.position.state_hash());
        h.update_u64(self.rotation.state_hash());
        h.update_u64(self.scale.state_hash());
        h.update_u64(self.opacity.state_hash());
        h.update_u64(self.size.state_hash());
        h.update_u64(self.anchor.state_hash());
        h.update_u64(self.mode.state_hash());
        h.update_u64(self.mask.state_hash());
        h.update_u64(self.source.state_hash());
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
        self.mode.reset();
        self.mask.reset();
        self.source.reset();
    }
}
