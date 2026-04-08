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

    pub fn duration(&self) -> std::time::Duration {
        self.animations
            .iter()
            .map(|a| a.duration())
            .max()
            .unwrap_or(std::time::Duration::ZERO)
    }
}

/// Run animations in parallel.
#[macro_export]
macro_rules! all {
    ($($x:expr),* $(,)?) => {
        $crate::engine::animation::flow::all(vec![$(Into::<Box<dyn $crate::engine::animation::base::Animation>>::into($x)),*])
    };
}

/// Run animations in race.
#[macro_export]
macro_rules! any {
    ($($x:expr),* $(,)?) => {
        $crate::engine::animation::flow::any(vec![$(Into::<Box<dyn $crate::engine::animation::base::Animation>>::into($x)),*])
    };
}

/// Run animations sequentially.
#[macro_export]
macro_rules! chain {
    ($($x:expr),* $(,)?) => {
        $crate::engine::animation::flow::chain(vec![$(Into::<Box<dyn $crate::engine::animation::base::Animation>>::into($x)),*])
    };
}

/// Create a sequence with staggered start times.
#[macro_export]
macro_rules! sequence {
    ($stagger:expr, $($x:expr),* $(,)?) => {
        $crate::engine::animation::flow::sequence($stagger, vec![$(Into::<Box<dyn $crate::engine::animation::base::Animation>>::into($x)),*])
    };
}

/// Delay an animation.
#[macro_export]
macro_rules! delay {
    ($duration:expr, $inner:expr $(,)?) => {
        $crate::engine::animation::flow::delay(
            $duration,
            Into::<Box<dyn $crate::engine::animation::base::Animation>>::into($inner),
        )
    };
}

/// Loop an animation factory.
#[macro_export]
macro_rules! loop_anim {
    ($anim:expr, $count:expr $(,)?) => {
        $crate::engine::animation::flow::loop_anim(
            Box::new(move || {
                Into::<Box<dyn $crate::engine::animation::base::Animation>>::into($anim)
            }),
            $count,
        )
    };
}
/// Run animations in parallel with a shared easing override.
#[macro_export]
macro_rules! with_easing {
    ($easing:expr, [$($x:expr),* $(,)?]) => {
        $crate::engine::animation::flow::with_easing($easing, vec![$(Into::<Box<dyn $crate::engine::animation::base::Animation>>::into($x)),*])
    };
}
