use crate::core::scene::BaseScene;
use crate::Result;
use std::time::Duration;

/// Interface for handling audio orchestration during preview or export.
///
/// `AudioHandler` abstracts the differences between real audio playback (using `rodio`)
/// and headless audio processing (for export).
pub trait AudioHandler {
    /// Performs any necessary initialization (e.g., disabling live playback during export).
    fn setup(&mut self);
    /// Returns the total duration of the audio timeline.
    fn get_duration(&self, scene: &BaseScene) -> Duration;
    /// Collects audio events from the scene for the current timestamp.
    fn collect_events(&mut self, scene: &mut BaseScene, current_time: Duration);
    /// Returns true if the audio timeline has finished.
    fn is_finished(&self, scene: &BaseScene) -> bool;
    /// Returns true if this handler is capable of processing audio.
    fn has_audio(&self) -> bool;
    /// Finalizes audio processing (e.g., merging audio into a video file via FFmpeg).
    fn finish(&self, title: &str, use_ffmpeg: bool) -> Result<()>;
}

/// Factory function to create an appropriate audio handler based on active features.
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
    events: Vec<crate::core::animation::base::AudioEvent>,
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
        crate::elements::media::audio::set_audio_playback(false);
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
            crate::assets::export::merge_audio(title, &self.events)?;
        }
        #[cfg(not(feature = "export"))]
        let _ = (title, use_ffmpeg);

        crate::elements::media::audio::set_audio_playback(true);
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
