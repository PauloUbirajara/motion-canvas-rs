use crate::engine::animation::base::{Animation, AudioEvent};
use std::time::Duration;

/// An animation that runs multiple animations sequentially.
pub struct Chain {
    pub(crate) animations: Vec<Box<dyn Animation>>,
    pub(crate) index: usize,
}

impl Chain {
    pub fn new(animations: Vec<Box<dyn Animation>>) -> Self {
        Self {
            animations,
            index: 0,
        }
    }
}

impl Animation for Chain {
    fn update(&mut self, mut dt: Duration) -> (bool, Duration) {
        while let Some(anim) = self.animations.get_mut(self.index) {
            let (finished, leftover) = anim.update(dt);
            if !finished {
                return (false, Duration::ZERO);
            }

            self.index += 1;
            dt = leftover;

            if dt == Duration::ZERO && self.index < self.animations.len() {
                return (false, Duration::ZERO);
            }
        }
        (true, dt)
    }

    fn duration(&self) -> Duration {
        self.animations
            .iter()
            .map(|a| a.duration())
            .fold(Duration::ZERO, |acc, d| acc + d)
    }

    fn set_easing(&mut self, easing: fn(f32) -> f32) {
        for anim in &mut self.animations {
            anim.set_easing(easing);
        }
    }

    fn collect_audio_events(&mut self, current_time: Duration, events: &mut Vec<AudioEvent>) {
        if self.index < self.animations.len() {
            self.animations[self.index].collect_audio_events(current_time, events);
        }
    }
}

/// Creates an animation that runs multiple animations one after another.
///
/// Generally used via the `chain!` macro.
///
/// ### Example
/// ```rust
/// # use motion_canvas_rs::prelude::*;
/// # use std::time::Duration;
/// # let node = Rect::default().with_size(Vec2::new(100.0, 100.0)).with_fill(Color::RED);
/// chain![
///     node.position.to(Vec2::new(100.0, 0.0), Duration::from_secs(1)),
///     node.position.to(Vec2::new(100.0, 100.0), Duration::from_secs(1)),
/// ];
/// ```
pub fn chain(animations: Vec<Box<dyn Animation>>) -> Box<dyn Animation> {
    Box::new(Chain::new(animations))
}
