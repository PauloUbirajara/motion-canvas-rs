use crate::engine::animation::base::{Animation, AudioEvent};
use std::time::Duration;

/// An animation that runs multiple animations in parallel, but with a staggered start time.
pub struct Sequence {
    pub(crate) items: Vec<(Duration, Box<dyn Animation>)>,
    pub(crate) finished: Vec<bool>,
    pub(crate) elapsed: Duration,
}

impl Sequence {
    pub fn new(stagger: Duration, animations: Vec<Box<dyn Animation>>) -> Self {
        let len = animations.len();
        let items = animations
            .into_iter()
            .enumerate()
            .map(|(i, anim)| (stagger * i as u32, anim))
            .collect();
        Self {
            items,
            finished: vec![false; len],
            elapsed: Duration::ZERO,
        }
    }
}

impl Animation for Sequence {
    fn update(&mut self, dt: Duration) -> (bool, Duration) {
        self.elapsed += dt;
        let mut all_finished = true;
        let mut min_leftover = dt;

        for (i, (start_time, anim)) in self.items.iter_mut().enumerate() {
            if self.finished[i] {
                continue;
            }

            if self.elapsed < *start_time {
                all_finished = false;
                continue;
            }

            let time_since_start = self.elapsed.saturating_sub(*start_time);
            if time_since_start == Duration::ZERO {
                all_finished = false;
                continue;
            }

            let item_dt = if time_since_start < dt {
                time_since_start
            } else {
                dt
            };

            let (finished, leftover) = anim.update(item_dt);
            if !finished {
                all_finished = false;
                continue;
            }

            self.finished[i] = true;
            min_leftover = min_leftover.min(leftover);
        }

        let total_finished = all_finished && self.elapsed >= self.duration();
        let final_leftover = if total_finished {
            self.elapsed.saturating_sub(self.duration())
        } else {
            Duration::ZERO
        };

        (total_finished, final_leftover)
    }

    fn duration(&self) -> Duration {
        self.items
            .iter()
            .map(|(start, anim)| *start + anim.duration())
            .max()
            .unwrap_or(Duration::ZERO)
    }

    fn set_easing(&mut self, easing: fn(f32) -> f32) {
        for (_, anim) in &mut self.items {
            anim.set_easing(easing);
        }
    }

    fn collect_audio_events(&mut self, current_time: Duration, events: &mut Vec<AudioEvent>) {
        for (start, anim) in &mut self.items {
            if self.elapsed >= *start {
                anim.collect_audio_events(current_time, events);
            }
        }
    }
}

/// Creates an animation that runs multiple animations in parallel with a staggered start.
///
/// Generally used via the `sequence!` macro.
///
/// ### Example
/// ```rust
/// # use motion_canvas_rs::prelude::*;
/// # use std::time::Duration;
/// # let node1 = Rect::default().with_size(Vec2::new(100.0, 100.0)).with_fill(Color::RED);
/// # let node2 = Rect::default().with_size(Vec2::new(100.0, 100.0)).with_fill(Color::RED);
/// # let node3 = Rect::default().with_size(Vec2::new(100.0, 100.0)).with_fill(Color::RED);
/// # let target = Vec2::new(100.0, 100.0);
/// # let dur = Duration::from_secs(1);
/// sequence!(
///     Duration::from_millis(100),
///     node1.position.to(target.clone(), dur),
///     node2.position.to(target.clone(), dur),
///     node3.position.to(target, dur)
/// );
/// ```
pub fn sequence(stagger: Duration, animations: Vec<Box<dyn Animation>>) -> Box<dyn Animation> {
    Box::new(Sequence::new(stagger, animations))
}
