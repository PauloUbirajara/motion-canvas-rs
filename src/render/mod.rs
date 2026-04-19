use crate::engine::scene::Scene2D;
use std::time::{Duration, Instant};
pub use vello::peniko::Color;
use vello::{
    util::{RenderContext, RenderSurface},
    Renderer, RendererOptions, Scene,
};
use winit::{
    event::{Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};

const TUI_HEADER: &str = "--- motion-canvas-rs playback ---";
const TUI_CONTROLS: &str = r#"
Controls:
  Esc / Q      : [Q]uit
  R            : [R]estart
  Space / P    : [P]ause / Resume
  .            : Step +1 frame
  ,            : Step -1 frame
  Right / L    : Seek +10s
  Left / H     : Seek -10s
  Up / K       : Increase speed
  Down / J     : Decrease speed (min 0.1x)
"#;
const TUI_FOOTER: &str = "---------------------------------";

pub mod export;
use std::future::Future;

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

    pub fn resume(&mut self, window: &'a Window) {
        let size = window.inner_size();
        let surface: RenderSurface = {
            let mut future = std::pin::pin!(self.context.create_surface(
                window,
                size.width,
                size.height,
                vello::wgpu::PresentMode::Fifo,
            ));
            let waker = std::task::Waker::noop();
            let mut cx = std::task::Context::from_waker(&waker);

            loop {
                match future.as_mut().poll(&mut cx) {
                    std::task::Poll::Ready(val) => break val.unwrap(),
                    std::task::Poll::Pending => std::hint::spin_loop(),
                }
            }
        };

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

        let mut renderer_opt: Option<VelloRenderer> = None;
        let mut last_update = Instant::now();
        let mut last_hash = 0u64;
        let mut finished = false;
        let dt = Duration::from_secs_f32(1.0 / self.project.fps as f32);

        event_loop.run(|event, elwt| {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => elwt.exit(),

                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
                    if let Some(ref mut renderer) = renderer_opt {
                        renderer.render(
                            &self.project.scene,
                            self.project.width,
                            self.project.height,
                        );
                    }
                }

                Event::WindowEvent {
                    event: WindowEvent::KeyboardInput {
                        event: KeyEvent {
                            physical_key: PhysicalKey::Code(code),
                            state: winit::event::ElementState::Pressed,
                            ..
                        },
                        ..
                    },
                    ..
                } => self.handle_keyboard_input(
                    code,
                    elwt,
                    &window,
                    &mut finished,
                    &mut last_update,
                    dt,
                ),

                Event::AboutToWait => self.handle_playback_update(
                    elwt,
                    &window,
                    &mut last_update,
                    &mut last_hash,
                    &mut finished,
                    dt,
                ),

                Event::Resumed => {
                    let renderer = renderer_opt.get_or_insert_with(|| {
                        VelloRenderer::new(self.project.use_gpu, self.project.background_color)
                    });
                    renderer.resume(&window);
                }

                _ => (),
            }
        })?;

        Ok(())
    }

    fn handle_keyboard_input(
        &mut self,
        code: KeyCode,
        elwt: &winit::event_loop::EventLoopWindowTarget<()>,
        window: &Window,
        finished: &mut bool,
        last_update: &mut Instant,
        dt: Duration,
    ) {
        match code {
            KeyCode::Escape | KeyCode::KeyQ => {
                *finished = true;
                elwt.exit();
            }
            KeyCode::Space | KeyCode::KeyP => {
                self.project.paused = !self.project.paused;
            }
            KeyCode::ArrowRight | KeyCode::KeyL => {
                let target = self.project.current_time + Duration::from_secs(10);
                self.project.seek_to(target);
                *last_update = Instant::now();
                window.request_redraw();
            }
            KeyCode::ArrowLeft | KeyCode::KeyH => {
                let target = self
                    .project
                    .current_time
                    .saturating_sub(Duration::from_secs(10));
                self.project.seek_to(target);
                *last_update = Instant::now();
                window.request_redraw();
            }
            KeyCode::Period => {
                let target = self.project.current_time + dt;
                self.project.seek_to(target);
                *last_update = Instant::now();
                window.request_redraw();
            }
            KeyCode::Comma => {
                let target = self.project.current_time.saturating_sub(dt);
                self.project.seek_to(target);
                *last_update = Instant::now();
                window.request_redraw();
            }
            KeyCode::ArrowUp | KeyCode::KeyK => {
                self.project.speed += 0.5;
            }
            KeyCode::ArrowDown | KeyCode::KeyJ => {
                self.project.speed = (self.project.speed - 0.5).max(0.1);
            }
            KeyCode::KeyR => {
                self.project.seek_to(Duration::ZERO);
                *finished = false;
                *last_update = Instant::now();
                window.request_redraw();
            }
            _ => (),
        }
    }

    fn handle_playback_update(
        &mut self,
        elwt: &winit::event_loop::EventLoopWindowTarget<()>,
        window: &Window,
        last_update: &mut Instant,
        last_hash: &mut u64,
        finished: &mut bool,
        dt: Duration,
    ) {
        if *finished {
            elwt.set_control_flow(ControlFlow::Wait);
            return;
        }

        let mut elapsed = last_update.elapsed();
        if elapsed < dt {
            elwt.set_control_flow(ControlFlow::WaitUntil(*last_update + dt));
            return;
        }

        // Process all pending updates (catch-up)
        if !self.project.paused {
            let effective_dt = dt.mul_f32(self.project.speed);
            while elapsed >= dt {
                self.project.scene.update(effective_dt);
                self.project.current_time += effective_dt;
                elapsed -= dt;
                *last_update += dt;
            }
        } else {
            *last_update = Instant::now();
            self.project.speed = 1.0;
        }

        // Minimal TUI: Print status
        print!("\x1B[2J\x1B[H");
        println!("{}", TUI_HEADER);
        println!("{}", TUI_CONTROLS);
        println!("{}", TUI_FOOTER);
        println!(
            "[Playback] Time: {:.2}s | Speed: {:.1}x | {}",
            self.project.current_time.as_secs_f32(),
            self.project.speed,
            if self.project.paused {
                "PAUSED "
            } else {
                "PLAYING"
            }
        );
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let current_hash = self.project.scene.state_hash();
        if current_hash != *last_hash {
            window.request_redraw();
            *last_hash = current_hash;
        }

        let is_video_finished = self.project.scene.video_timeline.finished();
        let is_audio_finished = {
            #[cfg(feature = "audio")]
            {
                self.project.scene.audio_timeline.finished()
            }
            #[cfg(not(feature = "audio"))]
            {
                true
            }
        };

        if is_video_finished && is_audio_finished {
            println!("Animation finished.");
            *finished = true;

            if self.project.close_on_finish {
                elwt.exit();
                return;
            }

            elwt.set_control_flow(ControlFlow::Wait);
            return;
        }

        elwt.set_control_flow(ControlFlow::WaitUntil(*last_update + dt));
    }
}
