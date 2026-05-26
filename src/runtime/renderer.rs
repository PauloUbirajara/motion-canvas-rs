use crate::core::scene::{OffscreenRenderer, Scene2D};
use indicatif::ProgressBar;
use std::future::Future;
use vello::{
    util::{RenderContext, RenderSurface},
    Renderer, RendererOptions, Scene,
};
use winit::window::Window;

/// High-level renderer that bridges the engine's `Scene2D` with Vello and wgpu.
///
/// `VelloRenderer` manages the GPU context, surfaces, and the core Vello renderer instance.
/// It is responsible for translating the declarative scene into drawing commands on the screen.
pub struct VelloRenderer {
    context: RenderContext,
    surface: Option<RenderSurface<'static>>,
    renderer: Option<Renderer>,
    scene: Scene,
    use_gpu: bool,
    background_color: vello::peniko::Color,
    offscreen_renderer: Option<std::rc::Rc<crate::core::scene::GpuOffscreenRenderer>>,
}

impl VelloRenderer {
    /// Creates a new renderer with the specified GPU preference and background color.
    pub fn new(use_gpu: bool, background_color: vello::peniko::Color) -> Self {
        Self {
            context: RenderContext::new(),
            surface: None,
            renderer: None,
            scene: Scene::new(),
            use_gpu,
            background_color,
            offscreen_renderer: None,
        }
    }

    /// (Re)initializes the rendering surface for a specific window.
    ///
    /// This is typically called when the window is first created or resumed (on Android/iOS).
    /// It performs an asynchronous surface creation and blocks until a GPU device is acquired.
    pub fn resume(&mut self, window: &Window, pb: &ProgressBar) {
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
                        num_init_threads: None,
                    },
                )
            })
            .unwrap();

        // Safety: We ensure the window outlives the renderer by having them both
        // owned by the same event loop closure.
        let surface_static =
            unsafe { std::mem::transmute::<RenderSurface<'_>, RenderSurface<'static>>(surface) };

        self.surface = Some(surface_static);
        self.renderer = Some(renderer);
    }

    pub fn render(&mut self, scene_2d: &mut dyn Scene2D, width: u32, height: u32) {
        if let (Some(surface), Some(renderer)) = (&self.surface, &mut self.renderer) {
            let device_handle = &self.context.devices[surface.dev_id];

            let recreate = match &self.offscreen_renderer {
                Some(r) => r.width() != width || r.height() != height,
                None => true,
            };
            if recreate {
                self.offscreen_renderer = Some(std::rc::Rc::new(
                    crate::core::scene::GpuOffscreenRenderer::new(
                        &device_handle.device,
                        &device_handle.queue,
                        width,
                        height,
                        self.use_gpu,
                    ),
                ));
            }

            // Bind offscreen renderer in the thread-local
            crate::core::scene::ACTIVE_OFFSCREEN_RENDERER.with(|cell| {
                *cell.borrow_mut() = self
                    .offscreen_renderer
                    .clone()
                    .map(|r| r as std::rc::Rc<dyn crate::core::scene::OffscreenRenderer>);
            });

            self.scene.reset();
            scene_2d.render(&mut self.scene);

            // Clean up offscreen renderer binding
            crate::core::scene::ACTIVE_OFFSCREEN_RENDERER.with(|cell| {
                *cell.borrow_mut() = None;
            });

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
