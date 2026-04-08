use crate::engine::animation::{Signal, Node};
use vello::peniko::{Color, Stroke};
use vello::Scene;
use glam::Vec2;
use vello::kurbo::{Affine, Line as VelloLine};
use std::time::Duration;

#[derive(Clone)]
pub struct Line {
    pub transform: Signal<Affine>,
    pub start: Signal<Vec2>,
    pub end: Signal<Vec2>,
    pub stroke_width: f32,
    pub color: Color,
}

impl Line {
    pub fn new(start: Vec2, end: Vec2, stroke_width: f32, color: Color) -> Self {
        Self {
            transform: Signal::new(Affine::IDENTITY),
            start: Signal::new(start),
            end: Signal::new(end),
            stroke_width,
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

impl Node for Line {
    fn render(&self, scene: &mut Scene, parent_transform: Affine, parent_opacity: f32) {
        let local_transform = self.transform.get();
        let start = self.start.get();
        let end = self.end.get();
        let combined_transform = parent_transform * local_transform;
        
        let line = VelloLine::new((start.x as f64, start.y as f64), (end.x as f64, end.y as f64));
        scene.stroke(
            &Stroke::new(self.stroke_width as f32),
            combined_transform,
            self.color.with_alpha_factor(parent_opacity),
            None,
            &line,
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
        self.start.get().x.to_bits().hash(&mut s);
        self.start.get().y.to_bits().hash(&mut s);
        self.end.get().x.to_bits().hash(&mut s);
        self.end.get().y.to_bits().hash(&mut s);
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
