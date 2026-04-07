use vello::{
    util::{RenderContext, RenderSurface},
    Renderer, RendererOptions, Scene,
};
use winit::window::Window;

pub struct VelloRenderer<'a> {
    context: RenderContext,
    surface: Option<RenderSurface<'a>>,
    renderer: Option<Renderer>,
    scene: Scene,
}

impl<'a> VelloRenderer<'a> {
    pub fn new() -> Self {
        Self {
            context: RenderContext::new(),
            surface: None,
            renderer: None,
            scene: Scene::new(),
        }
    }

    pub async fn resume(&mut self, window: &'a Window) {
        let size = window.inner_size();
        let surface = self
            .context
            .create_surface(window, size.width, size.height, vello::wgpu::PresentMode::Fifo)
            .await
            .unwrap();
        
        let device_handle = &self.context.devices[surface.dev_id];
        let renderer = Renderer::new(
            &device_handle.device,
            RendererOptions {
                surface_format: Some(surface.format),
                use_cpu: false,
                antialiasing_support: vello::AaSupport::all(),
                num_init_threads: None,
            },
        )
        .unwrap();

        self.surface = Some(surface);
        self.renderer = Some(renderer);
    }

    pub fn render(&mut self, scene_2d: &dyn crate::engine::scene::Scene2D, width: u32, height: u32) {
        if let (Some(surface), Some(renderer)) = (&self.surface, &mut self.renderer) {
            self.scene.reset();
            scene_2d.render(&mut self.scene);

            let device_handle = &self.context.devices[surface.dev_id];
            let surface_texture = surface.surface.get_current_texture().unwrap();
            
            renderer
                .render_to_surface(
                    &device_handle.device,
                    &device_handle.queue,
                    &self.scene,
                    &surface_texture,
                    &vello::RenderParams {
                        base_color: vello::peniko::Color::BLACK,
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
