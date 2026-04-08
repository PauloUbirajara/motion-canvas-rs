use crate::engine::animation::{Signal, Node};
use vello::peniko::{Brush, Color, Fill};
use vello::Scene;
use glam::Vec2;
use vello::kurbo::{Affine, Circle as KurboCircle};
use std::time::Duration;

const DEFAULT_RADIUS: f32 = 50.0;
const DEFAULT_COLOR: Color = Color::RED;
const DEFAULT_OPACITY: f32 = 1.0;

#[derive(Clone)]
pub struct Circle {
    pub transform: Signal<Affine>,
    pub radius: Signal<f32>,
    pub color: Signal<Color>,
    pub opacity: Signal<f32>,
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            transform: Signal::new(Affine::IDENTITY),
            radius: Signal::new(DEFAULT_RADIUS),
            color: Signal::new(DEFAULT_COLOR),
            opacity: Signal::new(DEFAULT_OPACITY),
        }
    }
}

impl Circle {
    pub fn new(position: Vec2, radius: f32, color: Color) -> Self {
        Self::default()
            .with_position(position)
            .with_radius(radius)
            .with_color(color)
    }

    pub fn with_transform(mut self, transform: Affine) -> Self {
        self.transform = Signal::new(transform);
        self
    }

    pub fn with_position(mut self, position: Vec2) -> Self {
        self.transform = Signal::new(Affine::translate((position.x as f64, position.y as f64)));
        self
    }

    pub fn with_rotation(mut self, angle: f32) -> Self {
        let current = self.transform.get();
        let coeffs = current.as_coeffs();
        // Extract translation from current transform
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

    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = Signal::new(opacity);
        self
    }

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = Signal::new(radius);
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Signal::new(color);
        self
    }
}

impl Node for Circle {
    fn render(&self, scene: &mut Scene, parent_transform: Affine, parent_opacity: f32) {
        let local_transform = self.transform.get();
        let radius = self.radius.get();
        let color = self.color.get();
        let opacity = self.opacity.get();
        
        // Combine transforms and opacities
        let combined_transform = parent_transform * local_transform;
        let combined_opacity = parent_opacity * opacity;
        
        let mut final_color = color;
        final_color.a = (color.a as f32 * combined_opacity).clamp(0.0, 255.0) as u8;
        
        let brush = Brush::Solid(final_color);
        
        scene.fill(
            Fill::NonZero,
            combined_transform,
            &brush,
            None,
            &KurboCircle::new((0.0, 0.0), radius as f64),
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
        let color = self.color.get();
        color.r.hash(&mut s);
        color.g.hash(&mut s);
        color.b.hash(&mut s);
        self.opacity.get().to_bits().hash(&mut s);
        s.finish()
    }

    fn clone_node(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
