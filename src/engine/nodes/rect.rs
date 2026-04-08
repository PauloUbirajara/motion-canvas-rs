use crate::engine::animation::{Signal, Node};
use vello::peniko::{Brush, Color, Fill};
use vello::Scene;
use glam::Vec2;
use vello::kurbo::{Affine, RoundedRect as KurboRoundedRect};
use std::time::Duration;

#[derive(Clone)]
pub struct Rect {
    pub position: Signal<Vec2>,
    pub size: Signal<Vec2>,
    pub color: Signal<Color>,
    pub radius: Signal<f32>,
}

impl Rect {
    pub fn new(position: Vec2, size: Vec2, color: Color) -> Self {
        Self {
            position: Signal::new(position),
            size: Signal::new(size),
            color: Signal::new(color),
            radius: Signal::new(0.0),
        }
    }
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = Signal::new(radius);
        self
    }
}

impl Node for Rect {
    fn render(&self, scene: &mut Scene) {
        let pos = self.position.get();
        let size = self.size.get();
        let color = self.color.get();
        let radius = self.radius.get();
        let brush = Brush::Solid(color);
        
        scene.fill(
            Fill::NonZero,
            Affine::translate((pos.x as f64, pos.y as f64)),
            &brush,
            None,
            &KurboRoundedRect::new(0.0, 0.0, size.x as f64, size.y as f64, radius as f64),
        );
    }
    fn update(&mut self, _dt: Duration) {}
    fn state_hash(&self) -> u64 {
        let pos = self.position.get();
        let size = self.size.get();
        let color = self.color.get();
        let radius = self.radius.get();
        let mut hash = 0u64;
        hash ^= pos.x.to_bits() as u64;
        hash ^= pos.y.to_bits() as u64;
        hash ^= size.x.to_bits() as u64;
        hash ^= size.y.to_bits() as u64;
        hash ^= radius.to_bits() as u64;
        hash ^= (color.r as u64) << 24 | (color.g as u64) << 16 | (color.b as u64) << 8 | (color.a as u64);
        hash
    }
}
