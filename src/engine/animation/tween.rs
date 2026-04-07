use std::time::Duration;
use std::sync::{Arc, Mutex};
use glam::Vec2;
use crate::engine::animation::base::Animation;
use crate::engine::node::{PathData, PathNode};

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

/// SignalData now only stores the current value, allowing multiple animations
/// to control it sequentially or in parallel without state interference.
pub struct SignalData<T> {
    pub value: T,
}

#[derive(Clone)]
pub struct Signal<T> {
    pub data: Arc<Mutex<SignalData<T>>>,
}

impl<T: Tweenable> Signal<T> {
    pub fn new(value: T) -> Self {
        Self {
            data: Arc::new(Mutex::new(SignalData { value })),
        }
    }

    pub fn to(&self, target: T, duration: Duration) -> SignalTween<T> {
        SignalTween {
            data: self.data.clone(),
            start_value: None,
            target_value: target,
            duration,
            elapsed: Duration::ZERO,
            easing: crate::engine::easings::linear,
        }
    }

    pub fn follow(&self, path: &PathNode, duration: Duration) -> FollowPath<T> 
    where T: From<Vec2>
    {
        FollowPath {
            data: self.data.clone(),
            path_data: path.data.clone(),
            duration,
            elapsed: Duration::ZERO,
            easing: crate::engine::easings::linear,
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
            self.start_value = Some(self.data.lock().unwrap().value);
        }

        if self.duration == Duration::ZERO {
            let mut data = self.data.lock().unwrap();
            data.value = self.target_value;
            return true;
        }

        self.elapsed += dt;
        let t_linear = (self.elapsed.as_secs_f32() / self.duration.as_secs_f32()).min(1.0);
        let t_eased = (self.easing)(t_linear);
        
        let start = self.start_value.unwrap();
        let mut data = self.data.lock().unwrap();
        data.value = T::interpolate(start, self.target_value, t_eased);
        
        self.elapsed >= self.duration
    }

    fn duration(&self) -> Duration {
        self.duration
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
