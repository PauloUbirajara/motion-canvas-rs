use crate::engine::animation::{Node, Signal};
use glam::Vec2;
use std::time::Duration;
use vello::kurbo::{Affine, Line as KurboLine, Stroke};
use vello::peniko::{Brush, Color};
use vello::Scene;

#[derive(Clone)]
pub struct Line {
    pub transform: Signal<Affine>,
    pub start: Signal<Vec2>,
    pub end: Signal<Vec2>,
    pub color: Signal<Color>,
    pub width: Signal<f32>,
    pub opacity: Signal<f32>,
}

impl Line {
    pub fn new(start: Vec2, end: Vec2, color: Color, width: f32) -> Self {
        Self {
            transform: Signal::new(Affine::IDENTITY),
            start: Signal::new(start),
            end: Signal::new(end),
            color: Signal::new(color),
            width: Signal::new(width),
            opacity: Signal::new(1.0),
        }
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
}

impl Node for Line {
    fn render(&self, scene: &mut Scene, parent_transform: Affine, parent_opacity: f32) {
        let start = self.start.get();
        let end = self.end.get();
        let color = self.color.get();
        let width = self.width.get();
        let local_transform = self.transform.get();
        let opacity = self.opacity.get();

        let combined_transform = parent_transform * local_transform;
        let combined_opacity = parent_opacity * opacity;

        let mut final_color = color;
        final_color.a = (color.a as f32 * combined_opacity).clamp(0.0, 255.0) as u8;

        let brush = Brush::Solid(final_color);

        scene.stroke(
            &Stroke::new(width as f64),
            combined_transform,
            &brush,
            None,
            &KurboLine::new(
                (start.x as f64, start.y as f64),
                (end.x as f64, end.y as f64),
            ),
        );
    }
    fn update(&mut self, _dt: Duration) {}
    fn state_hash(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut s = DefaultHasher::new();

        let coeffs = self.transform.get().as_coeffs();
        for c in coeffs {
            c.to_bits().hash(&mut s);
        }

        self.start.get().x.to_bits().hash(&mut s);
        self.start.get().y.to_bits().hash(&mut s);
        self.end.get().x.to_bits().hash(&mut s);
        self.end.get().y.to_bits().hash(&mut s);
        self.width.get().to_bits().hash(&mut s);
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
