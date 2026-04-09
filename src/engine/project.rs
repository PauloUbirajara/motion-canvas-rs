use crate::engine::scene::BaseScene;
#[cfg(feature = "export")]
use crate::engine::scene::Scene2D;
use crate::render::AnimationWindow;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "export")]
use std::fs;
#[cfg(feature = "export")]
use std::io::{self, Write};
#[cfg(feature = "export")]
use std::path::Path;
use std::path::PathBuf;
use vello::peniko::Color;

const DEFAULT_FPS: u32 = 60;
const DEFAULT_WIDTH: u32 = 800;
const DEFAULT_HEIGHT: u32 = 600;
const DEFAULT_TITLE: &str = "motion-canvas-rs";
const DEFAULT_OUTPUT_PATH: &str = "output";
const DEFAULT_BACKGROUND_COLOR: Color = Color::rgb8(0x1a, 0x1a, 0x1a);
const DEFAULT_USE_CACHE: bool = true;
const DEFAULT_USE_GPU: bool = true;
const DEFAULT_USE_FFMPEG: bool = false;

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
    pub use_cache: bool,
    pub use_ffmpeg: bool,
    pub use_gpu: bool,
    pub background_color: Color,
    pub close_on_finish: bool,
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

    pub fn close_on_finish(self) -> Self {
        self.with_close_on_finish(true)
    }

    pub fn export(&mut self) -> crate::Result<()> {
        #[cfg(not(feature = "export"))]
        return Err("Export failed: 'export' feature is disabled.".into());

        #[cfg(feature = "export")]
        {
            println!("Exporting project: {}", self.title);
            fs::create_dir_all(&self.output_path)?;

            let cache_file = Path::new(".motion_canvas_cache");
            let mut manifest: CacheManifest = (self.use_cache && cache_file.exists())
                .then(|| fs::read_to_string(cache_file).ok())
                .flatten()
                .and_then(|c| serde_json::from_str(&c).ok())
                .unwrap_or_default();

            let mut exporter = crate::render::export::Exporter::new(
                self.width,
                self.height,
                self.use_gpu,
                self.background_color,
            );
            let dt = std::time::Duration::from_secs_f32(1.0 / self.fps as f32);
            let mut frame_count = 0;
            let mut rendered_count = 0;
            let mut skipped_count = 0;

            let mut audio_events = Vec::new();
            let video_duration = self.scene.video_timeline.duration();
            let audio_duration = {
                #[cfg(feature = "audio")]
                {
                    self.scene.audio_timeline.duration()
                }
                #[cfg(not(feature = "audio"))]
                {
                    std::time::Duration::ZERO
                }
            };
            let total_duration = video_duration.max(audio_duration);
            let total_frames = (total_duration.as_secs_f32() * self.fps as f32).ceil() as u32;

            // Use rayon for background PNG saving
            let (tx, rx) = std::sync::mpsc::channel::<(Vec<u8>, PathBuf)>();
            let width = self.width;
            let height = self.height;
            use std::sync::atomic::{AtomicU32, Ordering};
            let saved_count = std::sync::Arc::new(AtomicU32::new(0));
            let saved_count_clone = saved_count.clone();

            // Initialize FFmpeg if requested
            let mut ffmpeg_process = self.use_ffmpeg.then(|| {
                crate::engine::util::export::start_ffmpeg(
                    &self.title,
                    width,
                    height,
                    self.fps,
                    cfg!(feature = "audio")
                ).map_err(|e| {
                    eprintln!("Failed to start FFmpeg: {}. Falling back to PNGs.", e);
                    e
                }).ok().flatten()
            }).flatten();

            let saving_thread = std::thread::spawn(move || {
                while let Ok((pixels, path)) = rx.recv() {
                    let buffer: image::ImageBuffer<image::Rgba<u8>, _> =
                        image::ImageBuffer::from_raw(width, height, pixels).unwrap();
                    buffer.save(path).unwrap();
                    saved_count_clone.fetch_add(1, Ordering::SeqCst);
                }
            });

            // Export until all animations are finished
            loop {
                let hash = self.scene.state_hash();
                let frame_name = self.get_frame_name(frame_count);
                let frame_path = self.output_path.join(frame_name);

                // Check cache
                let is_cached = self.use_cache
                    && manifest.frames.get(&frame_count) == Some(&hash)
                    && frame_path.exists();

                if is_cached {
                    skipped_count += 1;
                    saved_count.fetch_add(1, Ordering::SeqCst);
                    // If we are skipping, we still need to feed FFmpeg the frame if it's open
                    if let Some(ref mut stdin) = ffmpeg_process {
                        let pixels = image::open(&frame_path).unwrap().to_rgba8().into_raw();
                        stdin.write_all(&pixels)?;
                    }
                } else {
                    let pixels = exporter.export_frame(&self.scene);

                    // Write to FFmpeg if active
                    if let Some(ref mut stdin) = ffmpeg_process {
                        stdin.write_all(&pixels)?;
                    }

                    // Send to background PNG saver
                    tx.send((pixels, frame_path)).unwrap();
                    manifest.frames.insert(frame_count, hash);
                    rendered_count += 1;
                }

                // Progress Bar (now reflects saved count)
                let current_saved = saved_count.load(Ordering::SeqCst);
                let progress = if total_frames > 0 {
                    (current_saved as f32 / total_frames as f32).min(1.0)
                } else {
                    1.0
                };
                let bar_len = 20;
                let filled = (progress * bar_len as f32) as usize;
                let bar: String = std::iter::repeat('=')
                    .take(filled)
                    .chain(std::iter::once('>'))
                    .chain(std::iter::repeat(' ').take(bar_len - filled))
                    .collect();

                print!(
                    "\r[Exporting] Frame {}/{} [{}] {:.0}% (Skipped {})",
                    current_saved.min(total_frames),
                    total_frames,
                    bar,
                    progress * 100.0,
                    skipped_count
                );
                io::stdout().flush()?;

                let is_video_finished = self.scene.video_timeline.finished();
                let is_audio_finished = {
                    #[cfg(feature = "audio")]
                    {
                        self.scene.audio_timeline.finished()
                    }
                    #[cfg(not(feature = "audio"))]
                    {
                        true
                    }
                };

                if is_video_finished && is_audio_finished {
                    break;
                }

                #[cfg(feature = "audio")]
                {
                    let current_time =
                        std::time::Duration::from_secs_f32(frame_count as f32 / self.fps as f32);
                    self.scene
                        .collect_audio_events(current_time, &mut audio_events);
                }

                self.scene.update(dt);
                frame_count += 1;
            }

            // Clean up
            drop(tx);

            // Wait for all frames to be saved while updating the progress bar
            while saved_count.load(Ordering::SeqCst) < frame_count + 1 {
                let current_saved = saved_count.load(Ordering::SeqCst);
                let progress = if total_frames > 0 {
                    (current_saved as f32 / total_frames as f32).min(1.0)
                } else {
                    1.0
                };
                let bar_len = 20;
                let filled = (progress * bar_len as f32) as usize;
                let bar: String = std::iter::repeat('=')
                    .take(filled)
                    .chain(std::iter::once('>'))
                    .chain(std::iter::repeat(' ').take(bar_len - filled))
                    .collect();

                print!(
                    "\r[Exporting] Frame {}/{} [{}] {:.0}% (Skipped {})",
                    current_saved.min(total_frames),
                    total_frames,
                    bar,
                    progress * 100.0,
                    skipped_count
                );
                io::stdout().flush()?;
                std::thread::sleep(std::time::Duration::from_millis(50));
            }

            saving_thread.join().unwrap();
            if let Some(stdin) = ffmpeg_process {
                drop(stdin); // Flush and close FFmpeg pipe
            }

            // Save updated cache
            if self.use_cache {
                let json = serde_json::to_string_pretty(&manifest)?;
                fs::write(cache_file, json)?;
            }

            println!(
                "\nExport finished: {} frames rendered, {} skipped.",
                rendered_count, skipped_count
            );

            #[cfg(feature = "audio")]
            if self.use_ffmpeg {
                crate::engine::util::export::merge_audio(&self.title, &audio_events)?;
            }

            Ok(())
        }
    }

    pub fn show(self) -> crate::Result<()> {
        let window = AnimationWindow::new(self)?;
        window.run()
    }

    fn sanitize_title(&self) -> String {
        crate::engine::util::export::sanitize_title(&self.title)
    }

    pub fn get_frame_name(&self, frame_count: u32) -> String {
        let sanitized = self.sanitize_title();
        format!("{}_{:04}.png", sanitized, frame_count)
    }
}
