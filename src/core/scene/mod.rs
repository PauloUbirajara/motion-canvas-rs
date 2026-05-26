use crate::core::animation::Node;
use kurbo::Affine;
#[cfg(feature = "runtime")]
use vello::Scene;

/// A trait for objects that can manage a 2D visual scene.
pub trait Scene2D {
    /// Renders all elements of the scene into a Vello scene.
    #[cfg(feature = "runtime")]
    fn render(&self, scene: &mut Scene);
    /// Advances the state of the scene by the given delta time.
    fn update(&mut self, dt: std::time::Duration);
    /// Returns a hash representing the current visual state of the entire scene.
    fn state_hash(&self) -> u64;
    /// Check if scene is dirty (has visual updates since last frame).
    fn is_dirty(&self) -> bool {
        true
    }
    /// Set the dirty flag.
    fn set_dirty(&mut self, _dirty: bool) {}
}

/// The standard implementation of a 2D scene, containing a collection of nodes
/// and timelines for video and audio animations.
pub struct BaseScene {
    /// The collection of visual nodes in the scene.
    pub nodes: Vec<crate::core::animation::AnyNode>,
    /// The primary timeline for video animations.
    pub video_timeline: crate::core::Timeline,
    /// The separate timeline for purely audio events.
    #[cfg(feature = "audio")]
    pub audio_timeline: crate::core::Timeline,
    // New fields:
}

impl BaseScene {
    /// Creates a new empty scene.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            video_timeline: crate::core::Timeline::new(),
            #[cfg(feature = "audio")]
            audio_timeline: crate::core::Timeline::new(),
        }
    }

    /// Adds a node to the scene.
    pub fn add(&mut self, node: impl Into<crate::core::animation::AnyNode>) {
        self.nodes.push(node.into());
    }

    /// Resets all timelines and nodes within the scene to their initial state.
    pub fn reset(&mut self) {
        self.video_timeline.reset();
        #[cfg(feature = "audio")]
        self.audio_timeline.reset();
        for node in &mut self.nodes {
            node.reset();
        }
    }

    /// Recursively collects audio events from all timelines.
    #[cfg(feature = "audio")]
    pub fn collect_audio_events(
        &mut self,
        current_time: std::time::Duration,
        events: &mut Vec<crate::core::animation::base::AudioEvent>,
    ) {
        self.video_timeline
            .collect_audio_events(current_time, events);
        self.audio_timeline
            .collect_audio_events(current_time, events);
    }
}

impl Scene2D for BaseScene {
    #[cfg(feature = "runtime")]
    fn render(&self, scene: &mut Scene) {
        for node in &self.nodes {
            node.render(scene, Affine::IDENTITY, 1.0);
        }
    }

    fn update(&mut self, dt: std::time::Duration) {
        self.video_timeline.update(dt);
        #[cfg(feature = "audio")]
        self.audio_timeline.update(dt);
        for node in &mut self.nodes {
            node.update(dt);
        }
    }

    fn state_hash(&self) -> u64 {
        use rayon::prelude::*;
        self.nodes
            .par_iter()
            .enumerate()
            .map(|(i, node)| crate::assets::hash::combine_hashes(node.state_hash(), i as u64))
            .reduce(|| 0u64, |a, b| a.wrapping_add(b))
    }
}

#[cfg(feature = "runtime")]
pub trait OffscreenRenderer {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn render_to_rgba(&self, scene: &vello::Scene) -> Vec<u8>;
}

#[cfg(feature = "runtime")]
use std::cell::RefCell;
#[cfg(feature = "runtime")]
use std::rc::Rc;

#[cfg(feature = "runtime")]
thread_local! {
    pub static ACTIVE_OFFSCREEN_RENDERER: RefCell<Option<Rc<dyn OffscreenRenderer>>> = const { RefCell::new(None) };
}

#[cfg(feature = "runtime")]
#[cfg(feature = "runtime")]
pub struct GpuOffscreenRenderer {
    device_ptr: *const vello::wgpu::Device,
    queue_ptr: *const vello::wgpu::Queue,
    width: u32,
    height: u32,
    renderer: std::cell::RefCell<vello::Renderer>,
    texture: vello::wgpu::Texture,
    texture_view: vello::wgpu::TextureView,
    output_buffer: vello::wgpu::Buffer,
    bytes_per_row: u32,
    unaligned_bytes_per_row: u32,
}

