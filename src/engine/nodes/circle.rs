use crate::engine::animation::{Signal, Node};
use vello::peniko::{Brush, Color, Fill};
use vello::Scene;
use glam::Vec2;
use vello::kurbo::{Affine, Circle as KurboCircle};
use std::time::Duration;

#[derive(Clone)]
pub struct Circle {
    pub position: Signal<Vec2>,
    pub radius: Signal<f32>,
    pub color: Signal<Color>,
}

impl Circle {
    pub fn new(position: Vec2, radius: f32, color: Color) -> Self {
        Self {
            position: Signal::new(position),
            radius: Signal::new(radius),
            color: Signal::new(color),
        }
    }
}

impl Node for Circle {
    fn render(&self, scene: &mut Scene) {
        let pos = self.position.get();
        let radius = self.radius.get();
        let color = self.color.get();
        let brush = Brush::Solid(color);
        
        scene.fill(
            Fill::NonZero,
            Affine::translate((pos.x as f64, pos.y as f64)),
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
        self.position.get().x.to_bits().hash(&mut s);
        self.position.get().y.to_bits().hash(&mut s);
        self.radius.get().to_bits().hash(&mut s);
        let color = self.color.get();
        color.r.hash(&mut s);
        color.g.hash(&mut s);
        color.b.hash(&mut s);
        color.a.hash(&mut s);
        s.finish()
    }
}
