use crate::engine::animation::{Signal, Node};
use vello::peniko::{Brush, Color};
use vello::Scene;
use glam::Vec2;
use vello::kurbo::{Affine, Stroke, Line as KurboLine};
use std::time::Duration;

#[derive(Clone)]
pub struct Line {
    pub start: Signal<Vec2>,
    pub end: Signal<Vec2>,
    pub color: Signal<Color>,
    pub width: Signal<f32>,
}

impl Line {
    pub fn new(start: Vec2, end: Vec2, color: Color, width: f32) -> Self {
        Self {
            start: Signal::new(start),
            end: Signal::new(end),
            color: Signal::new(color),
            width: Signal::new(width),
        }
    }
}

impl Node for Line {
    fn render(&self, scene: &mut Scene) {
        let start = self.start.get();
        let end = self.end.get();
        let color = self.color.get();
        let width = self.width.get();
        let brush = Brush::Solid(color);
        
        scene.stroke(
            &Stroke::new(width as f64),
            Affine::IDENTITY,
            &brush,
            None,
            &KurboLine::new((start.x as f64, start.y as f64), (end.x as f64, end.y as f64)),
        );
    }
    fn update(&mut self, _dt: Duration) {}
    fn state_hash(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;
        let mut s = DefaultHasher::new();
        self.start.get().x.to_bits().hash(&mut s);
        self.start.get().y.to_bits().hash(&mut s);
        self.end.get().x.to_bits().hash(&mut s);
        self.end.get().y.to_bits().hash(&mut s);
        self.width.get().to_bits().hash(&mut s);
        let color = self.color.get();
        color.r.hash(&mut s);
        color.g.hash(&mut s);
        color.b.hash(&mut s);
        color.a.hash(&mut s);
        s.finish()
    }
}
