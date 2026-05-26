#![cfg(feature = "image")]
use crate::core::animation::{Node, Signal};
use glam::Vec2;
use kurbo::Affine;
use peniko::Image as PenikoImage;
use std::sync::Arc;
use std::time::Duration;
#[cfg(feature = "runtime")]
use vello::Scene;

use crate::assets::image_manager::ImageManager;

/// A visual node that renders a bitmap image from a file path.
///
/// `ImageNode` automatically loads and caches images using the global `ImageManager`.
/// It supports standard transformations like position, scale, and rotation.
///
/// ### Example
/// ```rust
/// # use motion_canvas_rs::prelude::*;
/// let logo = ImageNode::default()
///     .with_position(Vec2::new(640.0, 360.0))
///     .with_path("assets/logo.png")
///     .with_scale(0.5);
/// ```
#[derive(Clone)]
pub struct ImageNode {
    /// The absolute position of the image's center (before anchor adjustment).
    pub position: Signal<Vec2>,
    /// Rotation in radians.
    pub rotation: Signal<f32>,
    /// Scaling factor for the image.
    pub scale: Signal<Vec2>,
    /// The target display size of the image.
    pub size: Signal<Vec2>,
    /// The cached raw image data from Vello/Peniko.
    pub image: Option<Arc<PenikoImage>>,
    /// Opacity from 0.0 (transparent) to 1.0 (opaque).
    pub opacity: Signal<f32>,
    /// The relative transformation origin. (-1,-1) is top-left, (0,0) is center, (1,1) is bottom-right.
    pub anchor: Signal<Vec2>,
    /// The source filesystem path for the image.
    pub path: String,
    /// Blur radius signal.
    pub blur: Signal<f32>,
}

impl Default for ImageNode {
    fn default() -> Self {
        Self {
            position: Signal::new(Vec2::ZERO),
            rotation: Signal::new(0.0),
            scale: Signal::new(Vec2::ONE),
            size: Signal::new(Vec2::ZERO),
            image: None,
            opacity: Signal::new(1.0),
            anchor: Signal::new(Vec2::ZERO),
            path: String::new(),
            blur: Signal::new(crate::core::filters::DEFAULT_BLUR),
        }
    }
}

impl ImageNode {
    /// Creates a new image node at the given position from a file path.
    pub fn new(pos: Vec2, path: &str) -> Self {
        Self::default().with_position(pos).with_path(path)
    }

    /// Sets the absolute position of the image.
    pub fn with_position(mut self, position: Vec2) -> Self {
        self.position = Signal::new(position);
        self
    }

    /// Sets the rotation of the image in radians.
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

    /// Sets the opacity of the image (0.0 to 1.0).
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = Signal::new(opacity);
        self
    }

    /// Manually overrides the display size of the image.
    pub fn with_size(mut self, size: Vec2) -> Self {
        self.size = Signal::new(size);
        self
    }

    /// Sets the source path and triggers image loading.
    pub fn with_path(mut self, path: &str) -> Self {
        self.path = path.to_string();
        self.image = ImageManager::get_image(path);
        if let Some(ref img) = self.image {
            self.size = Signal::new(Vec2::new(img.width as f32, img.height as f32));
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

impl crate::core::filters::Blur for ImageNode {
    fn with_blur(mut self, radius: f32) -> Self {
        self.blur = Signal::new(radius);
        self
    }
}

impl Node for ImageNode {
    #[cfg(feature = "runtime")]
    fn render(&self, scene: &mut Scene, parent_transform: Affine, parent_opacity: f32) {
        let Some(ref img) = self.image else {
            return;
        };

        let blur_radius = self.blur.get().max(0.0);
        let opacity = self.opacity.get();
        let combined_opacity = parent_opacity * opacity;

        crate::core::filters::apply_blur_filter(
            scene,
            blur_radius,
            combined_opacity,
            |scene, target_opacity| {
                let size = self.size.get();
                let pos = self.position.get();
                let rot = self.rotation.get();
                let sc = self.scale.get();
                let anchor = self.anchor.get();

                let anchor_offset = anchor * size * 0.5;

                let local_transform = Affine::translate((pos.x as f64, pos.y as f64))
                    * Affine::rotate(rot as f64)
                    * Affine::scale_non_uniform(sc.x as f64, sc.y as f64)
                    * Affine::translate((-anchor_offset.x as f64, -anchor_offset.y as f64));

                let transform = parent_transform
                    * local_transform
                    * Affine::translate((-size.x as f64 / 2.0, -size.y as f64 / 2.0))
                    * Affine::scale_non_uniform(
                        size.x as f64 / img.width as f64,
                        size.y as f64 / img.height as f64,
                    );

                if target_opacity < 1.0 {
                    // Use Identity transform for the layer to avoid coordinate system confusion with clip rect
                    scene.push_layer(
                        peniko::Mix::Normal,
                        target_opacity,
                        Affine::IDENTITY,
                        &kurbo::Rect::new(-10000.0, -10000.0, 10000.0, 10000.0),
                    );
                    scene.draw_image(img, transform);
                    scene.pop_layer();
                    return;
                }

                scene.draw_image(img, transform);
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
