use crate::engine::scene::{BaseScene, Scene2D};
use crate::render::VelloRenderer;
use crate::render::export::Exporter;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use std::time::{Duration, Instant};

pub struct Project {
    pub scene: BaseScene,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
}

impl Project {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            scene: BaseScene::new(),
            width,
            height,
            fps: 60,
        }
    }

    pub fn run(mut self) -> anyhow::Result<()> {
        let args: Vec<String> = std::env::args().collect();
        let export_mode = args.contains(&"--export".to_string());

        if export_mode {
            self.export()
        } else {
            self.show()
        }
    }

    fn export(&mut self) -> anyhow::Result<()> {
        println!("Exporting PNG sequence ({}x{} @ {}fps)...", self.width, self.height, self.fps);
        let mut exporter = Exporter::new(self.width, self.height);
        let duration_secs = 2; // TODO: Calculate from timeline
        let total_frames = self.fps * duration_secs;
        let dt = Duration::from_secs_f32(1.0 / self.fps as f32);

        std::fs::create_dir_all("output")?;

        for i in 0..total_frames {
            let path = format!("output/frame_{:04}.png", i);
            exporter.export_frame(&self.scene, std::path::Path::new(&path));
            Scene2D::update(&mut self.scene, dt);
            if i % 10 == 0 {
                println!("Progress: {}/{}", i, total_frames);
            }
        }
        println!("Export finished! Check the 'output' directory.");
        Ok(())
    }

    fn show(mut self) -> anyhow::Result<()> {
        let event_loop = EventLoop::new()?;
        let window = WindowBuilder::new()
            .with_title("Motion Canvas RS")
            .with_inner_size(winit::dpi::LogicalSize::new(self.width as f64, self.height as f64))
            .build(&event_loop)?;

        // Safety: We leak the window to keep it alive for the duration of the app.
        // This is a common pattern in winit 0.29 for 'static lifetime requirements.
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
