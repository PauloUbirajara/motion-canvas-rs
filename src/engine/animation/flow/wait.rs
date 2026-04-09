use crate::engine::animation::base::Animation;
use std::time::Duration;

/// An animation that simply waits for a duration.
///
/// Useful for creating pauses between other animations.
pub struct Wait {
    pub(crate) duration: Duration,
    pub(crate) elapsed: Duration,
}

impl Animation for Wait {
    fn update(&mut self, dt: Duration) -> (bool, Duration) {
        self.elapsed += dt;
        let finished = self.elapsed >= self.duration;
        let leftover = if finished {
            self.elapsed - self.duration
        } else {
            Duration::ZERO
        };
        (finished, leftover)
    }

    fn duration(&self) -> Duration {
        self.duration
    }
}

/// Creates an animation that waits for a duration.
///
/// Generally used via the `audio_wait!` macro in audio timelines or
/// directly as `wait(dur)` in video timelines.
///
/// ### Example
/// ```rust
/// # use motion_canvas_rs::prelude::*;
/// # use std::time::Duration;
/// # let node = nodes::Rect::new(Vec2::ZERO, Vec2::new(100.0, 100.0), Color::RED);
/// # let target = Affine::translate((100.0, 100.0));
/// # let dur = Duration::from_secs(1);
/// chain![
///     node.transform.to(target, dur),
///     wait(Duration::from_secs(1)), // Pause for 1s
///     node.opacity.to(0.0, dur),
/// ];
/// ```
pub fn wait(duration: Duration) -> Box<dyn Animation> {
    Box::new(Wait {
        duration,
        elapsed: Duration::ZERO,
    })
}
