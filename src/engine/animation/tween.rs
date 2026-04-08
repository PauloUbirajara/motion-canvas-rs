use crate::engine::animation::base::Animation;
use crate::engine::nodes::{PathData, PathNode};
use glam::Vec2;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use vello::kurbo::Affine;
use vello::peniko::Color;

const DEFAULT_EASING: fn(f32) -> f32 = crate::engine::easings::cubic_in_out;

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub trait Tweenable: Clone + Send + Sync + std::fmt::Debug + 'static {
    fn interpolate(a: &Self, b: &Self, t: f32) -> Self;
}

impl Tweenable for f32 {
    fn interpolate(a: &Self, b: &Self, t: f32) -> Self {
        lerp(*a, *b, t)
    }
}

impl Tweenable for Vec2 {
    fn interpolate(a: &Self, b: &Self, t: f32) -> Self {
        Vec2::new(lerp(a.x, b.x, t), lerp(a.y, b.y, t))
    }
}

impl Tweenable for String {
    fn interpolate(a: &Self, b: &Self, t: f32) -> Self {
        if t >= 1.0 {
            b.clone()
        } else {
            a.clone()
        }
    }
}

impl Tweenable for Color {
    fn interpolate(a: &Self, b: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Color::rgba8(
            lerp(a.r as f32, b.r as f32, t) as u8,
            lerp(a.g as f32, b.g as f32, t) as u8,
            lerp(a.b as f32, b.b as f32, t) as u8,
            lerp(a.a as f32, b.a as f32, t) as u8,
        )
    }
}

impl Tweenable for Affine {
    fn interpolate(a: &Self, b: &Self, t: f32) -> Self {
        let t = t as f64;
        let c1 = a.as_coeffs();
        let c2 = b.as_coeffs();
        Affine::new([
            c1[0] + (c2[0] - c1[0]) * t,
            c1[1] + (c2[1] - c1[1]) * t,
            c1[2] + (c2[2] - c1[2]) * t,
            c1[3] + (c2[3] - c1[3]) * t,
            c1[4] + (c2[4] - c1[4]) * t,
            c1[5] + (c2[5] - c1[5]) * t,
        ])
    }
}

/// SignalData now only stores the current value, allowing multiple animations
/// to control it sequentially or in parallel without state interference.
pub struct SignalData<T> {
    pub value: T,
}

#[derive(Clone)]
pub struct Signal<T> {
    pub data: Arc<Mutex<SignalData<T>>>,
}

impl<T: Tweenable + PartialEq> Signal<T> {
    pub fn new(value: T) -> Self {
        Self {
            data: Arc::new(Mutex::new(SignalData { value })),
        }
    }

    pub fn get(&self) -> T {
        self.data.lock().unwrap().value.clone()
    }

    pub fn set(&self, value: T) {
        let mut data = self.data.lock().unwrap();
        if data.value != value {
            data.value = value;
        }
    }

    pub fn to(&self, target: T, duration: Duration) -> SignalTween<T> {
        SignalTween {
            data: self.data.clone(),
            start_value: None,
            target_value: target,
            duration,
            elapsed: Duration::ZERO,
            easing: DEFAULT_EASING,
        }
    }

    pub fn follow(&self, path: &PathNode, duration: Duration) -> FollowPath<T>
    where
        T: From<Vec2>,
    {
        FollowPath {
            data: self.data.clone(),
            path_data: path.data.clone(),
            duration,
            elapsed: Duration::ZERO,
            easing: DEFAULT_EASING,
        }
    }
}

/// SignalTween now tracks its own elapsed time and start/target values.
pub struct SignalTween<T> {
    data: Arc<Mutex<SignalData<T>>>,
    start_value: Option<T>,
    target_value: T,
    duration: Duration,
    elapsed: Duration,
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
        // Capture start value on first update
        if self.start_value.is_none() {
            let val = self.data.lock().unwrap().value.clone();
            self.start_value = Some(val);
        }

        if self.duration == Duration::ZERO {
            let mut data = self.data.lock().unwrap();
            data.value = self.target_value.clone();
            return true;
        }

        self.elapsed += dt;
        let t_linear = (self.elapsed.as_secs_f32() / self.duration.as_secs_f32()).min(1.0);
        let t_eased = (self.easing)(t_linear);

        let start = self.start_value.as_ref().unwrap();
        let mut data = self.data.lock().unwrap();
        data.value = T::interpolate(start, &self.target_value, t_eased);

        self.elapsed >= self.duration
    }

    fn duration(&self) -> Duration {
        self.duration
    }

    fn set_easing(&mut self, easing: fn(f32) -> f32) {
        self.easing = easing;
    }
}

pub struct FollowPath<T> {
    data: Arc<Mutex<SignalData<T>>>,
    path_data: Arc<PathData>,
    duration: Duration,
    elapsed: Duration,
    easing: fn(f32) -> f32,
}

impl<T: Send + Sync + 'static> FollowPath<T> {
    pub fn ease(mut self, easing: fn(f32) -> f32) -> Self {
        self.easing = easing;
        self
    }
}

impl<T: Tweenable + From<Vec2>> Animation for FollowPath<T> {
    fn update(&mut self, dt: Duration) -> bool {
        if self.duration == Duration::ZERO {
            let mut data = self.data.lock().unwrap();
            data.value = T::from(self.path_data.sample(1.0));
            return true;
        }

        self.elapsed += dt;
        let t_linear = (self.elapsed.as_secs_f32() / self.duration.as_secs_f32()).min(1.0);
        let t_eased = (self.easing)(t_linear);

        let mut data = self.data.lock().unwrap();
        data.value = T::from(self.path_data.sample(t_eased));

        self.elapsed >= self.duration
    }

    fn duration(&self) -> Duration {
        self.duration
    }

    fn set_easing(&mut self, easing: fn(f32) -> f32) {
        self.easing = easing;
    }
}

impl<T: Tweenable> From<SignalTween<T>> for Box<dyn Animation> {
    fn from(tween: SignalTween<T>) -> Self {
        Box::new(tween)
    }
}

impl<T: Tweenable + From<Vec2>> From<FollowPath<T>> for Box<dyn Animation> {
    fn from(anim: FollowPath<T>) -> Self {
        Box::new(anim)
    }
}
