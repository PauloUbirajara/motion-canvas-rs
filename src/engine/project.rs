use std::path::PathBuf;
use crate::render::AnimationWindow;
use crate::engine::scene::{BaseScene, Scene2D};
use std::fs;
use std::collections::HashMap;
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default)]
pub struct CacheManifest {
    pub frames: HashMap<u32, u64>, // frame_index -> state_hash
}

pub struct Project {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub title: String,
    pub scene: BaseScene,
    pub output_path: PathBuf,
    pub frame_template: String,
    pub use_cache: bool,
}

impl Project {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            fps: 60,
            title: "Motion Canvas Rust".to_string(),
            scene: BaseScene::new(),
            output_path: PathBuf::from("output"),
            frame_template: "frame_{:04}.png".to_string(),
            use_cache: true, // Cache is now enabled by default
        }
    }

    pub fn with_fps(mut self, fps: u32) -> Self {
        self.fps = fps;
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

    pub fn with_frame_template(mut self, template: &str) -> Self {
        self.frame_template = template.to_string();
        self
    }

    pub fn export(&mut self) -> anyhow::Result<()> {
        println!("Exporting project: {}", self.title);
        fs::create_dir_all(&self.output_path)?;

        let cache_file = Path::new(".motion_canvas_cache");
        let mut manifest = if self.use_cache && cache_file.exists() {
            let content = fs::read_to_string(cache_file)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            CacheManifest::default()
        };

        let mut exporter = crate::render::export::Exporter::new(self.width, self.height);
        let dt = std::time::Duration::from_secs_f32(1.0 / self.fps as f32);
        let mut frame_count = 0;

        // Export until all animations are finished
        loop {
            let hash = self.scene.state_hash();
            let frame_path = self.output_path.join(format!("frame_{:04}.png", frame_count));

            // Check cache: skip if hash matches AND file exists
            if self.use_cache && manifest.frames.get(&frame_count) == Some(&hash) && frame_path.exists() {
                // Skip rendering
            } else {
                exporter.export_frame(&self.scene, &frame_path);
                manifest.frames.insert(frame_count, hash);
            }

            if self.scene.timeline.finished() {
                break;
            }
            self.scene.update(dt);
            frame_count += 1;
        }

        // Save updated cache
        if self.use_cache {
            let json = serde_json::to_string_pretty(&manifest)?;
            fs::write(cache_file, json)?;
        }

        println!("Export finished: {} frames rendered/skipped.", frame_count + 1);
        Ok(())
    }

    pub fn show(self) -> anyhow::Result<()> {
        let window = AnimationWindow::new(self)?;
        window.run()
    }
}
