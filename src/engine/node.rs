use glam::Vec2;
use vello::peniko::{Brush, Color, Fill};
use vello::Scene;
use std::time::Duration;
use crate::engine::animation::{Signal, Node};

pub struct Circle {
    pub position: Signal<Vec2>,
    pub radius: Signal<f32>,
    pub fill: Color,
}

impl Node for Circle {
    fn render(&self, scene: &mut Scene) {
        let brush = Brush::Solid(self.fill);
        let pos = self.position.data.lock().unwrap().value;
        let radius = self.radius.data.lock().unwrap().value;
        
        scene.fill(
            Fill::NonZero,
            vello::kurbo::Affine::translate((pos.x as f64, pos.y as f64)),
            &brush,
            None,
            &vello::kurbo::Circle::new((0.0, 0.0), radius as f64),
        );
    }

    fn update(&mut self, _dt: Duration) {}

    fn state_hash(&self) -> u64 {
        let pos = self.position.data.lock().unwrap().value;
        let radius = self.radius.data.lock().unwrap().value;
        let mut hash = 0u64;
        hash ^= pos.x.to_bits() as u64;
        hash ^= pos.y.to_bits() as u64;
        hash ^= radius.to_bits() as u64;
        hash ^= self.fill.r as u64;
        hash ^= (self.fill.g as u64) << 8;
        hash ^= (self.fill.b as u64) << 16;
        hash ^= (self.fill.a as u64) << 24;
        hash
    }
}

pub struct Rect {
    pub position: Signal<Vec2>,
    pub size: Signal<Vec2>,
    pub fill: Color,
    pub radius: f32, // border radius
}

impl Node for Rect {
    fn render(&self, scene: &mut Scene) {
        let brush = Brush::Solid(self.fill);
        let pos = self.position.data.lock().unwrap().value;
        let size = self.size.data.lock().unwrap().value;
        
        scene.fill(
            Fill::NonZero,
            vello::kurbo::Affine::translate((pos.x as f64, pos.y as f64)),
            &brush,
            None,
            &vello::kurbo::RoundedRect::new(0.0, 0.0, size.x as f64, size.y as f64, self.radius as f64),
        );
    }

    fn update(&mut self, _dt: Duration) {}

    fn state_hash(&self) -> u64 {
        let pos = self.position.data.lock().unwrap().value;
        let size = self.size.data.lock().unwrap().value;
        let mut hash = 0u64;
        hash ^= pos.x.to_bits() as u64;
        hash ^= pos.y.to_bits() as u64;
        hash ^= size.x.to_bits() as u64;
        hash ^= size.y.to_bits() as u64;
        hash ^= self.radius.to_bits() as u64;
        hash ^= self.fill.r as u64;
        hash ^= (self.fill.g as u64) << 8;
        hash ^= (self.fill.b as u64) << 16;
        hash ^= (self.fill.a as u64) << 24;
        hash
    }
}

pub struct Line {
    pub start: Signal<Vec2>,
    pub end: Signal<Vec2>,
    pub stroke: Color,
    pub thickness: f32,
}

impl Node for Line {
    fn render(&self, scene: &mut Scene) {
        let brush = Brush::Solid(self.stroke);
        let start = self.start.data.lock().unwrap().value;
        let end = self.end.data.lock().unwrap().value;
        
        scene.stroke(
            &vello::kurbo::Stroke::new(self.thickness as f64),
            vello::kurbo::Affine::IDENTITY,
            &brush,
            None,
            &vello::kurbo::Line::new((start.x as f64, start.y as f64), (end.x as f64, end.y as f64)),
        );
    }

    fn update(&mut self, _dt: Duration) {}

    fn state_hash(&self) -> u64 {
        let start = self.start.data.lock().unwrap().value;
        let end = self.end.data.lock().unwrap().value;
        let mut hash = 0u64;
        hash ^= start.x.to_bits() as u64;
        hash ^= start.y.to_bits() as u64;
        hash ^= end.x.to_bits() as u64;
        hash ^= end.y.to_bits() as u64;
        hash ^= self.thickness.to_bits() as u64;
        hash ^= self.stroke.r as u64;
        hash ^= (self.stroke.g as u64) << 8;
        hash ^= (self.stroke.b as u64) << 16;
        hash ^= (self.stroke.a as u64) << 24;
        hash
    }
}