#[cfg(feature = "runtime")]
unsafe impl Send for GpuOffscreenRenderer {}
#[cfg(feature = "runtime")]
unsafe impl Sync for GpuOffscreenRenderer {}

#[cfg(feature = "runtime")]
impl GpuOffscreenRenderer {
    pub fn new(
        device: &vello::wgpu::Device,
        queue: &vello::wgpu::Queue,
        width: u32,
        height: u32,
        use_gpu: bool,
    ) -> Self {
        let renderer = vello::Renderer::new(
            device,
            vello::RendererOptions {
                surface_format: None,
                use_cpu: !use_gpu,
                antialiasing_support: vello::AaSupport::all(),
                num_init_threads: std::num::NonZeroUsize::new(1),
            },
        )
        .unwrap();

        // Allocate offscreen texture and output buffer
        let texture_desc = vello::wgpu::TextureDescriptor {
            label: Some("Offscreen Pass Texture"),
            size: vello::wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: vello::wgpu::TextureDimension::D2,
            format: vello::wgpu::TextureFormat::Rgba8Unorm,
            usage: vello::wgpu::TextureUsages::STORAGE_BINDING
                | vello::wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        };
        let texture = device.create_texture(&texture_desc);
        let texture_view = texture.create_view(&Default::default());

        let u32_size = std::mem::size_of::<u32>() as u32;
        let unaligned_bytes_per_row = width * u32_size;
        let align = vello::wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
        let padding = (align - unaligned_bytes_per_row % align) % align;
        let bytes_per_row = unaligned_bytes_per_row + padding;

        let output_buffer_desc = vello::wgpu::BufferDescriptor {
            size: (bytes_per_row * height) as vello::wgpu::BufferAddress,
            usage: vello::wgpu::BufferUsages::COPY_DST | vello::wgpu::BufferUsages::MAP_READ,
            label: Some("Offscreen Pass Buffer"),
            mapped_at_creation: false,
        };
        let output_buffer = device.create_buffer(&output_buffer_desc);

        Self {
            device_ptr: device as *const _,
            queue_ptr: queue as *const _,
            width,
            height,
            renderer: std::cell::RefCell::new(renderer),
            texture,
            texture_view,
            output_buffer,
            bytes_per_row,
            unaligned_bytes_per_row,
        }
    }
}

#[cfg(feature = "runtime")]
impl OffscreenRenderer for GpuOffscreenRenderer {
    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
    fn render_to_rgba(&self, scene: &vello::Scene) -> Vec<u8> {
        let mut renderer = self.renderer.borrow_mut();
        let device = unsafe { &*self.device_ptr };
        let queue = unsafe { &*self.queue_ptr };

        // 1. Render the sub-scene to the offscreen texture
        renderer
            .render_to_texture(
                device,
                queue,
                scene,
                &self.texture_view,
                &vello::RenderParams {
                    base_color: vello::peniko::Color::TRANSPARENT,
                    width: self.width,
                    height: self.height,
                    antialiasing_method: vello::AaConfig::Msaa16,
                },
            )
            .unwrap();

        // 2. Copy texture to buffer
        let mut encoder = device.create_command_encoder(&Default::default());
        encoder.copy_texture_to_buffer(
            vello::wgpu::ImageCopyTexture {
                aspect: vello::wgpu::TextureAspect::All,
                texture: &self.texture,
                mip_level: 0,
                origin: vello::wgpu::Origin3d::ZERO,
            },
            vello::wgpu::ImageCopyBuffer {
                buffer: &self.output_buffer,
                layout: vello::wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(self.bytes_per_row),
                    rows_per_image: Some(self.height),
                },
            },
            vello::wgpu::Extent3d {
                width: self.width,
                height: self.height,
                depth_or_array_layers: 1,
            },
        );
        queue.submit(Some(encoder.finish()));

        // 3. Map buffer and extract pixels
        let buffer_slice = self.output_buffer.slice(..);
        let (tx, rx) = std::sync::mpsc::channel();
        buffer_slice.map_async(vello::wgpu::MapMode::Read, move |res| tx.send(res).unwrap());
        device.poll(vello::wgpu::Maintain::Wait);
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
