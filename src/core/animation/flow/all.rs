use crate::core::animation::base::{Animation, AudioEvent};
use crate::core::animation::AnyAnimation;
use std::time::Duration;

/// An animation that runs multiple animations in parallel.
///
/// `All` manages a set of animations, updating each of them by the same `dt`
/// every frame. It is considered finished only when **every** child animation
/// has completed.
pub struct All {
    pub(crate) animations: Vec<AnyAnimation>,
    pub(crate) finished: Vec<bool>,
}

impl All {
    /// Creates a new `All` container with the provided animations.
    pub fn new(animations: Vec<AnyAnimation>) -> Self {
        let len = animations.len();
        Self {
            animations,
            finished: vec![false; len],
        }
    }
}

impl Animation for All {
    /// Updates all non-finished child animations.
    /// Returns `true` if all children are finished.
    fn update(&mut self, dt: Duration) -> (bool, Duration) {
        let mut all_finished = true;
        let mut min_leftover = dt;

        for (i, anim) in self.animations.iter_mut().enumerate() {
            if self.finished[i] {
                continue;
            }

            let (finished, leftover) = anim.update(dt);
            if finished {
                self.finished[i] = true;
                min_leftover = min_leftover.min(leftover);
            } else {
                all_finished = false;
            }
        }

        (
            all_finished,
            if all_finished {
                min_leftover
            } else {
                Duration::ZERO
            },
        )
    }

    /// The duration is the duration of the longest child animation.
    fn duration(&self) -> Duration {
        self.animations
            .iter()
            .map(|a| a.duration())
            .max()
            .unwrap_or(Duration::ZERO)
    }

    /// Propagates the easing function to all child animations.
    fn set_easing(&mut self, easing: fn(f32) -> f32) {
        for anim in &mut self.animations {
            anim.set_easing(easing);
        }
    }

    /// Collects audio events from all child animations using the same start time.
    fn collect_audio_events(&mut self, current_time: Duration, events: &mut Vec<AudioEvent>) {
        for anim in &mut self.animations {
            anim.collect_audio_events(current_time, events);
        }
    }

    /// Resets all child animations to their initial state.
    fn reset(&mut self) {
        for anim in &mut self.animations {
            anim.reset();
        }
        for f in &mut self.finished {
            *f = false;
        }
    }
}

/// Creates an animation that runs all passed animations in parallel.
///
/// Generally used via the [`all!`](crate::all) macro.
///
/// ### Example
/// ```rust
/// # use motion_canvas_rs::prelude::*;
/// # use std::time::Duration;
/// # let node = Rect::default()
/// #    .with_size(Vec2::new(100.0, 100.0))
/// #    .with_fill(Color::RED);
/// # let dur = Duration::from_secs(1);
/// all![
///     node.position.to(Vec2::new(100.0, 100.0), dur),
///     node.fill_color.to(Color::BLUE, dur),
/// ];
/// ```
pub fn all(animations: Vec<AnyAnimation>) -> AnyAnimation {
    AnyAnimation::All(All::new(animations))
}
