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
            })),
        }
    }

    pub fn to(&self, target: T, duration: Duration) -> Box<dyn Animation> {
        {
            let mut data = self.data.lock().unwrap();
            data.target = target;
            data.duration = duration;
            data.elapsed = Duration::ZERO;
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
        let t = (data.elapsed.as_secs_f32() / data.duration.as_secs_f32()).min(1.0);
        data.value = T::interpolate(data.value, data.target, t);
        
        data.elapsed >= data.duration
    }
}

pub trait Node: Send + Sync {
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
}
