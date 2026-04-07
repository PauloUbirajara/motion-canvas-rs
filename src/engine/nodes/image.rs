use crate::engine::animation::{Signal, Node};
use vello::peniko::{Image as PenikoImage, Format, Extend, Blob};
use vello::Scene;
use glam::Vec2;
use vello::kurbo::Affine;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use lazy_static::lazy_static;

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

        // Load image from disk
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
    pub size: Signal<Vec2>,
    pub image: Option<Arc<PenikoImage>>,
    pub opacity: Signal<f32>,
}

impl ImageNode {
    pub fn new(pos: Vec2, path: &str) -> Self {
        let image = ImageManager::get_image(path);
        let size = if let Some(ref img) = image {
            Vec2::new(img.width as f32, img.height as f32)
        } else {
            Vec2::ZERO
        };

        Self {
            position: Signal::new(pos),
            size: Signal::new(size),
            image,
            opacity: Signal::new(1.0),
        }
    }

    pub fn with_size(mut self, size: Vec2) -> Self {
        self.size = Signal::new(size);
        self
    }
}

impl Node for ImageNode {
    fn render(&self, scene: &mut Scene) {
        if let Some(ref img) = self.image {
            let pos = self.position.data.lock().unwrap().value;
            let size = self.size.data.lock().unwrap().value;
            let _opacity = self.opacity.data.lock().unwrap().value;

            let transform = Affine::translate((pos.x as f64, pos.y as f64))
                * Affine::scale_non_uniform(
                    size.x as f64 / img.width as f64,
                    size.y as f64 / img.height as f64
                );

            scene.draw_image(img, transform);
            // Note: Peniko/Vello basic draw_image doesn't take opacity directly easily in this version
            // Alternative: use a layer or brush, but for now we draw the image.
        }
    }
    fn update(&mut self, _dt: Duration) {}
    fn state_hash(&self) -> u64 {
        let pos = self.position.data.lock().unwrap().value;
        pos.x.to_bits() as u64
    }
}
