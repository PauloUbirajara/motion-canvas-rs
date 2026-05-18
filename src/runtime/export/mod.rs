use std::collections::HashMap;
use std::fs;
use std::future::Future;
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Duration;

use image::GenericImageView;
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use vello::wgpu;
use vello::{util::RenderContext, Renderer, RendererOptions, Scene};

use crate::core::scene::Scene2D;
use crate::Project;

/// A manifest representing the current state of exported frames.
///
/// Used by the caching system to avoid re-rendering frames that haven't changed.
#[derive(Serialize, Deserialize, Default)]
pub struct CacheManifest {
    /// The width of the frames in the cache.
    pub width: u32,
    /// The height of the frames in the cache.
    pub height: u32,
    /// A map from frame index to its state hash.
    pub frames: HashMap<u32, u64>, // frame_index -> state_hash
}

/// Headless renderer used for exporting scenes to raw image data.
///
/// `Exporter` handles the low-level wgpu buffer mapping and texture copies required
/// to extract high-quality frames from the GPU.
pub struct Exporter {
    width: u32,
    height: u32,
    context: RenderContext,
    device_id: usize,
    renderer: Renderer,
    scene: Scene,
    // Cached resources for export
    texture: wgpu::Texture,
    texture_view: wgpu::TextureView,
    output_buffer: wgpu::Buffer,
    bytes_per_row: u32,
    unaligned_bytes_per_row: u32,
    background_color: vello::peniko::Color,
}

impl Exporter {
    /// Initializes a new exporter with pre-allocated GPU resources.
    pub fn new(
        width: u32,
        height: u32,
        use_gpu: bool,
        background_color: vello::peniko::Color,
    ) -> Self {
        let mut context = RenderContext::new();
        let device_id: usize = {
            let mut future = std::pin::pin!(context.device(None));
            let waker = std::task::Waker::noop();
            let mut cx = std::task::Context::from_waker(&waker);

            loop {
                match future.as_mut().poll(&mut cx) {
                    std::task::Poll::Ready(val) => break val.unwrap(),
                    std::task::Poll::Pending => std::hint::spin_loop(),
                }
            }
        };

        let device_handle = &context.devices[device_id];
        let device = &device_handle.device;

        let renderer = Renderer::new(
            device,
            RendererOptions {
                surface_format: None,
                use_cpu: !use_gpu,
                antialiasing_support: vello::AaSupport::all(),
                num_init_threads: std::num::NonZeroUsize::new(1),
            },
        )
        .unwrap();

        // Pre-allocate texture
        let texture_desc = wgpu::TextureDescriptor {
            label: Some("Export Texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        };
        let texture = device.create_texture(&texture_desc);
        let texture_view = texture.create_view(&Default::default());

        // Pre-calculate alignment
        let u32_size = std::mem::size_of::<u32>() as u32;
        let unaligned_bytes_per_row = width * u32_size;
        let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
        let padding = (align - unaligned_bytes_per_row % align) % align;
        let bytes_per_row = unaligned_bytes_per_row + padding;

        let output_buffer_desc = wgpu::BufferDescriptor {
            size: (bytes_per_row * height) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            label: Some("Export Buffer"),
            mapped_at_creation: false,
        };
        let output_buffer = device.create_buffer(&output_buffer_desc);

        Self {
            width,
            height,
            context,
            device_id,
            renderer,
            scene: Scene::new(),
            texture,
            texture_view,
            output_buffer,
            bytes_per_row,
            unaligned_bytes_per_row,
            background_color,
        }
    }

    /// Renders a frame and returns the raw RGBA pixels.
    pub fn export_frame(&mut self, scene_2d: &dyn Scene2D) -> Vec<u8> {
        let device_handle = &self.context.devices[self.device_id];
        let device = &device_handle.device;
        let queue = &device_handle.queue;

        // 1. Render the scene
        self.scene.reset();
        scene_2d.render(&mut self.scene);

        self.renderer
            .render_to_texture(
                device,
                queue,
                &self.scene,
                &self.texture_view,
                &vello::RenderParams {
                    base_color: self.background_color,
                    width: self.width,
                    height: self.height,
                    antialiasing_method: vello::AaConfig::Msaa16,
                },
            )
            .unwrap();

        // 2. Copy texture to buffer
        let mut encoder = device.create_command_encoder(&Default::default());
        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::ImageCopyBuffer {
                buffer: &self.output_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(self.bytes_per_row),
                    rows_per_image: Some(self.height),
                },
            },
            wgpu::Extent3d {
                width: self.width,
                height: self.height,
                depth_or_array_layers: 1,
            },
        );
        queue.submit(Some(encoder.finish()));

        // 3. Map buffer and extract pixels
        let buffer_slice = self.output_buffer.slice(..);
        let (tx, rx) = mpsc::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |res| tx.send(res).unwrap());
        device.poll(wgpu::Maintain::Wait);
        rx.recv().unwrap().unwrap();

