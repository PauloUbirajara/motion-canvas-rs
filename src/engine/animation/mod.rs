pub mod base;
pub mod flow;
pub mod tween;

pub use base::*;
pub use flow::*;
pub use tween::*;

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

    pub fn update(&mut self, dt: std::time::Duration) {
        self.animations.retain_mut(|anim| !anim.update(dt));
    }

    pub fn finished(&self) -> bool {
        self.animations.is_empty()
    }
}


/// Run animations in parallel.
#[macro_export]
macro_rules! all {
    ($($x:expr),*) => {
        $crate::engine::animation::flow::all(vec![$(Into::<Box<dyn $crate::engine::animation::base::Animation>>::into($x)),*])
    };
}

/// Run animations in race.
#[macro_export]
macro_rules! any {
    ($($x:expr),*) => {
        $crate::engine::animation::flow::any(vec![$(Into::<Box<dyn $crate::engine::animation::base::Animation>>::into($x)),*])
    };
}

/// Run animations sequentially.
#[macro_export]
macro_rules! chain {
    ($($x:expr),*) => {
        $crate::engine::animation::flow::chain(vec![$(Into::<Box<dyn $crate::engine::animation::base::Animation>>::into($x)),*])
    };
}

/// Create a sequence with staggered start times.
#[macro_export]
macro_rules! sequence {
    ($stagger:expr, $($x:expr),*) => {
        $crate::engine::animation::flow::sequence($stagger, vec![$(Into::<Box<dyn $crate::engine::animation::base::Animation>>::into($x)),*])
    };
}

