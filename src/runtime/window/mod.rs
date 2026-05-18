use indicatif::{ProgressBar, ProgressStyle};
use std::{
    sync::LazyLock,
    time::{Duration, Instant},
};
use winit::{
    event::{Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};

use crate::core::scene::Scene2D;
use crate::runtime::renderer::VelloRenderer;
use crate::Project;

const SEEK_DURATION_SECS: u64 = 5;
const TUI_HEADER: &str = "--- motion-canvas-rs playback ---";
const TUI_CONTROLS: LazyLock<String> = LazyLock::new(|| {
    format!(
        r#"
Controls:
  R         : [R]estart
  Esc   / Q : [Q]uit
  Space / P : [P]ause / Resume

  . (Dot)       : Step +1 frame
  , (Comma)     : Step -1 frame
  > (Right) / L : Seek +{SEEK_DURATION_SECS}s
  < (Left)  / H : Seek -{SEEK_DURATION_SECS}s
  ^ (Up)    / K : Increase speed
  v (Down)  / J : Decrease speed (min 0.1x)
"#
    )
});

/// An interactive preview window for viewing animations.
///
/// `AnimationWindow` uses `winit` for window management and event handling,
/// providing real-time playback with a CLI progress bar (via `indicatif`).
pub struct AnimationWindow {
    project: Project,
    pb: ProgressBar,
}

impl AnimationWindow {
    /// Creates a new animation window for the given project.
    /// Initializes the TUI header and progress bar.
    pub fn new(project: Project) -> crate::Result<Self> {
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
        println!("{}", *TUI_CONTROLS);

        let pb = ProgressBar::new((total_duration.as_secs_f32() * 1000.0) as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{msg} [{bar:40.cyan/blue}] {pos}/{len}ms | {status}")
                .unwrap()
                .progress_chars("=>-"),
        );

        Ok(Self { project, pb })
    }

    /// Starts the interactive event loop.
    ///
    /// This method blocks the current thread until the window is closed or the animation finishes
    /// (if `close_on_finish` is set). It handles window resizing, keyboard input, and
    /// scheduled updates.
    pub fn run(mut self) -> crate::Result<()> {
        let event_loop = EventLoop::new()?;
        let window = WindowBuilder::new()
            .with_title(format!("{}", self.project.title,))
            .with_inner_size(winit::dpi::LogicalSize::new(
                self.project.width,
                self.project.height,
            ))
            .build(&event_loop)?;

        let mut renderer_opt: Option<VelloRenderer> = None;
        let mut last_update = Instant::now();
        let mut finished = false;
        let dt = Duration::from_secs_f32(1.0 / self.project.fps as f32);
        let mut time_accumulator = 0.0f32;

        self.project.timeline.state = if self.project.paused {
            crate::core::timeline::PlaybackState::Paused
        } else {
            crate::core::timeline::PlaybackState::Playing
        };
        self.project.timeline.time = self.project.current_time.as_secs_f32();

        event_loop.run(move |event, elwt| match event {
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
                        &mut self.project.scene,
                        self.project.width,
                        self.project.height,
                    );
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
            } => self.handle_keyboard_input(
                code,
                elwt,
                &window,
                &mut finished,
                &mut last_update,
                &mut time_accumulator,
                dt,
            ),

            Event::AboutToWait => self.handle_playback_update(
                elwt,
                &window,
                &mut last_update,
                &mut finished,
                &mut time_accumulator,
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
        time_accumulator: &mut f32,
        dt: Duration,
    ) {
        match code {
            KeyCode::Escape | KeyCode::KeyQ => {
                *finished = true;
                elwt.exit();
            }
            KeyCode::Space | KeyCode::KeyP => {
                self.project.paused = !self.project.paused;
                self.project.timeline.state = if self.project.paused {
                    crate::core::timeline::PlaybackState::Paused
                } else {
                    crate::core::timeline::PlaybackState::Playing
                };
                self.project.speed = 1.0;
            }
            KeyCode::ArrowRight | KeyCode::KeyL => {
                let target = self.project.current_time + Duration::from_secs(SEEK_DURATION_SECS);
                self.project.seek_to(target);
                self.project.timeline.time = target.as_secs_f32();
                self.project.timeline.force_compile_frame = true;
                *last_update = Instant::now();
                window.request_redraw();
            }
            KeyCode::ArrowLeft | KeyCode::KeyH => {
                let target = self
                    .project
                    .current_time
                    .saturating_sub(Duration::from_secs(SEEK_DURATION_SECS));
                self.project.seek_to(target);
                self.project.timeline.time = target.as_secs_f32();
                self.project.timeline.force_compile_frame = true;
                *last_update = Instant::now();
                window.request_redraw();
            }
            KeyCode::Period => {
                *time_accumulator += dt.as_secs_f32();
                self.project.timeline.force_compile_frame = true;
                *last_update = Instant::now();
                window.request_redraw();
            }
            KeyCode::Comma => {
                let target = self.project.current_time.saturating_sub(dt);
                self.project.seek_to(target);
                self.project.timeline.time = target.as_secs_f32();
                self.project.timeline.force_compile_frame = true;
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
                self.project.timeline.time = 0.0;
                self.project.timeline.force_compile_frame = true;
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
        finished: &mut bool,
        time_accumulator: &mut f32,
        dt: Duration,
    ) {
        if *finished {
            elwt.set_control_flow(ControlFlow::Wait);
            return;
        }

        let elapsed = last_update.elapsed();
        if elapsed < dt
            && !self.project.timeline.force_compile_frame
            && *time_accumulator < dt.as_secs_f32()
        {
            elwt.set_control_flow(ControlFlow::WaitUntil(*last_update + dt));
            return;
        }

        let real_dt = elapsed.as_secs_f32() * self.project.speed;
        *last_update = Instant::now();

        // 1. Only feed the accumulator if the timeline is actually playing
        if let crate::core::timeline::PlaybackState::Playing = self.project.timeline.state {
            *time_accumulator += real_dt;
        }

        let target_dt = dt.as_secs_f32();
        let mut ran_update = false;

        // 2. Consume accumulated time slices deterministically
        while *time_accumulator >= target_dt {
            self.project.scene.update(dt);
            self.project.timeline.time += target_dt;
            self.project.current_time += dt;
            *time_accumulator -= target_dt;
            ran_update = true;
        }

        let force_redraw = self.project.timeline.force_compile_frame;
        // 3. Handle Manual Overrides (Seeks / Frame Steps)
        if force_redraw && !ran_update {
            // Run exactly one evaluation tick to let reactive bindings update spatial properties
            self.project.scene.update(Duration::ZERO);
            self.project.timeline.force_compile_frame = false;
        }

        // Update indicatif progress bar
        let current_ms = (self.project.current_time.as_secs_f32() * 1000.0) as u64;
        self.pb.set_position(current_ms);

        let total_secs = self.project.current_time.as_secs() as u32;
        let hours = total_secs / 3600;
        let minutes = (total_secs % 3600) / 60;
        let seconds = total_secs % 60;
        let status_str = if self.project.paused {
            "PAUSED"
        } else {
            "PLAYING"
        };

        self.pb
            .set_message(format!("[{:02}:{:02}:{:02}]", hours, minutes, seconds));
        self.pb.set_style(
            ProgressStyle::default_bar()
                .template(&format!(
                    "{{msg}} [{{bar:40.cyan/blue}}] {{pos}}/{{len}}ms | Time: {:.2}s | Speed: {:.1}x | {}",
                    self.project.current_time.as_secs_f32(),
                    self.project.speed,
                    status_str
                ))
                .unwrap()
                .progress_chars("=>-"),
        );

        if ran_update || force_redraw {
            window.request_redraw();
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

/// Convenience function to run a project in a preview window.
pub fn run_window_session(project: Project) -> crate::Result<()> {
    let window = AnimationWindow::new(project)?;
    window.run()
}
