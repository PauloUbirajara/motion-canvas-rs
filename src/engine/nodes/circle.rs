use crate::engine::animation::{Signal, Node};
use vello::peniko::Color;
use vello::Scene;
use glam::Vec2;
use vello::kurbo::Affine;
use std::time::Duration;

#[derive(Clone)]
pub struct Circle {
    pub transform: Signal<Affine>,
    pub radius: Signal<f32>,
    pub color: Color,
}

impl Circle {
    pub fn new(center: Vec2, radius: f32, color: Color) -> Self {
        Self {
            transform: Signal::new(Affine::translate((center.x as f64, center.y as f64))),
            radius: Signal::new(radius),
            color,
        }
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
}

impl Node for Circle {
    fn render(&self, scene: &mut Scene, parent_transform: Affine, parent_opacity: f32) {
        let local_transform = self.transform.get();
        let radius = self.radius.get() as f64;
        let combined_transform = parent_transform * local_transform;
        
        scene.fill(
            vello::peniko::Fill::NonZero,
            combined_transform,
            self.color.with_alpha_factor(parent_opacity),
            None,
            &vello::kurbo::Circle::new((0.0, 0.0), radius),
        );
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
        self.radius.get().to_bits().hash(&mut s);
        let color = self.color;
        color.r.hash(&mut s);
        color.g.hash(&mut s);
        color.b.hash(&mut s);
        color.a.hash(&mut s);
        s.finish()
    }

    fn clone_node(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
