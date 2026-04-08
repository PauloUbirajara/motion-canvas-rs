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
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;
        let mut s = DefaultHasher::new();
        self.position.get().x.to_bits().hash(&mut s);
        self.position.get().y.to_bits().hash(&mut s);
        self.size.get().x.to_bits().hash(&mut s);
        self.size.get().y.to_bits().hash(&mut s);
        self.radius.get().to_bits().hash(&mut s);
        let color = self.color.get();
        color.r.hash(&mut s);
        color.g.hash(&mut s);
        color.b.hash(&mut s);
        color.a.hash(&mut s);
        s.finish()
    }
}
