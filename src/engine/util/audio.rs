use std::time::Duration;
use crate::engine::scene::BaseScene;
use crate::Result;

pub trait AudioHandler {
    fn setup(&mut self);
    fn get_duration(&self, scene: &BaseScene) -> Duration;
    fn collect_events(&mut self, scene: &mut BaseScene, current_time: Duration);
    fn is_finished(&self, scene: &BaseScene) -> bool;
    fn has_audio(&self) -> bool;
    fn finish(&self, title: &str, use_ffmpeg: bool) -> Result<()>;
}

pub fn create_audio_handler() -> Box<dyn AudioHandler> {
    #[cfg(feature = "audio")]
    {
        Box::new(RealAudioHandler::new())
    }
    #[cfg(not(feature = "audio"))]
    {
        Box::new(NoopAudioHandler)
    }
}

#[cfg(feature = "audio")]
struct RealAudioHandler {
    events: Vec<crate::engine::animation::base::AudioEvent>,
}

#[cfg(feature = "audio")]
impl RealAudioHandler {
    fn new() -> Self {
        Self { events: Vec::new() }
    }
}

#[cfg(feature = "audio")]
impl AudioHandler for RealAudioHandler {
    fn setup(&mut self) {
        crate::engine::nodes::audio::set_audio_playback(false);
    }
    fn get_duration(&self, scene: &BaseScene) -> Duration {
        scene.audio_timeline.duration()
    }
    fn collect_events(&mut self, scene: &mut BaseScene, current_time: Duration) {
        scene.collect_audio_events(current_time, &mut self.events);
    }
    fn is_finished(&self, scene: &BaseScene) -> bool {
        scene.audio_timeline.finished()
    }
    fn has_audio(&self) -> bool {
        true
    }
    fn finish(&self, title: &str, use_ffmpeg: bool) -> Result<()> {
        #[cfg(feature = "export")]
        if use_ffmpeg {
            crate::engine::util::export::merge_audio(title, &self.events)?;
        }
        #[cfg(not(feature = "export"))]
        let _ = (title, use_ffmpeg);

        crate::engine::nodes::audio::set_audio_playback(true);
        Ok(())
    }
}

#[cfg(not(feature = "audio"))]
struct NoopAudioHandler;

#[cfg(not(feature = "audio"))]
impl AudioHandler for NoopAudioHandler {
    fn setup(&mut self) {}
    fn get_duration(&self, _scene: &BaseScene) -> Duration {
        Duration::ZERO
    }
    fn collect_events(&mut self, _scene: &mut BaseScene, _current_time: Duration) {}
    fn is_finished(&self, _scene: &BaseScene) -> bool {
        true
    }
    fn has_audio(&self) -> bool {
        false
    }
    fn finish(&self, _title: &str, _use_ffmpeg: bool) -> Result<()> {
        Ok(())
    }
}
