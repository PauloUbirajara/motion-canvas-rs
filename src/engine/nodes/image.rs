use crate::engine::animation::{Signal, Node};
use vello::Scene;
use glam::Vec2;
use vello::kurbo::Affine;
use std::time::Duration;
use vello::peniko::Image;

#[derive(Clone)]
pub struct ImageNode {
    pub transform: Signal<Affine>,
    pub opacity: Signal<f32>,
    pub image: Image,
    pub size: Vec2,
}

impl ImageNode {
    pub fn new(pos: Vec2, image: Image) -> Self {
        let size = Vec2::new(image.width as f32, image.height as f32);
        Self {
            transform: Signal::new(Affine::translate((pos.x as f64, pos.y as f64))),
            opacity: Signal::new(1.0),
            image,
            size,
        }
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.size = Vec2::new(width, height);
        self
    }

    pub fn with_transform(mut self, transform: Affine) -> Self {
        self.transform = Signal::new(transform);
        self
    }

    pub fn with_position(mut self, pos: Vec2) -> Self {
        self.transform = Signal::new(Affine::translate((pos.x as f64, pos.y as f64)));
        self
    }

    pub fn with_rotation(mut self, rad: f32) -> Self {
        self.transform = Signal::new(self.transform.get() * Affine::rotate(rad as f64));
        self
    }

    pub fn with_scale(mut self, s: f32) -> Self {
        self.transform = Signal::new(self.transform.get() * Affine::scale(s as f64));
        self
    }

    pub fn with_opacity(mut self, a: f32) -> Self {
        self.opacity = Signal::new(a);
        self
    }
}

impl Node for ImageNode {
    fn render(&self, scene: &mut Scene, parent_transform: Affine, parent_opacity: f32) {
        let local_transform = self.transform.get();
        let opacity = self.opacity.get() * parent_opacity;
        
        let combined_transform = parent_transform * local_transform;
        
        if opacity <= 0.0 {
            return;
        }

        // Apply scale based on original image size vs requested size
        let sx = self.size.x as f64 / self.image.width as f64;
        let sy = self.size.y as f64 / self.image.height as f64;
        let scale = Affine::scale_non_uniform(sx, sy);
        
        scene.push_layer(
            vello::peniko::Mix::Normal,
            opacity,
            combined_transform,
            &vello::kurbo::Rect::new(0.0, 0.0, self.image.width as f64, self.image.height as f64)
        );
        scene.draw_image(&self.image, scale);
        scene.pop_layer();
    }

    fn update(&mut self, _dt: Duration) {}

    fn state_hash(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;
        let mut s = DefaultHasher::new();
        
        let coeffs = self.transform.get().as_coeffs();
        for c in coeffs {
            c.to_bits().hash(&mut s);
        }
        self.opacity.get().to_bits().hash(&mut s);
        self.size.x.to_bits().hash(&mut s);
        self.size.y.to_bits().hash(&mut s);
        
        s.finish()
    }

    fn clone_node(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
