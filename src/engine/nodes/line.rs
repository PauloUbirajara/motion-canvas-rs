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
    pub stroke: Color,
    pub thickness: f32,
}

impl Line {
    pub fn new(start: Vec2, end: Vec2, stroke: Color, thickness: f32) -> Self {
        Self {
            start: Signal::new(start),
            end: Signal::new(end),
            stroke,
            thickness,
        }
    }
}

impl Node for Line {
    fn render(&self, scene: &mut Scene) {
        let brush = Brush::Solid(self.stroke);
        let start = self.start.data.lock().unwrap().value.clone();
        let end = self.end.data.lock().unwrap().value.clone();
        
        scene.stroke(
            &Stroke::new(self.thickness as f64),
            Affine::IDENTITY,
            &brush,
            None,
            &KurboLine::new((start.x as f64, start.y as f64), (end.x as f64, end.y as f64)),
        );
    }
    fn update(&mut self, _dt: Duration) {}
    fn state_hash(&self) -> u64 {
        let start = self.start.data.lock().unwrap().value;
        let end = self.end.data.lock().unwrap().value;
        let mut hash = 0u64;
        hash ^= start.x.to_bits() as u64;
        hash ^= end.x.to_bits() as u64;
        hash
    }
}