        let data = buffer_slice.get_mapped_range();

        let mut pixels = Vec::with_capacity((self.width * self.height * 4) as usize);
        if self.bytes_per_row == self.unaligned_bytes_per_row {
            pixels.extend_from_slice(&data[..(self.width * self.height * 4) as usize]);
        } else {
            for row in 0..self.height {
                let start = (row * self.bytes_per_row) as usize;
                let end = start + self.unaligned_bytes_per_row as usize;
                pixels.extend_from_slice(&data[start..end]);
            }
        }

        drop(data);
        self.output_buffer.unmap();

        pixels
    }
}

/// Exports the project to a sequence of PNG frames and optionally encodes them to a video file.
///
/// `run_export_session` iterates through every frame of the project's timeline,
/// rendering them at full resolution. It supports:
/// - **Caching**: Skips re-rendering if the frame's `state_hash` hasn't changed.
/// - **Background Saving**: Saves PNGs in parallel using a background thread to avoid blocking the GPU.
/// - **FFmpeg Integration**: Streams raw frames directly to FFmpeg for high-speed video encoding.
#[cfg(feature = "export")]
pub fn run_export_session(project: &mut Project) -> crate::Result<()> {
    println!("Exporting project: {}", project.title);
    fs::create_dir_all(&project.output_path)?;

    let sanitized = crate::assets::sanitize_title(&project.title);
    let cache_file = project
        .output_path
        .join(format!(".motion_canvas_cache_{}", sanitized));
    let mut manifest: CacheManifest = (project.use_cache && cache_file.exists())
        .then(|| fs::read_to_string(&cache_file).ok())
        .flatten()
        .and_then(|c| serde_json::from_str(&c).ok())
        .filter(|m: &CacheManifest| m.width == project.width && m.height == project.height)
        .unwrap_or(CacheManifest {
            width: project.width,
            height: project.height,
            frames: HashMap::new(),
        });

    let mut audio_handler = crate::assets::audio::create_audio_handler();
    audio_handler.setup();

    let mut exporter = Exporter::new(
        project.width,
        project.height,
        project.use_gpu,
        project.background_color,
    );
    let dt = Duration::from_secs_f32(1.0 / project.fps as f32);
    let mut frame_count = 0;
    let mut rendered_count = 0;
    let mut skipped_count = 0;

    let video_duration = project.scene.video_timeline.duration();
    let audio_duration = audio_handler.get_duration(&project.scene);
    let total_duration = video_duration.max(audio_duration);
    let total_frames = (total_duration.as_secs_f32() * project.fps as f32).ceil() as u32;

    // Use rayon for background PNG saving
    let (tx, rx) = mpsc::channel::<(Vec<u8>, PathBuf)>();
    let width = project.width;
    let height = project.height;
    let saved_count = Arc::new(AtomicU32::new(0));
    let saved_count_clone = saved_count.clone();

    // Initialize FFmpeg if requested
    let mut ffmpeg_process = project
        .use_ffmpeg
        .then(|| {
            crate::assets::export::start_ffmpeg(
                &project.title,
                width,
                height,
                project.fps,
                audio_handler.has_audio(),
            )
            .map_err(|e| {
                eprintln!("Failed to start FFmpeg: {}. Falling back to PNGs.", e);
                e
            })
            .ok()
            .flatten()
        })
        .flatten();

    let saving_thread = thread::spawn(move || {
        while let Ok((pixels, path)) = rx.recv() {
            let buffer: image::ImageBuffer<image::Rgba<u8>, _> =
                image::ImageBuffer::from_raw(width, height, pixels).unwrap();
            buffer.save(path).unwrap();
            saved_count_clone.fetch_add(1, Ordering::SeqCst);
        }
    });

    let pb = ProgressBar::new(total_frames as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("=>-"),
    );

    // Export until all animations are finished
    loop {
        let hash = project.scene.state_hash();
        let sanitized = crate::assets::sanitize_title(&project.title);
        let frame_name = format!("{}_{:04}.png", sanitized, frame_count);
        let frame_path = project.output_path.join(frame_name);

        // Check cache
        let is_cached = project.use_cache
            && manifest.frames.get(&frame_count) == Some(&hash)
            && frame_path.exists();

        if is_cached {
            skipped_count += 1;
            saved_count.fetch_add(1, Ordering::SeqCst);
            // If we are skipping, we still need to feed FFmpeg the frame if it's open
            if let Some(ref mut stdin) = ffmpeg_process {
                match image::open(&frame_path) {
                    Ok(img) => {
                        let (w, h) = img.dimensions();
                        if w == project.width && h == project.height {
                            let pixels = img.to_rgba8().into_raw();
                            stdin.write_all(&pixels)?;
                        } else {
                            eprintln!("\nWarning: Cached frame resolution mismatch at {:?} (expected {}x{}, found {}x{}). Re-rendering...", frame_path, project.width, project.height, w, h);
                            let pixels = exporter.export_frame(&project.scene);
                            stdin.write_all(&pixels)?;
                            tx.send((pixels, frame_path)).unwrap();
                            rendered_count += 1;
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "\nCache corruption detected at {:?}: {}. Re-rendering...",
                            frame_path, e
                        );
                        let pixels = exporter.export_frame(&project.scene);
                        stdin.write_all(&pixels)?;
                        tx.send((pixels, frame_path)).unwrap();
                        rendered_count += 1;
                    }
                }
            }
        } else {
            let pixels = exporter.export_frame(&project.scene);

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
        pb.set_position(current_saved as u64);
        pb.set_message(format!("(Skipped {})", skipped_count));

        let current_time = Duration::from_secs_f32(frame_count as f32 / project.fps as f32);
        audio_handler.collect_events(&mut project.scene, current_time);

        let is_video_finished = project.scene.video_timeline.finished();
        let is_audio_finished = audio_handler.is_finished(&project.scene);

        if is_video_finished && is_audio_finished {
            break;
        }

        project.scene.update(dt);
        frame_count += 1;
    }

    // Clean up
    drop(tx);

    // Wait for all frames to be saved while updating the progress bar
    while saved_count.load(Ordering::SeqCst) < frame_count + 1 {
        let current_saved = saved_count.load(Ordering::SeqCst);
        pb.set_position(current_saved as u64);
        thread::sleep(Duration::from_millis(50));
    }

    pb.finish_with_message(format!(
        "Export finished: {} frames rendered, {} skipped.",
        rendered_count, skipped_count
    ));

    saving_thread.join().unwrap();
    if let Some(stdin) = ffmpeg_process {
        drop(stdin); // Flush and close FFmpeg pipe
    }

    // Save updated cache
    if project.use_cache {
        let json = serde_json::to_string_pretty(&manifest)?;
        fs::write(&cache_file, json)?;
    }

    audio_handler.finish(&project.title, project.use_ffmpeg)?;
    Ok(())
}
