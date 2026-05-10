use crate::core::scene::BaseScene;
use crate::core::scene::Scene2D;
use vello::peniko::Color;
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
const DEFAULT_PREVIEW_QUALITY: f32 = 0.9;
const DEFAULT_EXPORT_QUALITY: f32 = 4.0;

pub struct Project {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub title: String,
    pub scene: BaseScene,
    pub output_path: PathBuf,
    pub use_cache: bool,
    pub use_ffmpeg: bool,
    pub use_gpu: bool,
    pub background_color: Color,
    pub close_on_finish: bool,
    pub current_time: std::time::Duration,
    pub paused: bool,
    pub speed: f32,
    pub preview_quality: f32,
    pub export_quality: f32,
}

impl Project {
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
            preview_quality: DEFAULT_PREVIEW_QUALITY,
            export_quality: DEFAULT_EXPORT_QUALITY,
        }
    }
}

impl Default for Project {
    fn default() -> Self {
        Self::new(DEFAULT_WIDTH, DEFAULT_HEIGHT)
    }
}

impl Project {
    pub fn with_fps(mut self, fps: u32) -> Self {
        self.fps = fps;
        self
    }

    pub fn with_dimensions(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn with_output_path(mut self, path: &str) -> Self {
        self.output_path = PathBuf::from(path);
        self
    }

    pub fn with_cache(mut self, use_cache: bool) -> Self {
        self.use_cache = use_cache;
        self
    }

    pub fn with_ffmpeg(mut self, use_ffmpeg: bool) -> Self {
        self.use_ffmpeg = use_ffmpeg;
        self
    }

    pub fn with_gpu(mut self, use_gpu: bool) -> Self {
        self.use_gpu = use_gpu;
        self
    }

    pub fn with_background(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    pub fn with_close_on_finish(mut self, close: bool) -> Self {
        self.close_on_finish = close;
        self
    }

    pub fn with_preview_quality(mut self, quality: f32) -> Self {
        self.preview_quality = quality;
        self
    }

    pub fn with_export_quality(mut self, quality: f32) -> Self {
        self.export_quality = quality;
        self
    }

    pub fn close_on_finish(self) -> Self {
        self.with_close_on_finish(true)
    }

    pub fn seek_to(&mut self, target_time: std::time::Duration) {
        self.scene.reset();
        self.current_time = std::time::Duration::ZERO;
        let dt = std::time::Duration::from_secs_f32(1.0 / self.fps as f32);
        while self.current_time < target_time {
            self.scene.update(dt);
            self.current_time += dt;
        }
    }

    pub fn get_frame_name(&self, frame_index: u32) -> String {
        let sanitized = crate::assets::sanitize_title(&self.title);
        format!("{}_{:04}.png", sanitized, frame_index)
    }
}

