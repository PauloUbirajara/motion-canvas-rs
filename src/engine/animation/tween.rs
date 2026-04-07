use std::time::Duration;
use std::sync::{Arc, Mutex};
use glam::Vec2;
use crate::engine::animation::base::Animation;

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}


pub trait Tweenable: Copy + Send + Sync + 'static {
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

    pub fn to(&self, target: T, duration: Duration) -> SignalTween<T> {
        SignalTween {
            data: self.data.clone(),
            target,
            duration,
            easing: crate::engine::easings::linear,
        }
    }
}

pub struct SignalTween<T> {
    data: Arc<Mutex<SignalData<T>>>,
    target: T,
    duration: Duration,
    easing: fn(f32) -> f32,
}

impl<T: Tweenable> SignalTween<T> {
    pub fn ease(mut self, easing: fn(f32) -> f32) -> Self {
        self.easing = easing;
        self
    }
}

impl<T: Tweenable> Animation for SignalTween<T> {
    fn update(&mut self, dt: Duration) -> bool {
        let mut data = self.data.lock().unwrap();
        
        if data.elapsed == Duration::ZERO {
            data.target = self.target;
            data.duration = self.duration;
            data.easing = self.easing;
        }

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

impl<T: Tweenable> From<SignalTween<T>> for Box<dyn Animation> {
    fn from(tween: SignalTween<T>) -> Self {
        Box::new(tween)
    }
}
