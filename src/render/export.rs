use image::{ImageBuffer, Rgba};
use std::path::Path;
use vello::wgpu;
use vello::{util::RenderContext, Renderer, RendererOptions, Scene};

pub struct Exporter {
    width: u32,
    height: u32,
    context: RenderContext,
    device_id: usize,
    renderer: Renderer,
    scene: Scene,
}

impl Exporter {
    pub fn new(width: u32, height: u32) -> Self {
        let mut context = RenderContext::new();
        let device_id = pollster::block_on(context.device(None)).unwrap();
        let device_handle = &context.devices[device_id];

        let renderer = Renderer::new(
            &device_handle.device,
            RendererOptions {
                surface_format: None,
                use_cpu: false,
                antialiasing_support: vello::AaSupport::all(),
                num_init_threads: None,
            },
        )
        .unwrap();

        Self {
            width,
            height,
            context,
            device_id,
            renderer,
            scene: Scene::new(),
        }
    }

    pub fn export_frame(&mut self, scene_2d: &dyn crate::engine::scene::Scene2D, path: &Path) {
        let device_handle = &self.context.devices[self.device_id];
        let device = &device_handle.device;
        let queue = &device_handle.queue;

        // 1. Create a texture to render into
        let texture_desc = wgpu::TextureDescriptor {
            label: Some("Export Texture"),
            size: wgpu::Extent3d {
                width: self.width,
                height: self.height,
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

        // 2. Render the scene
        self.scene.reset();
        scene_2d.render(&mut self.scene);

        self.renderer
            .render_to_texture(
                device,
                queue,
                &self.scene,
                &texture_view,
                &vello::RenderParams {
                    base_color: vello::peniko::Color::BLACK,
                    width: self.width,
                    height: self.height,
                    antialiasing_method: vello::AaConfig::Msaa16,
                },
            )
            .unwrap();

        // 3. Copy texture to buffer (with alignment)
        let u32_size = std::mem::size_of::<u32>() as u32;
        let unaligned_bytes_per_row = self.width * u32_size;
        let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
        let padding = (align - unaligned_bytes_per_row % align) % align;
        let bytes_per_row = unaligned_bytes_per_row + padding;

        let output_buffer_desc = wgpu::BufferDescriptor {
            size: (bytes_per_row * self.height) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            label: Some("Export Buffer"),
            mapped_at_creation: false,
        };
        let output_buffer = device.create_buffer(&output_buffer_desc);

        let mut encoder = device.create_command_encoder(&Default::default());
        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::ImageCopyBuffer {
                buffer: &output_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(bytes_per_row),
                    rows_per_image: Some(self.height),
                },
            },
            texture_desc.size,
        );
        queue.submit(Some(encoder.finish()));

        // 4. Map buffer and save image
        let buffer_slice = output_buffer.slice(..);
        let (tx, rx) = std::sync::mpsc::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |res| tx.send(res).unwrap());
        device.poll(wgpu::Maintain::Wait);
        rx.recv().unwrap().unwrap();

        let data = buffer_slice.get_mapped_range();

        // Remove padding when creating the ImageBuffer
        let mut png_data = Vec::with_capacity((self.width * self.height * 4) as usize);
        for row in 0..self.height {
            let start = (row * bytes_per_row) as usize;
            let end = start + unaligned_bytes_per_row as usize;
            png_data.extend_from_slice(&data[start..end]);
        }

        let buffer: ImageBuffer<Rgba<u8>, _> =
            ImageBuffer::from_raw(self.width, self.height, png_data).unwrap();
        buffer.save(path).unwrap();

        drop(data);
        output_buffer.unmap();
    }
}
