use crate::engine::animation::base::{Animation, AudioEvent};
use std::time::Duration;

/// An animation that delays the execution of another animation.
pub struct Delay {
    pub(crate) duration: Duration,
    pub(crate) elapsed: Duration,
    pub(crate) inner: Box<dyn Animation>,
}

impl Delay {
    pub fn new(duration: Duration, inner: Box<dyn Animation>) -> Self {
        Self {
            duration,
            elapsed: Duration::ZERO,
            inner,
        }
    }
}

impl Animation for Delay {
    fn update(&mut self, dt: Duration) -> (bool, Duration) {
        if self.elapsed < self.duration {
            self.elapsed += dt;
            if self.elapsed >= self.duration {
                let leftover = self.elapsed - self.duration;
                self.inner.update(leftover)
            } else {
                (false, Duration::ZERO)
            }
        } else {
            self.inner.update(dt)
        }
    }

    fn duration(&self) -> Duration {
        self.duration + self.inner.duration()
    }

    fn set_easing(&mut self, easing: fn(f32) -> f32) {
        self.inner.set_easing(easing);
    }

    fn collect_audio_events(&mut self, current_time: Duration, events: &mut Vec<AudioEvent>) {
        if self.elapsed >= self.duration {
            self.inner.collect_audio_events(current_time, events);
        }
    }
}

/// Creates an animation that waits for a duration before starting the inner animation.
///
/// Generally used via the `delay!` macro.
///
/// ### Example
/// ```rust
/// # use motion_canvas_rs::prelude::*;
/// # use std::time::Duration;
/// # let node = nodes::Rect::new(Vec2::ZERO, Vec2::new(100.0, 100.0), Color::RED);
/// # let target = Affine::translate((100.0, 100.0));
/// # let dur = Duration::from_secs(1);
/// delay!(
///     Duration::from_secs(1),
///     node.transform.to(target, dur)
/// );
/// ```
pub fn delay(duration: Duration, inner: Box<dyn Animation>) -> Box<dyn Animation> {
    Box::new(Delay::new(duration, inner))
}
