use glam::Vec2;
use vello::peniko::{Brush, Color, Fill};
use vello::Scene;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use crate::engine::animation::{Animation, lerp};

pub trait Tweenable: Copy + Send + 'static {
    fn interpolate(a: Self, b: Self, t: f32) -> Self;
}

impl Tweenable for f32 {
    fn interpolate(a: Self, b: Self, t: f32) -> Self {
        lerp(a, b, t)
    }
}

impl Tweenable for Vec2 {
    fn interpolate(a: Self, b: Self, t: f32) -> Self {
        Vec2::new(lerp(a.x, b.x, t), lerp(a.y, b.y, t))
    }
}

pub struct SignalData<T> {
    pub value: T,
    pub target: T,
    pub duration: Duration,
    pub elapsed: Duration,
    pub easing: fn(f32) -> f32,
}

#[derive(Clone)]
pub struct Signal<T> {
    pub data: Arc<Mutex<SignalData<T>>>,
}

impl<T: Tweenable> Signal<T> {
    pub fn new(value: T) -> Self {
        Self {
            data: Arc::new(Mutex::new(SignalData {
                value,
                target: value,
                duration: Duration::ZERO,
                elapsed: Duration::ZERO,
                easing: crate::engine::easings::linear,
            })),
        }
    }

    pub fn to(&self, target: T, duration: Duration) -> Box<dyn Animation> {
        self.to_with_easing(target, duration, crate::engine::easings::linear)
    }

    pub fn to_with_easing(&self, target: T, duration: Duration, easing: fn(f32) -> f32) -> Box<dyn Animation> {
        {
            let mut data = self.data.lock().unwrap();
            data.target = target;
            data.duration = duration;
            data.elapsed = Duration::ZERO;
            data.easing = easing;
        }
        
        Box::new(SignalTween {
            data: self.data.clone(),
        })
    }
}

pub struct SignalTween<T> {
    data: Arc<Mutex<SignalData<T>>>,
}

impl<T: Tweenable> Animation for SignalTween<T> {
    fn update(&mut self, dt: Duration) -> bool {
        let mut data = self.data.lock().unwrap();
        if data.duration == Duration::ZERO {
            data.value = data.target;
            return true;
        }

        data.elapsed += dt;
        let t_linear = (data.elapsed.as_secs_f32() / data.duration.as_secs_f32()).min(1.0);
        let t_eased = (data.easing)(t_linear);
        data.value = T::interpolate(data.value, data.target, t_eased);
        
        data.elapsed >= data.duration
    }
}

pub trait Node: Send + Sync + 'static {
    fn render(&self, scene: &mut Scene);
    fn update(&mut self, dt: Duration);
    fn state_hash(&self) -> u64;
}

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
