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
    pub fill: Color,
    pub radius: f32,
}

impl Rect {
    pub fn new(position: Vec2, size: Vec2, fill: Color) -> Self {
        Self {
            position: Signal::new(position),
            size: Signal::new(size),
            fill,
            radius: 0.0,
        }
    }
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }
}

impl Node for Rect {
    fn render(&self, scene: &mut Scene) {
        let brush = Brush::Solid(self.fill);
        let pos = self.position.data.lock().unwrap().value.clone();
        let size = self.size.data.lock().unwrap().value.clone();
        
        scene.fill(
            Fill::NonZero,
            Affine::translate((pos.x as f64, pos.y as f64)),
            &brush,
            None,
            &KurboRoundedRect::new(0.0, 0.0, size.x as f64, size.y as f64, self.radius as f64),
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
        hash
    }
}
