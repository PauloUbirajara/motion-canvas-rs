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

    pub fn update(&mut self, mut dt: std::time::Duration) {
        while !self.animations.is_empty() {
            let (finished, leftover) = self.animations[0].update(dt);
            if finished {
                self.animations.remove(0);
                dt = leftover;
                if dt == std::time::Duration::ZERO {
                    break;
                }
            } else {
                break;
            }
        }
    }

    pub fn finished(&self) -> bool {
        self.animations.is_empty()
    }

    pub fn duration(&self) -> std::time::Duration {
        self.animations
            .iter()
            .map(|a| a.duration())
            .fold(std::time::Duration::ZERO, |acc, d| acc + d)
    }

    pub fn collect_audio_events(
        &mut self,
        current_time: std::time::Duration,
        events: &mut Vec<AudioEvent>,
    ) {
        if !self.animations.is_empty() {
            self.animations[0].collect_audio_events(current_time, events);
        }
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

/// Play an audio node.
#[cfg(feature = "audio")]
#[macro_export]
macro_rules! play {
    ($audio:expr) => {
        Into::<Box<dyn $crate::engine::animation::base::Animation>>::into($audio)
    };
}

/// Wait on the audio timeline.
#[cfg(feature = "audio")]
#[macro_export]
macro_rules! audio_wait {
    ($dur:expr) => {
        $crate::engine::animation::flow::wait(std::time::Duration::from_secs_f32($dur as f32))
    };
}
