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
///
/// ### Example
/// ```rust
/// # use motion_canvas_rs::prelude::*;
/// # use std::time::Duration;
/// # let circle = Circle::default();
/// # let rect = Rect::default();
/// all![
///     circle.position.to(Vec2::new(100.0, 100.0), Duration::from_secs_f32(1.0)),
///     rect.opacity.to(0.0, Duration::from_secs_f32(0.5)),
/// ];
/// ```
#[macro_export]
macro_rules! all {
    ($($anim:expr),* $(,)?) => {
        $crate::flows::all(vec![$(Box::new($anim) as Box<dyn $crate::core::animation::base::Animation>),*])
    };
}

/// Runs multiple animations in parallel.
/// The resulting animation finishes when the *first* child finishes.
///
/// ### Example
/// ```rust
/// # use motion_canvas_rs::prelude::*;
/// # use std::time::Duration;
/// # let circle = Circle::default();
/// # let rect = Rect::default();
/// any![
///     circle.position.to(Vec2::new(100.0, 100.0), Duration::from_secs(2)),
///     rect.opacity.to(0.0, Duration::from_secs_f32(0.5)), // Finish after 0.5s
/// ];
/// ```
#[macro_export]
macro_rules! any {
    ($($anim:expr),* $(,)?) => {
        $crate::flows::any(vec![$(Box::new($anim) as Box<dyn $crate::core::animation::base::Animation>),*])
    };
}

/// Runs multiple animations sequentially.
/// Each animation starts as soon as the previous one finishes.
///
/// ### Example
/// ```rust
/// # use motion_canvas_rs::prelude::*;
/// # use std::time::Duration;
/// # let circle = Circle::default();
/// # let target1 = Vec2::new(100.0, 0.0);
/// # let target2 = Vec2::new(100.0, 100.0);
/// # let dur = Duration::from_secs(1);
/// chain![
///     circle.position.to(target1.clone(), dur.clone()),
///     circle.position.to(target2.clone(), dur.clone()),
/// ];
/// ```
#[macro_export]
macro_rules! chain {
    ($($anim:expr),* $(,)?) => {
        $crate::flows::chain(vec![$(Box::new($anim) as Box<dyn $crate::core::animation::base::Animation>),*])
    };
}

/// Adds a pre-delay to an animation.
///
/// ### Example
/// ```rust
/// # use motion_canvas_rs::prelude::*;
/// # use std::time::Duration;
/// # let circle = Circle::default();
/// delay!(Duration::from_secs_f32(1.0), circle.opacity.to(1.0, Duration::from_secs_f32(0.5)));
/// ```
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
///
/// ### Example
/// ```rust
/// # use motion_canvas_rs::prelude::*;
/// # use std::time::Duration;
/// # let c1 = Circle::default();
/// # let c2 = Circle::default();
/// sequence!(
///     Duration::from_millis(100),
///     c1.scale.to(Vec2::splat(2.0), Duration::from_secs_f32(0.5)),
///     c2.scale.to(Vec2::splat(2.0), Duration::from_secs_f32(0.5)),
/// );
/// ```
#[macro_export]
macro_rules! sequence {
    ($stagger:expr, $($anim:expr),* $(,)?) => {
        $crate::flows::sequence($stagger, vec![$(Box::new($anim) as Box<dyn $crate::core::animation::base::Animation>),*])
    };
}

/// Repeats an animation factory multiple times.
///
/// ### Example
/// ```rust
/// # use motion_canvas_rs::prelude::*;
/// # use std::time::Duration;
/// # let circle = Circle::default();
/// # let target = 6.28;
/// # let dur = Duration::from_secs(1);
/// loop_anim!(circle.rotation.to(target, dur.clone()), Some(5)); // Loops 5 times
/// // loop_anim!(circle.rotation.to(target, dur), None); // Loops forever
/// ```
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
///
/// ### Example
/// ```rust
/// # use motion_canvas_rs::prelude::*;
/// # use std::time::Duration;
/// # let circle = Circle::default();
/// # let target = Vec2::new(100.0, 100.0);
/// # let dur = Duration::from_secs(1);
/// with_easing!(easings::elastic_out, [
///     circle.position.to(target.clone(), dur.clone()),
/// ]);
/// ```
#[macro_export]
macro_rules! with_easing {
    ($easing:expr, [$($anim:expr),* $(,)?] $(,)?) => {
        $crate::flows::with_easing($easing, vec![$(Box::new($anim) as Box<dyn $crate::core::animation::base::Animation>),*])
    };
}

/// Triggers playback of an audio node.
///
/// ### Example
/// ```rust
/// # use motion_canvas_rs::prelude::*;
/// # let audio = AudioNode::new("assets/sound.wav");
/// play!(audio);
/// ```
#[cfg(feature = "audio")]
#[macro_export]
macro_rules! play {
    ($node:expr) => {
        Box::new($crate::elements::media::AudioAnimation::new($node))
            as Box<dyn $crate::core::animation::base::Animation>
    };
}

/// A specialized wait macro for audio-syncing.
///
/// ### Example
/// ```rust
/// # use motion_canvas_rs::prelude::*;
/// audio_wait!(2.0); // Wait for 2 seconds
/// ```
#[cfg(feature = "audio")]
#[macro_export]
macro_rules! audio_wait {
    ($d:expr) => {
        $crate::flows::wait(std::time::Duration::from_secs_f32($d as f32))
    };
}

/// Creates a wait animation for a given number of seconds.
///
/// ### Example
/// ```rust
/// # use motion_canvas_rs::prelude::*;
/// chain![
///     all![/* ... */],
///     wait!(1.0),
///     any![/* ... */],
/// ];
/// ```
#[macro_export]
macro_rules! wait {
    ($d:expr) => {
        $crate::flows::wait(std::time::Duration::from_secs_f32($d as f32))
    };
}
