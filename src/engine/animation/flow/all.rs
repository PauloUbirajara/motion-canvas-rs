use crate::engine::animation::base::{Animation, AudioEvent};
use std::time::Duration;

/// An animation that runs multiple animations in parallel.
///
/// It finishes when all child animations have finished.
pub struct All {
    pub(crate) animations: Vec<Box<dyn Animation>>,
    pub(crate) finished: Vec<bool>,
}

impl All {
    pub fn new(animations: Vec<Box<dyn Animation>>) -> Self {
        let len = animations.len();
        Self {
            animations,
            finished: vec![false; len],
        }
    }
}

impl Animation for All {
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

    fn duration(&self) -> Duration {
        self.animations
            .iter()
            .map(|a| a.duration())
            .max()
            .unwrap_or(Duration::ZERO)
    }

    fn set_easing(&mut self, easing: fn(f32) -> f32) {
        for anim in &mut self.animations {
            anim.set_easing(easing);
        }
    }

    fn collect_audio_events(&mut self, current_time: Duration, events: &mut Vec<AudioEvent>) {
        for anim in &mut self.animations {
            anim.collect_audio_events(current_time, events);
        }
    }

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
/// Generally used via the `all!` macro.
///
/// ### Example
/// ```rust
/// # use motion_canvas_rs::prelude::*;
/// # use std::time::Duration;
/// # let node = Rect::default().with_size(Vec2::new(100.0, 100.0)).with_fill(Color::RED);
/// all![
///     node.position.to(Vec2::new(100.0, 100.0), Duration::from_secs(1)),
///     node.fill_color.to(Color::BLUE, Duration::from_secs(1)),
/// ];
/// ```
pub fn all(animations: Vec<Box<dyn Animation>>) -> Box<dyn Animation> {
    Box::new(All::new(animations))
}
