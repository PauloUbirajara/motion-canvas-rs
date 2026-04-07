use crate::engine::scene::{BaseScene, Scene2D};
use crate::render::VelloRenderer;
use crate::render::export::Exporter;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use std::time::{Duration, Instant};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub struct Project {
    pub scene: BaseScene,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub window_title: String,
    pub output_path: PathBuf,
    pub frame_template: String,
}

impl Project {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            scene: BaseScene::new(),
            width,
            height,
            fps: 60,
            window_title: "Motion Canvas RS".to_string(),
            output_path: PathBuf::from("output"),
            frame_template: "frame_{:04}.png".to_string(),
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.window_title = title.to_string();
        self
    }

    pub fn with_output_path(mut self, path: &str) -> Self {
        self.output_path = PathBuf::from(path);
        self
    }

    pub fn with_fps(mut self, fps: u32) -> Self {
        self.fps = fps;
        self
    }

    pub fn with_frame_template(mut self, template: &str) -> Self {
        self.frame_template = template.to_string();
        self
    }

    pub fn export(&mut self) -> anyhow::Result<()> {
        println!("Exporting sequence ({}x{} @ {}fps) to {:?}...", 
            self.width, self.height, self.fps, self.output_path);
        
        let mut exporter = Exporter::new(self.width, self.height);
        let duration_secs = 2; // TODO: Calculate from timeline
        let total_frames = self.fps * duration_secs;
        let dt = Duration::from_secs_f32(1.0 / self.fps as f32);

        std::fs::create_dir_all(&self.output_path)?;
        
        // Move manifest to a hidden file in the project directory instead of the output folder
        let manifest_path = std::path::Path::new(".motion_canvas_cache");
        let mut manifest: HashMap<String, u64> = if manifest_path.exists() {
            let content = std::fs::read_to_string(&manifest_path)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            HashMap::new()
        };

        let mut rendered_count = 0;
        let mut skipped_count = 0;

        for i in 0..total_frames {
            let filename = self.frame_template.replace("{:04}", &format!("{:04}", i));
            let path = self.output_path.join(&filename);
            
            let current_hash = Scene2D::state_hash(&self.scene);
            let cached_hash = manifest.get(&filename);

            if path.exists() && cached_hash == Some(&current_hash) {
                skipped_count += 1;
            } else {
                exporter.export_frame(&self.scene, &path);
                manifest.insert(filename, current_hash);
                rendered_count += 1;
            }

            Scene2D::update(&mut self.scene, dt);
            
            if i % (total_frames / 10).max(1) == 0 {
                println!("Progress: {}/{} (Rendered: {}, Skipped: {})", i, total_frames, rendered_count, skipped_count);
            }
        }

        // Save manifest
        let manifest_json = serde_json::to_string_pretty(&manifest)?;
        std::fs::write(&manifest_path, manifest_json)?;

        println!("Export finished! Rendered: {}, Skipped: {}. Check the {:?} directory.", rendered_count, skipped_count, self.output_path);
        Ok(())
    }

    pub fn show(mut self) -> anyhow::Result<()> {
        let event_loop = EventLoop::new()?;
        let window = WindowBuilder::new()
            .with_title(&self.window_title)
            .with_inner_size(winit::dpi::LogicalSize::new(self.width as f64, self.height as f64))
            .build(&event_loop)?;

        let window: &'static winit::window::Window = Box::leak(Box::new(window));
        let mut renderer = VelloRenderer::new();
        pollster::block_on(renderer.resume(window));

        let mut last_update = Instant::now();

        event_loop.run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::Poll);

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::RedrawRequested => {
                        let size = window.inner_size();
                        renderer.render(&self.scene, size.width, size.height);
                    }
                    _ => {}
                },
                Event::AboutToWait => {
                    let now = Instant::now();
                    let dt = now - last_update;
                    last_update = now;

                    Scene2D::update(&mut self.scene, dt);
                    window.request_redraw();
                }
                _ => {}
            }
        })?;

        Ok(())
    }
}
