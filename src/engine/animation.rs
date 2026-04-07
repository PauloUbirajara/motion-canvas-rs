use std::time::Duration;

pub trait Animation: Send + Sync {
    /// Update animation. Return true if finished.
    fn update(&mut self, dt: Duration) -> bool;
}

/// Helper macro to create a parallel animation group.
/// Boxes elements automatically so you don't have to call .into()!
#[macro_export]
macro_rules! all {
    ($($x:expr),*) => {
        $crate::engine::animation::all(vec![$(Box::new($x) as Box<dyn $crate::engine::animation::Animation>),*])
    };
}

/// Helper macro to create a race animation group.
#[macro_export]
macro_rules! any {
    ($($x:expr),*) => {
        $crate::engine::animation::any(vec![$(Box::new($x) as Box<dyn $crate::engine::animation::Animation>),*])
    };
}


pub struct All {
    animations: Vec<Box<dyn Animation>>,
}

impl All {
    pub fn new(animations: Vec<Box<dyn Animation>>) -> Self {
        Self { animations }
    }
}

impl Animation for All {
    fn update(&mut self, dt: Duration) -> bool {
        let mut all_finished = true;
        for anim in &mut self.animations {
            if !anim.update(dt) {
                all_finished = false;
            }
        }
        all_finished
    }
}

pub struct Any {
    animations: Vec<Box<dyn Animation>>,
}

impl Any {
    pub fn new(animations: Vec<Box<dyn Animation>>) -> Self {
        Self { animations }
    }
}

impl Animation for Any {
    fn update(&mut self, dt: Duration) -> bool {
        let mut any_finished = false;
        for anim in &mut self.animations {
            if anim.update(dt) {
                any_finished = true;
            }
        }
        any_finished
    }
}

pub struct Timeline {
    pub animations: Vec<Box<dyn Animation>>,
}

impl Timeline {
    pub fn new() -> Self {
        Self {
            animations: Vec::new(),
        }
    }

    pub fn add<A: Into<Box<dyn Animation>>>(&mut self, anim: A) {
        self.animations.push(anim.into());
    }

    pub fn update(&mut self, dt: Duration) {
        self.animations.retain_mut(|anim| !anim.update(dt));
    }
}

pub fn all(animations: Vec<Box<dyn Animation>>) -> Box<dyn Animation> {
    Box::new(All::new(animations))
}

pub fn any(animations: Vec<Box<dyn Animation>>) -> Box<dyn Animation> {
    Box::new(Any::new(animations))
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub fn ease_in_out(t: f32) -> f32 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
    }
}
