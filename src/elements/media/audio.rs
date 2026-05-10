#![cfg(feature = "audio")]
use crate::core::animation::Animation;
use std::time::Duration;

/// Configuration for an audio asset.
///
/// `AudioNode` defines the parameters for playing an audio file, such as its
/// volume and cropping. It is used to create `AudioAnimation` instances.
///
/// ### Example
/// ```rust
/// # use motion_canvas_rs::prelude::*;
/// # use std::time::Duration;
/// let music = AudioNode::default()
///     .with_path("assets/bgm.mp3")
///     .with_volume(0.5)
///     .with_start(Duration::from_secs(10));
/// ```
#[derive(Clone, Debug)]
pub struct AudioNode {
    /// The filesystem path to the audio file.
    pub path: String,
    /// The volume multiplier (1.0 is original).
    pub volume: f32,
    /// The amount of audio to skip from the beginning.
    pub start_crop: Duration,
    /// The amount of audio to ignore at the end.
    pub end_crop: Duration,
}

impl AudioNode {
    /// Creates a new audio configuration for the given file path.
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
            volume: 1.0,
            start_crop: Duration::ZERO,
            end_crop: Duration::ZERO,
        }
    }

    /// Sets the volume multiplier.
    pub fn with_volume(mut self, volume: f32) -> Self {
        self.volume = volume;
        self
    }

    /// Sets the start crop (skip duration).
    pub fn with_start(mut self, offset: Duration) -> Self {
        self.start_crop = offset;
        self
    }

    /// Sets the end crop (cutoff from end).
    pub fn with_end(mut self, offset: Duration) -> Self {
        self.end_crop = offset;
        self
    }
}

/// A utility for managing audio assets and durations.
pub struct AudioManager;

impl AudioManager {
    /// Attempts to retrieve the total duration of an audio file.
    pub fn get_duration(path: &str) -> Option<Duration> {
        let file = File::open(path).ok()?;
        let decoder = Decoder::new(BufReader::new(file)).ok()?;
        decoder.total_duration()
    }
}

use lazy_static::lazy_static;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Source};
use std::fs::File;
use std::io::BufReader;

lazy_static! {
    static ref AUDIO_HANDLE: OutputStreamHandle = {
        let (stream, handle) =
            OutputStream::try_default().expect("Failed to initialize audio output stream");
        std::mem::forget(stream); // Keep the stream alive forever
        handle
    };
    static ref AUDIO_PLAYBACK_ENABLED: AtomicBool = AtomicBool::new(true);
}

/// Enables or disables real-time audio playback during preview.
pub fn set_audio_playback(enabled: bool) {
    AUDIO_PLAYBACK_ENABLED.store(enabled, Ordering::SeqCst);
}

use std::sync::atomic::{AtomicBool, Ordering};

/// An animation that triggers audio playback on the timeline.
///
/// `AudioAnimation` wraps an `AudioNode` and manages its lifecycle during
/// project playback. It handles both real-time preview (via `rodio`) and
/// event collection for exporting.
///
/// Generally created via the `play!` macro.
pub struct AudioAnimation {
    /// The underlying audio configuration.
    pub node: AudioNode,
    /// The time elapsed since this animation started.
    pub elapsed: Duration,
    /// Internal flag to track if playback has started.
    pub started: AtomicBool,
    /// Total duration of the audio file (cached).
    pub total_duration: Duration,
    /// Internal flag to track if the event has been recorded for exporting.
    pub recorded: bool,
}

impl AudioAnimation {
    /// Creates a new audio animation from a node configuration.
    pub fn new(node: AudioNode) -> Self {
        let total_duration =
            AudioManager::get_duration(&node.path).unwrap_or(Duration::from_secs(1)); // Fallback if duration is unknown

        Self {
            node,
            elapsed: Duration::ZERO,
            started: AtomicBool::new(false),
            total_duration,
            recorded: false,
        }
    }

    fn play_audio(&self) {
        let path = self.node.path.clone();
        let volume = self.node.volume;
        let start_crop = self.node.start_crop;
        let end_crop = self.node.end_crop;
        let total_dur = self.total_duration;

        std::thread::spawn(move || {
            let file = match File::open(&path) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Failed to open audio file {}: {}", path, e);
                    return;
                }
            };

            let source = match Decoder::new(BufReader::new(file)) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Failed to decode audio file {}: {}", path, e);
                    return;
                }
            };

            let play_duration = total_dur
                .checked_sub(start_crop)
                .and_then(|d| d.checked_sub(end_crop))
                .unwrap_or(Duration::ZERO);

            if play_duration > Duration::ZERO {
                let source = source
                    .skip_duration(start_crop)
                    .take_duration(play_duration)
                    .amplify(volume);

                let _ = AUDIO_HANDLE.play_raw(source.convert_samples());
                std::thread::sleep(play_duration);
            }
        });
    }
}

impl Animation for AudioAnimation {
    fn update(&mut self, dt: Duration) -> (bool, Duration) {
        if !self.started.load(Ordering::SeqCst) {
            self.started.store(true, Ordering::SeqCst);
            if AUDIO_PLAYBACK_ENABLED.load(Ordering::SeqCst) {
                self.play_audio();
            }
        }

        self.elapsed += dt;

        let play_duration = self
            .total_duration
            .checked_sub(self.node.start_crop)
            .and_then(|d| d.checked_sub(self.node.end_crop))
            .unwrap_or(Duration::ZERO);

        let finished = self.elapsed >= play_duration;
        let leftover = self.elapsed.saturating_sub(play_duration);
        (finished, leftover)
    }

    fn duration(&self) -> Duration {
        self.total_duration
            .checked_sub(self.node.start_crop)
            .and_then(|d| d.checked_sub(self.node.end_crop))
            .unwrap_or(Duration::ZERO)
    }

    fn collect_audio_events(
        &mut self,
        current_time: Duration,
        events: &mut Vec<crate::core::animation::base::AudioEvent>,
    ) {
        if self.recorded {
            return;
        }

        self.recorded = true;
        events.push(crate::core::animation::base::AudioEvent {
            path: self.node.path.clone(),
            volume: self.node.volume,
            start_crop: self.node.start_crop,
            end_crop: self.node.end_crop,
            start_time: if current_time > self.elapsed {
                current_time - self.elapsed
            } else {
                Duration::ZERO
            },
        });
    }

    fn reset(&mut self) {
        self.elapsed = Duration::ZERO;
        self.started.store(false, Ordering::SeqCst);
        self.recorded = false;
    }
}

impl From<AudioNode> for Box<dyn Animation> {
    fn from(node: AudioNode) -> Self {
        Box::new(AudioAnimation::new(node))
    }
}
