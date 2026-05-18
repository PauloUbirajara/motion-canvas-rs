use crate::core::animation::base::{Animation, AudioEvent};
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaybackState {
    Playing,
    Paused,
}

/// A linear container for sequential animations.
///
/// `Timeline` manages a sequence of boxed [`Animation`] traits, updating them
/// in order based on the current elapsed time. It is used by [`BaseScene`](crate::core::scene::BaseScene)
/// to orchestrate the overall animation flow.
pub struct Timeline {
    /// The list of sequential animations.
    pub animations: Vec<Box<dyn Animation>>,
    /// The current playback time within this timeline.
    pub current_time: Duration,

    // Playback state fields
    pub time: f32,
    pub state: PlaybackState,
    /// Force-flag the engine to run exactly one update/render pass
    /// even if virtual time didn't advance (crucial for seeks and steps).
    pub force_compile_frame: bool,
}

impl Timeline {
    /// Creates a new, empty timeline.
    pub fn new() -> Self {
        Self {
            animations: Vec::new(),
            current_time: Duration::ZERO,
            time: 0.0,
            state: PlaybackState::Playing,
            force_compile_frame: false,
        }
    }

    /// Appends an animation to the end of the timeline.
    pub fn add(&mut self, animation: Box<dyn Animation>) {
        self.animations.push(animation);
    }

    /// Advances the timeline by `dt`.
    ///
    /// This method identifies which animation(s) should be active during the
    /// provided time slice and updates them accordingly.
    pub fn update(&mut self, dt: Duration) {
        let mut total_time = Duration::ZERO;
        for anim in &mut self.animations {
            let dur = anim.duration();
            let start = total_time;
            let end = start + dur;

            if self.current_time < end {
                let overlap_start = self.current_time.max(start);
                let overlap_end = (self.current_time + dt).min(end);
                if overlap_end > overlap_start {
                    anim.update(overlap_end - overlap_start);
                }
            }
            total_time = end;
        }
        self.current_time += dt;
    }

    /// Returns the total duration of all animations in the timeline.
    pub fn duration(&self) -> Duration {
        self.animations.iter().map(|a| a.duration()).sum()
    }

    /// Returns true if the current time has reached or exceeded the total duration.
    pub fn finished(&self) -> bool {
        self.current_time >= self.duration()
    }

    /// Resets the timeline and all its contained animations to time zero.
    pub fn reset(&mut self) {
        self.current_time = Duration::ZERO;
        for anim in &mut self.animations {
            anim.reset();
        }
    }

    /// Recursively collects all audio events from contained animations.
    pub fn collect_audio_events(&mut self, _current_time: Duration, events: &mut Vec<AudioEvent>) {
        let mut total_offset = Duration::ZERO;
        for anim in &mut self.animations {
            anim.collect_audio_events(total_offset, events);
            total_offset += anim.duration();
        }
    }
}

impl Default for Timeline {
    fn default() -> Self {
        Self::new()
    }
}
