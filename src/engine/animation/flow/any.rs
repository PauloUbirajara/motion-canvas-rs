use crate::engine::animation::base::{Animation, AudioEvent};
use std::time::Duration;

/// An animation that runs multiple animations in parallel, but finishes
/// as soon as ANY of the child animations finishes.
pub struct Any {
    pub(crate) animations: Vec<Box<dyn Animation>>,
}

impl Any {
    pub fn new(animations: Vec<Box<dyn Animation>>) -> Self {
        Self { animations }
    }
}

impl Animation for Any {
    fn update(&mut self, dt: Duration) -> (bool, Duration) {
        let mut any_finished = false;
        let mut max_leftover = Duration::ZERO;

        for anim in &mut self.animations {
            let (finished, leftover) = anim.update(dt);
            if finished {
                any_finished = true;
                max_leftover = max_leftover.max(leftover);
            }
        }

        (
            any_finished,
            if any_finished {
                max_leftover
            } else {
                Duration::ZERO
            },
        )
    }

    fn duration(&self) -> Duration {
        self.animations
            .iter()
            .map(|a| a.duration())
            .min()
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
    }
}

/// Creates an animation that runs multiple animations in parallel and finishes
/// when the first one completes.
///
/// Generally used via the `any!` macro.
///
/// ### Example
/// ```rust
/// # use motion_canvas_rs::prelude::*;
/// # use std::time::Duration;
/// # let node = Rect::default().with_size(Vec2::new(100.0, 100.0)).with_fill(Color::RED);
/// any![
///     node.position.to(Vec2::new(500.0, 500.0), Duration::from_secs(5)),
///     wait(Duration::from_secs(1)), // Finish after 1s regardless of position
/// ];
/// ```
pub fn any(animations: Vec<Box<dyn Animation>>) -> Box<dyn Animation> {
    Box::new(Any::new(animations))
}
