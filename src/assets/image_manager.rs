#![cfg(feature = "image")]
use std::collections::HashMap;
use std::sync::LazyLock;
use std::sync::{Arc, Mutex};
use vello::peniko::{Blob, Extend, Format, Image as PenikoImage};

static IMAGE_CACHE: LazyLock<Mutex<HashMap<String, Arc<PenikoImage>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Global manager for loading and caching image assets.
///
/// `ImageManager` provides a centralized system for loading raster images (PNG, JPG).
/// It handles the conversion of these formats into Vello-compatible [`PenikoImage`] structures.
pub struct ImageManager;

impl ImageManager {
    /// Loads an image from the specified file path.
    ///
    /// This method first checks an internal cache. If the image is not found:
    /// - **Raster**: The image is loaded from disk (requires the `image` feature).
    pub fn get_image(path: &str) -> Option<Arc<PenikoImage>> {
        let mut cache = IMAGE_CACHE.lock().unwrap();
        if let Some(img) = cache.get(path) {
            return Some(img.clone());
        }

        // Load raster image from disk
        match image::open(path) {
            Ok(img) => {
                let rgba = img.to_rgba8();
                let (width, height) = rgba.dimensions();
                let data = Arc::new(rgba.into_raw());
                let peniko_img = Arc::new(PenikoImage {
                    data: Blob::new(data),
                    alpha: 255,
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
