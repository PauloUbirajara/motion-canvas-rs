use std::time::Duration;
use vello::Scene;

#[derive(Clone, Debug)]
pub struct AudioEvent {
    pub path: String,
    pub volume: f32,
    pub start_crop: Duration,
    pub end_crop: Duration,
    pub start_time: Duration,
}

pub trait Animation: Send + Sync + 'static {
    /// Returns (finished, leftover_dt)
    fn update(&mut self, dt: Duration) -> (bool, Duration);
    fn duration(&self) -> Duration;
    fn set_easing(&mut self, _easing: fn(f32) -> f32) {}
    fn collect_audio_events(&mut self, _current_time: Duration, _events: &mut Vec<AudioEvent>) {}
    fn reset(&mut self);
}

impl<T: ?Sized + Animation> Animation for Box<T> {
    fn update(&mut self, dt: Duration) -> (bool, Duration) {
        (**self).update(dt)
    }
    fn duration(&self) -> Duration {
        (**self).duration()
    }
    fn set_easing(&mut self, easing: fn(f32) -> f32) {
        (**self).set_easing(easing)
    }
    fn collect_audio_events(&mut self, current_time: Duration, events: &mut Vec<AudioEvent>) {
        (**self).collect_audio_events(current_time, events)
    }
    fn reset(&mut self) {
        (**self).reset()
    }
}

pub trait Node: Send + Sync + 'static {
    fn render(
        &self,
        vello_scene: &mut Scene,
        parent_transform: vello::kurbo::Affine,
        parent_opacity: f32,
    );
    fn update(&mut self, dt: Duration);
    fn state_hash(&self) -> u64;
    fn clone_node(&self) -> Box<dyn Node>;
    fn reset(&mut self);
}
