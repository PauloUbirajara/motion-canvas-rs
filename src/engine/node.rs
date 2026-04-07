use glam::Vec2;
use std::time::Duration;
use vello::peniko::{Brush, Color, Fill};
use vello::Scene;

pub struct Signal<T> {
    pub value: T,
    pub target: T,
    pub duration: Duration,
    pub elapsed: Duration,
}

impl<T: Copy> Signal<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            target: value,
            duration: Duration::ZERO,
            elapsed: Duration::ZERO,
        }
    }

    pub fn to(&mut self, target: T, duration: Duration) {
        self.target = target;
        self.duration = duration;
        self.elapsed = Duration::ZERO;
    }
}

pub trait Node {
    fn render(&self, scene: &mut Scene);
    fn update(&mut self, dt: Duration);
}

pub struct Circle {
    pub position: Signal<Vec2>,
    pub radius: Signal<f32>,
    pub fill: Color,
}

impl Node for Circle {
    fn render(&self, scene: &mut Scene) {
        let brush = Brush::Solid(self.fill);
        scene.fill(
            Fill::NonZero,
            vello::kurbo::Affine::translate((
                self.position.value.x as f64,
                self.position.value.y as f64,
            )),
            &brush,
            None,
            &vello::kurbo::Circle::new((0.0, 0.0), self.radius.value as f64),
        );
    }

    fn update(&mut self, dt: Duration) {
        if self.radius.duration > Duration::ZERO && self.radius.elapsed < self.radius.duration {
            self.radius.elapsed += dt;
            let t =
                (self.radius.elapsed.as_secs_f32() / self.radius.duration.as_secs_f32()).min(1.0);
            self.radius.value =
                crate::engine::animation::lerp(self.radius.value, self.radius.target, t);
        }
    }
}
