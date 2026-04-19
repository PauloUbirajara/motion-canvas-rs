use crate::engine::animation::{Node, Signal};
use glam::Vec2;
use std::sync::Arc;
use std::time::Duration;
use vello::kurbo::Affine;
use vello::peniko::Image as PenikoImage;
use vello::Scene;

use crate::engine::util::image_manager::ImageManager;

#[derive(Clone)]
pub struct ImageNode {
    pub position: Signal<Vec2>,
    pub rotation: Signal<f32>,
    pub scale: Signal<Vec2>,
    pub size: Signal<Vec2>,
    pub image: Option<Arc<PenikoImage>>,
    pub opacity: Signal<f32>,
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
        }
    }
}

impl ImageNode {
    pub fn new(pos: Vec2, path: &str) -> Self {
        Self::default().with_position(pos).with_path(path)
    }

    pub fn with_position(mut self, position: Vec2) -> Self {
        self.position = Signal::new(position);
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

    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = Signal::new(opacity);
        self
    }

    pub fn with_size(mut self, size: Vec2) -> Self {
        self.size = Signal::new(size);
        self
    }

    pub fn with_path(mut self, path: &str) -> Self {
        self.image = ImageManager::get_image(path);
        if let Some(ref img) = self.image {
            self.size = Signal::new(Vec2::new(img.width as f32, img.height as f32));
        }
        self
    }
}

impl Node for ImageNode {
    fn render(&self, scene: &mut Scene, parent_transform: Affine, parent_opacity: f32) {
        let Some(ref img) = self.image else {
            return;
        };

        let size = self.size.get();
        let pos = self.position.get();
        let rot = self.rotation.get();
        let sc = self.scale.get();

        let local_transform = Affine::translate((pos.x as f64, pos.y as f64))
            * Affine::rotate(rot as f64)
            * Affine::scale_non_uniform(sc.x as f64, sc.y as f64);

        let opacity = self.opacity.get();
        let final_opacity = opacity * parent_opacity;

        if final_opacity <= 0.0 {
            return;
        }

        let transform = parent_transform
            * local_transform
            * Affine::scale_non_uniform(
                size.x as f64 / img.width as f64,
                size.y as f64 / img.height as f64,
            );

        if final_opacity < 1.0 {
            scene.push_layer(
                vello::peniko::Mix::Normal,
                final_opacity,
                transform,
                &vello::kurbo::Rect::new(0.0, 0.0, img.width as f64, img.height as f64),
            );
            scene.draw_image(img, Affine::IDENTITY);
            scene.pop_layer();
            return;
        }

        scene.draw_image(img, transform);
    }
    fn update(&mut self, _dt: Duration) {}
    fn state_hash(&self) -> u64 {
        self.position.state_hash()
            ^ self.rotation.state_hash()
            ^ self.scale.state_hash()
            ^ self.size.state_hash()
            ^ self.opacity.state_hash()
    }

    fn clone_node(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
