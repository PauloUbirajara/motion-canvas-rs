use crate::core::scene::BaseScene;
use crate::core::scene::Scene2D;
use peniko::Color;
use std::path::PathBuf;

const DEFAULT_FPS: u32 = 60;
const DEFAULT_WIDTH: u32 = 800;
const DEFAULT_HEIGHT: u32 = 600;
const DEFAULT_TITLE: &str = "motion-canvas-rs";
const DEFAULT_OUTPUT_PATH: &str = "output";
const DEFAULT_BACKGROUND_COLOR: Color = Color::rgb8(0x1a, 0x1a, 0x1a);
const DEFAULT_USE_CACHE: bool = true;
const DEFAULT_USE_GPU: bool = true;
const DEFAULT_USE_FFMPEG: bool = false;

/// The central configuration and state for a motion canvas animation.
///
/// `Project` stores all metadata about the animation, including dimensions,
/// frame rate, and export settings. It also owns the [`BaseScene`] which
/// contains the actual animation nodes.
pub struct Project {
    /// Target width of the animation.
    pub width: u32,
    /// Target height of the animation.
    pub height: u32,
    /// Frames per second.
    pub fps: u32,
    /// Human-readable title of the project.
    pub title: String,
    /// The root scene orchestration node.
    pub scene: BaseScene,
    /// Directory where exported frames and videos will be saved.
    pub output_path: PathBuf,
    /// Whether to use frame-level caching during export.
    pub use_cache: bool,
    /// Whether to automatically attempt FFmpeg encoding after export.
    pub use_ffmpeg: bool,
    /// Whether to use GPU acceleration for rendering.
    pub use_gpu: bool,
    /// Background clear color for the animation.
    pub background_color: Color,
    /// If true, the playback window will close automatically when the animation ends.
    pub close_on_finish: bool,
    /// The current playback/export time.
    pub current_time: std::time::Duration,
    /// Playback state (paused or playing).
    pub paused: bool,
    /// Playback speed multiplier.
    pub speed: f32,
    /// The master playback timeline state.
    pub timeline: crate::core::Timeline,
}

impl Project {
    /// Creates a new project with the given dimensions and default settings.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            fps: DEFAULT_FPS,
            title: DEFAULT_TITLE.to_string(),
            scene: BaseScene::new(),
            output_path: PathBuf::from(DEFAULT_OUTPUT_PATH),
            use_cache: DEFAULT_USE_CACHE,
            use_ffmpeg: DEFAULT_USE_FFMPEG,
            use_gpu: DEFAULT_USE_GPU,
            background_color: DEFAULT_BACKGROUND_COLOR,
            close_on_finish: false,
            current_time: std::time::Duration::ZERO,
            paused: false,
            speed: 1.0,
            timeline: crate::core::Timeline::new(),
        }
    }
}

impl Default for Project {
    fn default() -> Self {
        Self::new(DEFAULT_WIDTH, DEFAULT_HEIGHT)
    }
}

impl Project {
    /// Sets the target frames per second.
    pub fn with_fps(mut self, fps: u32) -> Self {
        self.fps = fps;
        self
    }

    /// Sets the width and height of the animation.
    pub fn with_dimensions(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Sets the project title.
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Sets the output directory for exports.
    pub fn with_output_path(mut self, path: &str) -> Self {
        self.output_path = PathBuf::from(path);
        self
    }

    /// Enables or disables frame-level caching.
    pub fn with_cache(mut self, use_cache: bool) -> Self {
        self.use_cache = use_cache;
        self
    }

    /// Enables or disables automatic FFmpeg encoding.
    pub fn with_ffmpeg(mut self, use_ffmpeg: bool) -> Self {
        self.use_ffmpeg = use_ffmpeg;
        self
    }

    /// Enables or disables GPU acceleration.
    pub fn with_gpu(mut self, use_gpu: bool) -> Self {
        self.use_gpu = use_gpu;
        self
    }

    /// Sets the background clear color.
    pub fn with_background(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    /// Sets whether the window should close automatically on finish.
    pub fn with_close_on_finish(mut self, close: bool) -> Self {
        self.close_on_finish = close;
        self
    }

    /// Convenience method to enable automatic window closing on finish.
    pub fn close_on_finish(self) -> Self {
        self.with_close_on_finish(true)
    }

    /// Resets the scene and advances it to the specified target time.
    ///
    /// This is used for seeking in the interactive preview.
    pub fn seek_to(&mut self, target_time: std::time::Duration) {
        self.scene.reset();
        self.current_time = std::time::Duration::ZERO;
        let dt = std::time::Duration::from_secs_f32(1.0 / self.fps as f32);
        while self.current_time < target_time {
            self.scene.update(dt);
            self.current_time += dt;
        }
    }

    /// Returns the sanitized filename for a specific frame index.
    pub fn get_frame_name(&self, frame_index: u32) -> String {
        let sanitized = crate::assets::sanitize_title(&self.title);
        format!("{}_{:04}.png", sanitized, frame_index)
    }
}
