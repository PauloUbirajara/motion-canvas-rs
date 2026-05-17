use crate::core::animation::base::{Animation, AudioEvent};
use std::time::Duration;

/// An animation that delays the execution of another animation.
///
/// `Delay` wraps an inner animation and prevents it from updating until the
/// specified delay duration has passed.
pub struct Delay {
    pub(crate) duration: Duration,
    pub(crate) elapsed: Duration,
    pub(crate) inner: Box<dyn Animation>,
}

impl Delay {
    /// Creates a new `Delay` for the provided duration and inner animation.
    pub fn new(duration: Duration, inner: Box<dyn Animation>) -> Self {
        Self {
            duration,
            elapsed: Duration::ZERO,
            inner,
        }
    }
}

impl Animation for Delay {
    /// Increments the internal elapsed timer.
    /// Once the timer exceeds the delay duration, it begins updating the inner animation.
    fn update(&mut self, dt: Duration) -> (bool, Duration) {
        if self.elapsed >= self.duration {
            return self.inner.update(dt);
        }

        self.elapsed += dt;
        if self.elapsed < self.duration {
            return (false, Duration::ZERO);
        }

        let leftover = self.elapsed - self.duration;
        self.inner.update(leftover)
    }

    /// The total duration is the delay plus the inner animation's duration.
    fn duration(&self) -> Duration {
        self.duration + self.inner.duration()
    }

    /// Propagates the easing function to the inner animation.
    fn set_easing(&mut self, easing: fn(f32) -> f32) {
        self.inner.set_easing(easing);
    }

    /// Collects audio events from the inner animation only if the delay has passed.
    fn collect_audio_events(&mut self, current_time: Duration, events: &mut Vec<AudioEvent>) {
        if self.elapsed >= self.duration {
            self.inner
                .collect_audio_events(current_time + self.duration, events);
        }
    }

    /// Resets the elapsed timer and the inner animation.
    fn reset(&mut self) {
        self.elapsed = Duration::ZERO;
        self.inner.reset();
    }
}

/// Creates an animation that waits for a duration before starting the inner animation.
///
/// Generally used via the [`delay!`](crate::delay) macro.
///
/// ### Example
/// ```rust
/// # use motion_canvas_rs::prelude::*;
/// # use std::time::Duration;
/// # let node = Rect::default()
/// #    .with_size(Vec2::new(100.0, 100.0))
/// #    .with_fill(Color::RED);
/// # let target = Vec2::new(100.0, 100.0);
/// # let dur = Duration::from_secs(1);
/// chain![
///     node.position.to(target.clone(), dur.clone()),
///     delay!(Duration::from_secs_f32(0.5), node.opacity.to(0.0, dur.clone())),
/// ];
/// ```
pub fn delay(duration: Duration, inner: Box<dyn Animation>) -> Box<dyn Animation> {
    Box::new(Delay::new(duration, inner))
}
