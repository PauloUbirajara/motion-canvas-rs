use crate::core::animation::base::{Animation, AudioEvent};
use crate::core::animation::AnyAnimation;
use std::time::Duration;

/// An animation that runs multiple animations sequentially.
///
/// `Chain` executes its child animations one by one. Only when the current
/// animation finishes does it move to the next one in the list.
pub struct Chain {
    pub(crate) animations: Vec<AnyAnimation>,
    pub(crate) index: usize,
}

impl Chain {
    /// Creates a new `Chain` container with the provided animations.
    pub fn new(animations: Vec<AnyAnimation>) -> Self {
        Self {
            animations,
            index: 0,
        }
    }
}

impl Animation for Chain {
    /// Updates the current child animation.
    /// If the child finishes, it uses the leftover time to start the next one.
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

    /// The duration is the sum of all child animation durations.
    fn duration(&self) -> Duration {
        self.animations
            .iter()
            .map(|a| a.duration())
            .fold(Duration::ZERO, |acc, d| acc + d)
    }

    /// Propagates the easing function to all child animations.
    fn set_easing(&mut self, easing: fn(f32) -> f32) {
        for anim in &mut self.animations {
            anim.set_easing(easing);
        }
    }

    /// Collects audio events only from the currently active child animation with proper relative offset.
    fn collect_audio_events(&mut self, current_time: Duration, events: &mut Vec<AudioEvent>) {
        if self.index < self.animations.len() {
            let preceding_dur: Duration = self.animations[..self.index]
                .iter()
                .map(|a| a.duration())
                .sum();
            self.animations[self.index].collect_audio_events(current_time + preceding_dur, events);
        }
    }

    /// Resets all child animations and restarts from the first one.
    fn reset(&mut self) {
        for anim in &mut self.animations {
            anim.reset();
        }
        self.index = 0;
    }
}

/// Creates an animation that runs multiple animations one after another.
///
/// Generally used via the [`chain!`](crate::chain) macro.
///
/// ### Example
/// ```rust
/// # use motion_canvas_rs::prelude::*;
/// # use std::time::Duration;
/// # let node = Rect::default()
/// #    .with_size(Vec2::new(100.0, 100.0))
/// #    .with_fill(Color::RED);
/// # let target1 = Vec2::new(100.0, 0.0);
/// # let target2 = Vec2::new(100.0, 100.0);
/// # let dur = Duration::from_secs(1);
/// chain![
///     node.position.to(target1, dur),
///     node.position.to(target2, dur),
/// ];
/// ```
pub fn chain(animations: Vec<AnyAnimation>) -> AnyAnimation {
    AnyAnimation::Chain(Chain::new(animations))
}
