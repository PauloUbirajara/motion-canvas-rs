#![cfg(feature = "svg")]
use crate::core::animation::{Node, Signal};
use glam::Vec2;
use kurbo::Affine;
use std::sync::Arc;
use std::time::Duration;
use usvg::Tree;
#[cfg(feature = "runtime")]
use vello::Scene;

use crate::assets::svg_manager::SvgManager;

/// A visual node that renders an SVG vector image from a file path.
///
/// `SvgNode` uses `vello_svg` to render SVGs directly as vector graphics,
/// maintaining sharpness at any scale.
#[derive(Clone)]
pub struct SvgNode {
    /// The absolute position of the SVG's center (before anchor adjustment).
    pub position: Signal<Vec2>,
    /// Rotation in radians.
    pub rotation: Signal<f32>,
    /// Scaling factor for the SVG.
    pub scale: Signal<Vec2>,
    /// The target display size of the SVG.
    pub size: Signal<Vec2>,
    /// The parsed SVG tree.
    pub tree: Option<Arc<Tree>>,
    /// The pre-rendered Vello scene.
    pub scene: Option<Arc<Scene>>,
    /// Opacity from 0.0 (transparent) to 1.0 (opaque).
    pub opacity: Signal<f32>,
    /// The relative transformation origin. (-1,-1) is top-left, (0,0) is center, (1,1) is bottom-right.
    pub anchor: Signal<Vec2>,
    /// The source filesystem path for the SVG.
    pub path: String,
    /// Blur radius signal.
    pub blur: Signal<f32>,
}

impl Default for SvgNode {
    fn default() -> Self {
        Self {
            position: Signal::new(Vec2::ZERO),
            rotation: Signal::new(0.0),
            scale: Signal::new(Vec2::ONE),
            size: Signal::new(Vec2::ZERO),
            tree: None,
            scene: None,
            opacity: Signal::new(1.0),
            anchor: Signal::new(Vec2::ZERO),
            path: String::new(),
            blur: Signal::new(crate::core::filters::DEFAULT_BLUR),
        }
    }
}

impl SvgNode {
    /// Creates a new SVG node at the given position from a file path.
    pub fn new(pos: Vec2, path: &str) -> Self {
        Self::default().with_position(pos).with_path(path)
    }

    /// Sets the absolute position of the SVG.
    pub fn with_position(mut self, position: Vec2) -> Self {
        self.position = Signal::new(position);
        self
    }

    /// Sets the rotation of the SVG in radians.
    pub fn with_rotation(mut self, angle: f32) -> Self {
        self.rotation = Signal::new(angle);
        self
    }

    /// Sets a uniform scale factor for both axes.
    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = Signal::new(Vec2::splat(scale));
        self
    }

    /// Sets non-uniform scaling factors for X and Y axes.
    pub fn with_scale_xy(mut self, scale: Vec2) -> Self {
        self.scale = Signal::new(scale);
        self
    }

    /// Sets the opacity of the SVG (0.0 to 1.0).
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = Signal::new(opacity);
        self
    }

    /// Manually overrides the display size of the SVG.
    pub fn with_size(mut self, size: Vec2) -> Self {
        self.size = Signal::new(size);
        self
    }

    /// Sets the source path and triggers SVG loading.
    pub fn with_path(mut self, path: &str) -> Self {
        self.path = path.to_string();
        if let Some((tree, scene)) = SvgManager::get_svg(path) {
            let size = tree.size();
            self.size = Signal::new(Vec2::new(size.width() as f32, size.height() as f32));
            self.tree = Some(tree);
            self.scene = Some(scene);
        }
        self
    }

    /// Sets the relative transformation origin (anchor).
    /// (-1, -1) is top-left, (0, 0) is center, (1, 1) is bottom-right.
    pub fn with_anchor(mut self, anchor: Vec2) -> Self {
        self.anchor = Signal::new(anchor);
        self
    }
}

impl crate::core::filters::Blur for SvgNode {
    fn with_blur(mut self, radius: f32) -> Self {
        self.blur = Signal::new(radius);
        self
    }
}

impl Node for SvgNode {
    #[cfg(feature = "runtime")]
    fn render(&self, scene: &mut Scene, parent_transform: Affine, parent_opacity: f32) {
        let blur_radius = self.blur.get().max(0.0);
        let opacity = self.opacity.get();
        let combined_opacity = parent_opacity * opacity;

        crate::core::filters::apply_blur_filter(
            scene,
            blur_radius,
            combined_opacity,
            |scene, target_opacity| {
                let (Some(ref tree), Some(ref svg_scene)) = (&self.tree, &self.scene) else {
                    return;
                };

                let size = self.size.get();
                let pos = self.position.get();
                let rot = self.rotation.get();
                let sc = self.scale.get();
                let anchor = self.anchor.get();

                // Use the actual content bounding box for precise centering
                let bbox = tree.root().bounding_box();
                let svg_w = bbox.width();
                let svg_h = bbox.height();

                // (anchor + 1.0) * size / 2.0 maps [-1, 1] to [0, size]
                let anchor_offset = (anchor + Vec2::new(1.0, 1.0)) * size * 0.5;

                let local_transform = Affine::translate((pos.x as f64, pos.y as f64))
                    * Affine::rotate(rot as f64)
                    * Affine::scale_non_uniform(sc.x as f64, sc.y as f64)
                    * Affine::translate((-anchor_offset.x as f64, -anchor_offset.y as f64));

                // Final transform: apply scaling to fit the content bbox into the target size
                let transform = parent_transform
                    * local_transform
                    * Affine::scale_non_uniform(
                        size.x as f64 / svg_w as f64,
                        size.y as f64 / svg_h as f64,
                    )
                    * Affine::translate((-bbox.left() as f64, -bbox.top() as f64));

                // Always use push_layer for consistent opacity application across all frames
                // Use IDENTITY for the layer and apply transform to the scene append to avoid vello layer transform issues
                scene.push_layer(
                    peniko::Mix::Normal,
                    target_opacity,
                    Affine::IDENTITY,
                    &kurbo::Rect::new(-1e10, -1e10, 1e10, 1e10),
                );
                scene.append(svg_scene, Some(transform));
                scene.pop_layer();
            },
        );
    }
    fn update(&mut self, _dt: Duration) {}
    fn state_hash(&self) -> u64 {
        use crate::assets::hash::Hasher;
        let mut h = Hasher::new();
        h.update_bytes(self.path.as_bytes());
        h.update_u64(self.position.state_hash());
        h.update_u64(self.rotation.state_hash());
        h.update_u64(self.scale.state_hash());
        h.update_u64(self.size.state_hash());
        h.update_u64(self.opacity.state_hash());
        h.update_u64(self.anchor.state_hash());
        h.update_u64(self.blur.state_hash());
        h.finish()
    }

    fn clone_node(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn reset(&mut self) {
        self.position.reset();
        self.rotation.reset();
        self.scale.reset();
        self.size.reset();
        self.opacity.reset();
        self.anchor.reset();
        self.blur.reset();
    }
}
