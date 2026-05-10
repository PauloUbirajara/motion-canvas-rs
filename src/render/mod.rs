use crate::engine::scene::Scene2D;
use std::time::{Duration, Instant};
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
use indicatif::{ProgressBar, ProgressStyle};

const TUI_HEADER: &str = "--- motion-canvas-rs playback ---";
const TUI_CONTROLS: &str = r#"
Controls:
  R         : [R]estart
  Esc   / Q : [Q]uit
  Space / P : [P]ause / Resume

  . (Dot)       : Step +1 frame
  , (Comma)     : Step -1 frame
  > (Right) / L : Seek +10s
  < (Left)  / H : Seek -10s
  ^ (Up)    / K : Increase speed
  v (Down)  / J : Decrease speed (min 0.1x)
"#;
#[cfg(feature = "export")]
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

    pub fn resume(&mut self, window: &'a Window, pb: &ProgressBar) {
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
        let renderer = pb
            .suspend(|| {
                Renderer::new(
                    &device_handle.device,
                    RendererOptions {
                        surface_format: Some(surface.format),
                        use_cpu: !self.use_gpu,
                        antialiasing_support: vello::AaSupport::all(),
                        num_init_threads: std::num::NonZeroUsize::new(1),
                    },
                )
            })
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
    pb: ProgressBar,
}

impl AnimationWindow {
    pub fn new(project: crate::engine::Project) -> Result<Self, Box<dyn std::error::Error>> {
        let video_duration = project.scene.video_timeline.duration();
        let audio_duration = {
            #[cfg(feature = "audio")]
            {
                project.scene.audio_timeline.duration()
            }
            #[cfg(not(feature = "audio"))]
            {
                Duration::ZERO
            }
        };
        let total_duration = video_duration.max(audio_duration);

        println!("{}", TUI_HEADER);
        println!("{}", TUI_CONTROLS);

        let pb = ProgressBar::new((total_duration.as_secs_f32() * 1000.0) as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len}ms | {msg}")
                .unwrap()
                .progress_chars("=>-"),
        );

        pb.set_message("Initializing...");
        pb.enable_steady_tick(Duration::from_millis(100));
        Ok(Self { project, pb })
    }

    pub fn run(mut self) -> Result<(), Box<dyn std::error::Error>> {
        let event_loop = EventLoop::new()?;
        let window = WindowBuilder::new()
            .with_title(format!(
                "{} (Preview Quality: {:.1}x)",
                self.project.title, self.project.preview_quality
            ))
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

        event_loop.run(|event, elwt| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => elwt.exit(),

            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                if let Some(ref mut renderer) = renderer_opt {
                    renderer.render(&self.project.scene, self.project.width, self.project.height);
                }
            }

            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                physical_key: PhysicalKey::Code(code),
                                state: winit::event::ElementState::Pressed,
                                ..
                            },
                        ..
                    },
                ..
            } => {
                self.handle_keyboard_input(code, elwt, &window, &mut finished, &mut last_update, dt)
            }

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
                renderer.resume(&window, &self.pb);
            }

            _ => (),
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
                self.project.speed = 1.0;
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
                self.project.speed = 1.0;
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
        }

        // Update indicatif progress bar
        self.pb
            .set_position((self.project.current_time.as_secs_f32() * 1000.0) as u64);

        let status = if self.project.paused {
            "PAUSED "
        } else {
            "PLAYING"
        };
        self.pb.set_message(format!(
            "Time: {:.2}s | Speed: {:.1}x | {}",
            self.project.current_time.as_secs_f32(),
            self.project.speed,
            status
        ));

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
            self.pb.finish_with_message("Animation finished.");
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
