use crate::engine::scene::Scene2D;
use std::time::{Duration, Instant};
pub use vello::peniko::Color;
use vello::{
    util::{RenderContext, RenderSurface},
    Renderer, RendererOptions, Scene,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub mod export;

pub struct VelloRenderer<'a> {
    context: RenderContext,
    surface: Option<RenderSurface<'a>>,
    renderer: Option<Renderer>,
    scene: Scene,
    use_gpu: bool,
    background_color: vello::peniko::Color,
}

impl<'a> VelloRenderer<'a> {
    pub fn new(use_gpu: bool, background_color: vello::peniko::Color) -> Self {
        Self {
            context: RenderContext::new(),
            surface: None,
            renderer: None,
            scene: Scene::new(),
            use_gpu,
            background_color,
        }
    }

    pub async fn resume(&mut self, window: &'a winit::window::Window) {
        let size = window.inner_size();
        let surface = self
            .context
            .create_surface(
                window,
                size.width,
                size.height,
                vello::wgpu::PresentMode::Fifo,
            )
            .await
            .unwrap();

        let device_handle = &self.context.devices[surface.dev_id];
        let renderer = Renderer::new(
            &device_handle.device,
            RendererOptions {
                surface_format: Some(surface.format),
                use_cpu: !self.use_gpu,
                antialiasing_support: vello::AaSupport::all(),
                num_init_threads: None,
            },
        )
        .unwrap();

        self.surface = Some(surface);
        self.renderer = Some(renderer);
    }

    pub fn render(&mut self, scene_2d: &dyn Scene2D, width: u32, height: u32) {
        if let (Some(surface), Some(renderer)) = (&self.surface, &mut self.renderer) {
            self.scene.reset();
            scene_2d.render(&mut self.scene);

            let device_handle = &self.context.devices[surface.dev_id];
            let surface_texture = match surface.surface.get_current_texture() {
                Ok(t) => t,
                Err(_) => return, // Surface lost or outdated
            };

            renderer
                .render_to_surface(
                    &device_handle.device,
                    &device_handle.queue,
                    &self.scene,
                    &surface_texture,
                    &vello::RenderParams {
                        base_color: self.background_color,
                        width,
                        height,
                        antialiasing_method: vello::AaConfig::Msaa16,
                    },
                )
                .unwrap();

            surface_texture.present();
        }
    }
}

pub struct AnimationWindow {
    project: crate::engine::Project,
}

impl AnimationWindow {
    pub fn new(project: crate::engine::Project) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self { project })
    }

    pub fn run(mut self) -> Result<(), Box<dyn std::error::Error>> {
        let event_loop = EventLoop::new()?;
        let window = WindowBuilder::new()
            .with_title(&self.project.title)
            .with_inner_size(winit::dpi::LogicalSize::new(
                self.project.width,
                self.project.height,
            ))
            .build(&event_loop)?;

        let mut renderer = VelloRenderer::new(self.project.use_gpu, self.project.background_color);
        let mut last_update = Instant::now();
        let mut last_hash = 0u64;
        let mut finished = false;
        let dt = Duration::from_secs_f32(1.0 / self.project.fps as f32);

        // Capture window as a reference to avoid lifetime issues with Move
        let window_ref = unsafe {
            std::mem::transmute::<&winit::window::Window, &'static winit::window::Window>(&window)
        };

        event_loop.run(move |event, elwt| {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => elwt.exit(),

                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
                    renderer.render(&self.project.scene, self.project.width, self.project.height);
                }

                Event::AboutToWait => {
                    if finished {
                        elwt.set_control_flow(ControlFlow::Wait);
                        return;
                    }

                    let now = Instant::now();
                    let elapsed = now.duration_since(last_update);

                    if elapsed < dt {
                        elwt.set_control_flow(ControlFlow::WaitUntil(last_update + dt));
                        return;
                    }

                    // Process update
                    self.project.scene.update(dt);
                    last_update = now;

                    let current_hash = self.project.scene.state_hash();
                    if current_hash != last_hash {
                        window_ref.request_redraw();
                        last_hash = current_hash;
                    }

                    if self.project.scene.timeline.finished() {
                        println!("Animation finished.");
                        finished = true;

                        if self.project.close_on_finish {
                            elwt.exit();
                            return;
                        }

                        elwt.set_control_flow(ControlFlow::Wait);
                        return;
                    }

                    elwt.set_control_flow(ControlFlow::WaitUntil(now + dt));
                }

                Event::Resumed => {
                    pollster::block_on(renderer.resume(window_ref));
                }

                _ => (),
            }
        })?;

        Ok(())
    }
}
