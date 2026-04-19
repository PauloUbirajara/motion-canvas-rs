use std::future::Future;
use vello::wgpu;
use vello::{util::RenderContext, Renderer, RendererOptions, Scene};

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
                num_init_threads: None,
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
    pub fn export_frame(&mut self, scene_2d: &dyn crate::engine::scene::Scene2D) -> Vec<u8> {
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
        let (tx, rx) = std::sync::mpsc::channel();
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
