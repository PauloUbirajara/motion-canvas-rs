use crate::core::animation::base::Animation;
use crate::elements::shapes::{PathData, PathNode};
use glam::Vec2;
use kurbo::Affine;
use peniko::Color;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const DEFAULT_EASING: fn(f32) -> f32 = crate::core::easings::cubic_in_out;

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// A trait for types that can be interpolated over time.
pub trait Tweenable: Clone + Send + Sync + std::fmt::Debug + 'static {
    /// Interpolates between `a` and `b` by factor `t` (0.0 to 1.0).
    fn interpolate(a: &Self, b: &Self, t: f32) -> Self;
    /// Returns a hash of the current value for change detection.
    fn state_hash(&self) -> u64;
}

impl Tweenable for f32 {
    fn interpolate(a: &Self, b: &Self, t: f32) -> Self {
        lerp(*a, *b, t)
    }
    fn state_hash(&self) -> u64 {
        crate::assets::hash::hash_f32(*self)
    }
}

impl Tweenable for Vec2 {
    fn interpolate(a: &Self, b: &Self, t: f32) -> Self {
        Vec2::new(lerp(a.x, b.x, t), lerp(a.y, b.y, t))
    }
    fn state_hash(&self) -> u64 {
        crate::assets::hash::combine_hashes(
            crate::assets::hash::hash_f32(self.x),
            crate::assets::hash::hash_f32(self.y),
        )
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
        let mut h = crate::assets::hash::Hasher::new();
        for v in self {
            h.update_u64(v.state_hash());
        }
        h.finish()
    }
}

impl Tweenable for bool {
    fn interpolate(a: &Self, b: &Self, t: f32) -> Self {
        if t >= 1.0 {
            *b
        } else {
            *a
        }
    }
    fn state_hash(&self) -> u64 {
        if *self {
            1
        } else {
            0
        }
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
        crate::assets::hash::hash_str(self)
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
        let mut h = crate::assets::hash::Hasher::new();
        h.update_bytes(&[self.r, self.g, self.b, self.a]);
        h.finish()
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
        let c = self.as_coeffs();
        let mut h = crate::assets::hash::Hasher::new();
        for val in c {
            h.update_u64(val.to_bits() as u64);
        }
        h.finish()
    }
}

/// Internal data for a signal.
pub struct SignalData<T> {
    /// The current value of the signal.
    pub value: T,
    /// The initial value the signal was created with.
    pub initial: T,
}

/// A reactive wrapper for a value that can be tweened over time.
///
/// `Signal` is the primary way to define animatable properties for `Node` elements.
/// It wraps a `Tweenable` value and provides methods to create animations that
/// change this value smoothly.
///
/// ### Example
/// ```rust
/// # use motion_canvas_rs::prelude::*;
/// # use glam::Vec2;
/// # use std::time::Duration;
/// let pos = Signal::new(Vec2::ZERO);
/// let tween = pos.to(Vec2::new(100.0, 100.0), Duration::from_secs(1));
/// ```
#[derive(Clone)]
pub struct Signal<T> {
    pub data: Arc<Mutex<SignalData<T>>>,
}

/// A trait for types that can be constructed from a `Vec2`.
/// Used for mapping path samples to signal types (e.g. `Vec2` or `Affine`).
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

/// The target of a tween animation.
pub enum Target<T> {
    /// A specific fixed value.
    Fixed(T),
    /// A value calculated lazily at the start of the animation based on the signal's state at that time.
    Lazy(Arc<dyn Fn(&T) -> T + Send + Sync>),
}

impl<T: Clone> Clone for Target<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Fixed(v) => Self::Fixed(v.clone()),
            Self::Lazy(f) => Self::Lazy(f.clone()),
        }
    }
}
impl<T: Tweenable + PartialEq> Signal<T> {
    /// Creates a new signal with the given initial value.
    pub fn new(value: T) -> Self {
        Self {
            data: Arc::new(Mutex::new(SignalData {
                value: value.clone(),
                initial: value,
            })),
        }
    }

    /// Retrieves a clone of the current value.
    pub fn get(&self) -> T {
        self.data.lock().unwrap().value.clone()
    }

    /// Manually overrides the current value.
    pub fn set(&self, value: T) {
        let mut data = self.data.lock().unwrap();
        if data.value != value {
            data.value = value;
        }
    }

    /// Returns a hash of the current value.
    pub fn state_hash(&self) -> u64 {
        self.data.lock().unwrap().value.state_hash()
    }

    /// Resets the signal to its initial value.
    pub fn reset(&self) {
        let mut data = self.data.lock().unwrap();
        data.value = data.initial.clone();
    }

    /// Returns a tween animation that changes the signal to a fixed target value.
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

    /// Returns a tween animation where the target is calculated lazily when the animation starts.
    ///
    /// Useful for relative offsets (e.g., `pos.to_lazy(|v| *v + offset, duration)`).
    pub fn to_lazy<F>(&self, factory: F, duration: Duration) -> SignalTween<T>
    where
        F: Fn(&T) -> T + Send + Sync + 'static,
    {
        SignalTween {
            data: self.data.clone(),
            start_value: None,
            target: Target::Lazy(Arc::new(factory)),
            target_value: None,
            duration,
            elapsed: Duration::ZERO,
            easing: DEFAULT_EASING,
        }
    }

    /// Returns an animation that makes the signal's value follow a `PathNode`.
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

    /// Binds this signal to another signal, transforming its value via a mapper function.
    pub fn bind<S: Tweenable + PartialEq, F>(
        &self,
        source: Signal<S>,
        mapper: F,
    ) -> crate::core::animation::binding::BindingNode<T, S>
    where
        F: Fn(S) -> T + Send + Sync + 'static,
    {
        crate::core::animation::binding::BindingNode::new(source, self.clone(), mapper)
    }
}

/// An animation that smoothly changes a `Signal`'s value over time.
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
    /// Applies a custom easing function to this tween.
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

            // Evaluate target if needed
            if self.target_value.is_none() {
                match &self.target {
                    Target::Fixed(v) => self.target_value = Some(v.clone()),
                    Target::Lazy(f) => self.target_value = Some(f(&current)),
                }
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

    fn reset(&mut self) {
        self.start_value = None;
        self.target_value = None;
        self.elapsed = Duration::ZERO;

        let mut data = self.data.lock().unwrap();
        data.value = data.initial.clone();
    }
}

/// An animation that samples a `PathNode`'s geometry to update a `Signal`.
pub struct FollowPath<T> {
    data: Arc<Mutex<SignalData<T>>>,
    path_data: Arc<PathData>,
    duration: Duration,
    elapsed: Duration,
    easing: fn(f32) -> f32,
}

impl<T: Send + Sync + 'static> FollowPath<T> {
    /// Applies a custom easing function to this animation.
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

    fn reset(&mut self) {
        self.elapsed = Duration::ZERO;
        let mut data = self.data.lock().unwrap();
        data.value = data.initial.clone();
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
