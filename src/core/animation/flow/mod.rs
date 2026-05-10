//! Control flow primitives for orchestrating multiple animations.
//!
//! This module provides functions and macros to combine animations in parallel,
//! sequence, or with specific timing modifications (delays, staggers, easing overrides).

pub mod all;
pub mod any;
pub mod chain;
pub mod delay;
pub mod easing;
pub mod loop_anim;
pub mod sequence;
pub mod wait;

pub use all::all;
pub use any::any;
pub use chain::chain;
pub use delay::delay;
pub use easing::with_easing;
pub use loop_anim::loop_anim;
pub use sequence::sequence;
pub use wait::wait;

/// Runs multiple animations in parallel.
/// The resulting animation finishes when the *last* child finishes.
#[macro_export]
macro_rules! all {
    ($($anim:expr),* $(,)?) => {
        $crate::flows::all(vec![$(Box::new($anim) as Box<dyn $crate::core::animation::base::Animation>),*])
    };
}

/// Runs multiple animations in parallel.
/// The resulting animation finishes when the *first* child finishes.
#[macro_export]
macro_rules! any {
    ($($anim:expr),* $(,)?) => {
        $crate::flows::any(vec![$(Box::new($anim) as Box<dyn $crate::core::animation::base::Animation>),*])
    };
}

/// Runs multiple animations sequentially.
/// Each animation starts as soon as the previous one finishes.
#[macro_export]
macro_rules! chain {
    ($($anim:expr),* $(,)?) => {
        $crate::flows::chain(vec![$(Box::new($anim) as Box<dyn $crate::core::animation::base::Animation>),*])
    };
}

/// Adds a pre-delay to an animation.
#[macro_export]
macro_rules! delay {
    ($d:expr, $anim:expr $(,)?) => {
        $crate::flows::delay(
            $d,
            Box::new($anim) as Box<dyn $crate::core::animation::base::Animation>,
        )
    };
}

/// Runs multiple animations sequentially with a fixed stagger delay between starts.
#[macro_export]
macro_rules! sequence {
    ($stagger:expr, $($anim:expr),* $(,)?) => {
        $crate::flows::sequence($stagger, vec![$(Box::new($anim) as Box<dyn $crate::core::animation::base::Animation>),*])
    };
}

/// Repeats an animation factory multiple times.
#[macro_export]
macro_rules! loop_anim {
    ($factory:expr, $iters:expr $(,)?) => {
        $crate::flows::loop_anim(
            Box::new(move || {
                Box::new($factory) as Box<dyn $crate::core::animation::base::Animation>
            }),
            $iters,
        )
    };
}

/// Overrides the easing function for a set of animations running in parallel.
#[macro_export]
macro_rules! with_easing {
    ($easing:expr, [$($anim:expr),* $(,)?] $(,)?) => {
        $crate::flows::with_easing($easing, vec![$(Box::new($anim) as Box<dyn $crate::core::animation::base::Animation>),*])
    };
}

/// Triggers playback of an audio node.
#[cfg(feature = "audio")]
#[macro_export]
macro_rules! play {
    ($node:expr) => {
        Box::new($crate::elements::media::AudioAnimation::new($node))
            as Box<dyn $crate::core::animation::base::Animation>
    };
}

/// A specialized wait macro for audio-syncing.
#[cfg(feature = "audio")]
#[macro_export]
macro_rules! audio_wait {
    ($d:expr) => {
        $crate::flows::wait(std::time::Duration::from_secs_f32($d as f32))
    };
}
