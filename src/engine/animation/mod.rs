pub mod base;
pub mod flow;
pub mod tween;

pub use base::*;
pub use flow::*;
pub use tween::*;

pub struct Timeline {
    pub animations: Vec<Box<dyn Animation>>,
    pub index: usize,
}

impl Timeline {
    pub fn new() -> Self {
        Self {
            animations: Vec::new(),
            index: 0,
        }
    }

    pub fn add<A: Into<Box<dyn Animation>>>(&mut self, anim: A) {
        self.animations.push(anim.into());
    }

    pub fn update(&mut self, mut dt: std::time::Duration) {
        while let Some(anim) = self.animations.get_mut(self.index) {
            let (finished, leftover) = anim.update(dt);
            if finished {
                self.index += 1;
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
        self.index >= self.animations.len()
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
        if let Some(anim) = self.animations.get_mut(self.index) {
            anim.collect_audio_events(current_time, events);
        }
    }

    pub fn reset(&mut self) {
        for anim in &mut self.animations {
            anim.reset();
        }
        self.index = 0;
    }
}

/// Run animations in parallel.
///
/// # Example
/// ```rust
/// # use std::time::Duration;
/// # use motion_canvas_rs::prelude::*;
/// # let node = Circle::default();
/// all![
///     node.position.to(Vec2::new(100.0, 100.0), Duration::from_secs(1)),
///     node.opacity.to(1.0, Duration::from_millis(500)),
/// ];
/// ```
#[macro_export]
macro_rules! all {
    ($($x:expr),* $(,)?) => {
        $crate::engine::animation::flow::all(vec![$(Into::<Box<dyn $crate::engine::animation::base::Animation>>::into($x)),*])
    };
}

/// Run animations in race (completes when the first one finishes).
///
/// # Example
/// ```rust
/// # use std::time::Duration;
/// # use motion_canvas_rs::prelude::*;
/// # let node = Circle::default();
/// # let target = Vec2::ZERO;
/// any![
///     wait(Duration::from_secs(5)),
///     node.position.to(target, Duration::from_secs(2)),
/// ];
/// ```
#[macro_export]
macro_rules! any {
    ($($x:expr),* $(,)?) => {
        $crate::engine::animation::flow::any(vec![$(Into::<Box<dyn $crate::engine::animation::base::Animation>>::into($x)),*])
    };
}

/// Run animations sequentially.
///
/// # Example
/// ```rust
/// # use std::time::Duration;
/// # use motion_canvas_rs::prelude::*;
/// # let node = Circle::default();
/// # let (p1, p2) = (Vec2::ZERO, Vec2::ZERO);
/// chain![
///     node.position.to(p1, Duration::from_secs(1)),
///     wait(Duration::from_millis(500)),
///     node.position.to(p2, Duration::from_secs(1)),
/// ];
/// ```
#[macro_export]
macro_rules! chain {
    ($($x:expr),* $(,)?) => {
        $crate::engine::animation::flow::chain(vec![$(Into::<Box<dyn $crate::engine::animation::base::Animation>>::into($x)),*])
    };
}

/// Create a sequence with staggered start times.
///
/// # Example
/// ```rust
/// # use std::time::Duration;
/// # use motion_canvas_rs::prelude::*;
/// # let nodes = [Circle::default(), Circle::default(), Circle::default()];
/// sequence![
///     Duration::from_millis(100),
///     nodes[0].opacity.to(1.0, Duration::from_secs(1)),
///     nodes[1].opacity.to(1.0, Duration::from_secs(1)),
///     nodes[2].opacity.to(1.0, Duration::from_secs(1)),
/// ];
/// ```
#[macro_export]
macro_rules! sequence {
    ($stagger:expr, $($x:expr),* $(,)?) => {
        $crate::engine::animation::flow::sequence($stagger, vec![$(Into::<Box<dyn $crate::engine::animation::base::Animation>>::into($x)),*])
    };
}

/// Delay the start of an animation.
///
/// # Example
/// ```rust
/// # use std::time::Duration;
/// # use motion_canvas_rs::prelude::*;
/// # let node = Circle::default();
/// delay!(
///     Duration::from_secs(1),
///     node.scale.to(Vec2::splat(2.0), Duration::from_secs(1))
/// );
/// ```
#[macro_export]
macro_rules! delay {
    ($duration:expr, $inner:expr $(,)?) => {
        $crate::engine::animation::flow::delay(
            $duration,
            Into::<Box<dyn $crate::engine::animation::base::Animation>>::into($inner),
        )
    };
}

/// Loop an animation factory a specified number of times.
///
/// # Example
/// ```rust
/// # use std::time::Duration;
/// # use motion_canvas_rs::prelude::*;
/// # let node = Circle::default();
/// loop_anim!(
///     node.rotation.to(std::f32::consts::TAU, Duration::from_secs(1)),
///     Some(5)
/// );
/// ```
#[macro_export]
macro_rules! loop_anim {
    ($anim:expr, $count:expr $(,)?) => {
        $crate::engine::animation::flow::loop_anim(
            move || {
                Into::<Box<dyn $crate::engine::animation::base::Animation>>::into($anim)
            },
            $count,
        )
    };
}

/// Run animations in parallel with a shared easing override.
///
/// # Example
/// ```rust
/// # use std::time::Duration;
/// # use motion_canvas_rs::prelude::*;
/// # let (node1, node2) = (Circle::default(), Circle::default());
/// with_easing!(
///     easings::elastic_out,
///     [
///         node1.scale.to(Vec2::splat(1.5), Duration::from_secs(1)),
///         node2.scale.to(Vec2::splat(1.5), Duration::from_secs(1)),
///     ]
/// );
/// ```
#[macro_export]
macro_rules! with_easing {
    ($easing:expr, [$($x:expr),* $(,)?]) => {
        $crate::engine::animation::flow::with_easing($easing, vec![$(Into::<Box<dyn $crate::engine::animation::base::Animation>>::into($x)),*])
    };
}

/// Play an audio node.
///
/// # Example
/// ```rust
/// # use motion_canvas_rs::prelude::*;
/// # let bg_music = AudioNode::new("music.mp3");
/// play!(bg_music);
/// ```
#[cfg(feature = "audio")]
#[macro_export]
macro_rules! play {
    ($audio:expr) => {
        Into::<Box<dyn $crate::engine::animation::base::Animation>>::into($audio)
    };
}

/// Wait on the audio timeline.
///
/// # Example
/// ```rust
/// # use motion_canvas_rs::prelude::*;
/// audio_wait!(2.5);
/// ```
#[cfg(feature = "audio")]
#[macro_export]
macro_rules! audio_wait {
    ($dur:expr) => {
        $crate::engine::animation::flow::wait(std::time::Duration::from_secs_f32($dur as f32))
    };
}
