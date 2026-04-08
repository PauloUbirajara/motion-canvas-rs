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
        let start = self.start.get();
        let end = self.end.get();
        let color = self.color.get();
        let width = self.width.get();
        let mut hash = 0u64;
        hash ^= start.x.to_bits() as u64;
        hash ^= start.y.to_bits() as u64;
        hash ^= end.x.to_bits() as u64;
        hash ^= end.y.to_bits() as u64;
        hash ^= width.to_bits() as u64;
        hash ^= (color.r as u64) << 24 | (color.g as u64) << 16 | (color.b as u64) << 8 | (color.a as u64);
        hash
    }
}
