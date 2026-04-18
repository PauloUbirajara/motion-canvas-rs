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
    fn state_hash(&self) -> u64;
}

impl Tweenable for f32 {
    fn interpolate(a: &Self, b: &Self, t: f32) -> Self {
        lerp(*a, *b, t)
    }
    fn state_hash(&self) -> u64 {
        self.to_bits() as u64
    }
}

impl Tweenable for Vec2 {
    fn interpolate(a: &Self, b: &Self, t: f32) -> Self {
        Vec2::new(lerp(a.x, b.x, t), lerp(a.y, b.y, t))
    }
    fn state_hash(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        self.x.to_bits().hash(&mut hasher);
        self.y.to_bits().hash(&mut hasher);
        hasher.finish()
    }
}

impl Tweenable for Vec<Vec2> {
    fn interpolate(a: &Self, b: &Self, t: f32) -> Self {
        if a.len() == b.len() {
            a.iter()
                .zip(b.iter())
                .map(|(v1, v2)| Vec2::interpolate(v1, v2, t))
                .collect()
        } else if t >= 1.0 {
            b.clone()
        } else {
            a.clone()
        }
    }
    fn state_hash(&self) -> u64 {
        let mut hash = 0u64;
        for v in self {
            hash ^= v.state_hash();
        }
        hash
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
    fn state_hash(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
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
    fn state_hash(&self) -> u64 {
        ((self.r as u64) << 24) | ((self.g as u64) << 16) | ((self.b as u64) << 8) | (self.a as u64)
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
    fn state_hash(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        let c = self.as_coeffs();
        for val in c {
            val.to_bits().hash(&mut hasher);
        }
        hasher.finish()
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

pub trait FromVec2: Send + Sync + 'static {
    fn from_vec2(v: Vec2) -> Self;
}

impl FromVec2 for Vec2 {
    fn from_vec2(v: Vec2) -> Self {
        v
    }
}

impl FromVec2 for Affine {
    fn from_vec2(v: Vec2) -> Self {
        Affine::translate((v.x as f64, v.y as f64))
    }
}

pub enum Target<T> {
    Fixed(T),
    Lazy(Box<dyn FnOnce(&T) -> T + Send + Sync>),
}

impl<T: Clone> Clone for Target<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Fixed(v) => Self::Fixed(v.clone()),
            Self::Lazy(_) => panic!("Cannot clone lazy target"),
        }
    }
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

    pub fn state_hash(&self) -> u64 {
        self.data.lock().unwrap().value.state_hash()
    }

    pub fn to(&self, target: T, duration: Duration) -> SignalTween<T> {
        SignalTween {
            data: self.data.clone(),
            start_value: None,
            target: Target::Fixed(target),
            target_value: None,
            duration,
            elapsed: Duration::ZERO,
            easing: DEFAULT_EASING,
        }
    }

    pub fn to_lazy<F>(&self, factory: F, duration: Duration) -> SignalTween<T>
    where
        F: FnOnce(&T) -> T + Send + Sync + 'static,
    {
        SignalTween {
            data: self.data.clone(),
            start_value: None,
            target: Target::Lazy(Box::new(factory)),
            target_value: None,
            duration,
            elapsed: Duration::ZERO,
            easing: DEFAULT_EASING,
        }
    }

    pub fn follow(&self, path: &PathNode, duration: Duration) -> FollowPath<T>
    where
        T: FromVec2,
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
    target: Target<T>,
    target_value: Option<T>,
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
    fn update(&mut self, dt: Duration) -> (bool, Duration) {
        // Capture values on first update
        if self.start_value.is_none() {
            let current = self.data.lock().unwrap().value.clone();
            self.start_value = Some(current.clone());

            // Evaluate lazy target if needed
            match std::mem::replace(&mut self.target, Target::Fixed(current.clone())) {
                Target::Fixed(v) => self.target_value = Some(v),
                Target::Lazy(f) => self.target_value = Some(f(&current)),
            }
        }

        let target = self.target_value.as_ref().unwrap();

        if self.duration == Duration::ZERO {
            let mut data = self.data.lock().unwrap();
            data.value = target.clone();
            return (true, dt);
        }

        self.elapsed += dt;
        let finished = self.elapsed >= self.duration;
        let leftover = if finished {
            self.elapsed - self.duration
        } else {
            Duration::ZERO
        };

        let t_linear = (self.elapsed.as_secs_f32() / self.duration.as_secs_f32()).min(1.0);
        let t_eased = (self.easing)(t_linear);

        let start = self.start_value.as_ref().unwrap();
        let mut data = self.data.lock().unwrap();
        data.value = T::interpolate(start, target, t_eased);

        (finished, leftover)
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

impl<T: Tweenable + FromVec2> Animation for FollowPath<T> {
    fn update(&mut self, dt: Duration) -> (bool, Duration) {
        if self.duration == Duration::ZERO {
            let mut data = self.data.lock().unwrap();
            data.value = T::from_vec2(self.path_data.sample(1.0));
            return (true, dt);
        }

        self.elapsed += dt;
        let finished = self.elapsed >= self.duration;
        let leftover = if finished {
            self.elapsed - self.duration
        } else {
            Duration::ZERO
        };

        let t_linear = (self.elapsed.as_secs_f32() / self.duration.as_secs_f32()).min(1.0);
        let t_eased = (self.easing)(t_linear);

        let mut data = self.data.lock().unwrap();
        data.value = T::from_vec2(self.path_data.sample(t_eased));

        (finished, leftover)
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

impl<T: Tweenable + FromVec2> From<FollowPath<T>> for Box<dyn Animation> {
    fn from(anim: FollowPath<T>) -> Self {
        Box::new(anim)
    }
}
