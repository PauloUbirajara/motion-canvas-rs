use std::time::Duration;
use crate::core::animation::base::{Animation, AudioEvent};

pub struct Timeline {
    pub animations: Vec<Box<dyn Animation>>,
    pub current_time: Duration,
}

impl Timeline {
    pub fn new() -> Self {
        Self {
            animations: Vec::new(),
            current_time: Duration::ZERO,
        }
    }

    pub fn add(&mut self, animation: Box<dyn Animation>) {
        self.animations.push(animation);
    }

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

    pub fn duration(&self) -> Duration {
        self.animations
            .iter()
            .map(|a| a.duration())
            .sum()
    }

    pub fn finished(&self) -> bool {
        self.current_time >= self.duration()
    }

    pub fn reset(&mut self) {
        self.current_time = Duration::ZERO;
        for anim in &mut self.animations {
            anim.reset();
        }
    }

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
