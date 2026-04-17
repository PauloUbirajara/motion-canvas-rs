use crate::engine::animation::{Node, Signal};
use glam::Vec2;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use vello::kurbo::Affine;
use vello::peniko::{Blob, Extend, Format, Image as PenikoImage};
use vello::Scene;

lazy_static! {
    static ref IMAGE_CACHE: Mutex<HashMap<String, Arc<PenikoImage>>> = Mutex::new(HashMap::new());
}

pub struct ImageManager;

impl ImageManager {
    pub fn get_image(path: &str) -> Option<Arc<PenikoImage>> {
        let mut cache = IMAGE_CACHE.lock().unwrap();
        if let Some(img) = cache.get(path) {
            return Some(img.clone());
        }

        #[cfg(feature = "svg")]
        if path.ends_with(".svg") {
            let svg_data = std::fs::read(path).ok()?;
            let opt = usvg::Options::default();
            let tree = usvg::Tree::from_data(&svg_data, &opt).ok()?;

            let size = tree.size();
            let mut pixmap =
                resvg::tiny_skia::Pixmap::new(size.width() as u32, size.height() as u32)?;
            resvg::render(
                &tree,
                resvg::tiny_skia::Transform::default(),
                &mut pixmap.as_mut(),
            );

            let data = Arc::new(pixmap.take());
            let peniko_img = Arc::new(PenikoImage {
                data: Blob::new(data),
                format: Format::Rgba8,
                width: size.width() as u32,
                height: size.height() as u32,
                extend: Extend::Pad,
            });
            cache.insert(path.to_string(), peniko_img.clone());
            return Some(peniko_img);
        }

        #[cfg(not(feature = "svg"))]
        if path.ends_with(".svg") {
            eprintln!("Error: SVG support is disabled. Enable the 'svg' feature to load '{}'", path);
            return None;
        }

        // Load raster image from disk
        match image::open(path) {
            Ok(img) => {
                let rgba = img.to_rgba8();
                let (width, height) = rgba.dimensions();
                let data = Arc::new(rgba.into_raw());
                let peniko_img = Arc::new(PenikoImage {
                    data: Blob::new(data),
                    format: Format::Rgba8,
                    width,
                    height,
                    extend: Extend::Pad,
                });
                cache.insert(path.to_string(), peniko_img.clone());
                return Some(peniko_img);
            }
            Err(e) => {
                eprintln!("Error: Failed to load image at '{}': {}", path, e);
            }
        }

        None
    }
}

#[derive(Clone)]
pub struct ImageNode {
    pub position: Signal<Vec2>,
    pub rotation: Signal<f32>,
    pub scale: Signal<Vec2>,
    pub size: Signal<Vec2>,
    pub image: Option<Arc<PenikoImage>>,
    pub opacity: Signal<f32>,
}

impl Default for ImageNode {
    fn default() -> Self {
        Self {
            position: Signal::new(Vec2::ZERO),
            rotation: Signal::new(0.0),
            scale: Signal::new(Vec2::ONE),
            size: Signal::new(Vec2::ZERO),
            image: None,
            opacity: Signal::new(1.0),
        }
    }
}

impl ImageNode {
    pub fn new(pos: Vec2, path: &str) -> Self {
        Self::default().with_position(pos).with_path(path)
    }

    pub fn with_position(mut self, position: Vec2) -> Self {
        self.position = Signal::new(position);
        self
    }

    pub fn with_rotation(mut self, angle: f32) -> Self {
        self.rotation = Signal::new(angle);
        self
    }

    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = Signal::new(Vec2::splat(scale));
        self
    }

    pub fn with_scale_xy(mut self, scale: Vec2) -> Self {
        self.scale = Signal::new(scale);
        self
    }

    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = Signal::new(opacity);
        self
    }

    pub fn with_size(mut self, size: Vec2) -> Self {
        self.size = Signal::new(size);
        self
    }

    pub fn with_path(mut self, path: &str) -> Self {
        self.image = ImageManager::get_image(path);
        if let Some(ref img) = self.image {
            self.size = Signal::new(Vec2::new(img.width as f32, img.height as f32));
        }
        self
    }
}

impl Node for ImageNode {
    fn render(&self, scene: &mut Scene, parent_transform: Affine, parent_opacity: f32) {
        let Some(ref img) = self.image else {
            return;
        };

        let size = self.size.get();
        let pos = self.position.get();
        let rot = self.rotation.get();
        let sc = self.scale.get();

        let local_transform = Affine::translate((pos.x as f64, pos.y as f64))
            * Affine::rotate(rot as f64)
            * Affine::scale_non_uniform(sc.x as f64, sc.y as f64);

        let opacity = self.opacity.get();
        let final_opacity = opacity * parent_opacity;

        if final_opacity <= 0.0 {
            return;
        }

        let transform = parent_transform
            * local_transform
            * Affine::scale_non_uniform(
                size.x as f64 / img.width as f64,
                size.y as f64 / img.height as f64,
            );

        if final_opacity < 1.0 {
            scene.push_layer(
                vello::peniko::Mix::Normal,
                final_opacity,
                transform,
                &vello::kurbo::Rect::new(0.0, 0.0, img.width as f64, img.height as f64),
            );
            scene.draw_image(img, Affine::IDENTITY);
            scene.pop_layer();
            return;
        }

        scene.draw_image(img, transform);
    }
    fn update(&mut self, _dt: Duration) {}
    fn state_hash(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut s = DefaultHasher::new();

        let pos = self.position.get();
        pos.x.to_bits().hash(&mut s);
        pos.y.to_bits().hash(&mut s);

        self.rotation.get().to_bits().hash(&mut s);

        let sc = self.scale.get();
        sc.x.to_bits().hash(&mut s);
        sc.y.to_bits().hash(&mut s);

        self.size.get().x.to_bits().hash(&mut s);
        self.size.get().y.to_bits().hash(&mut s);
        self.opacity.get().to_bits().hash(&mut s);
        s.finish()
    }

    fn clone_node(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
