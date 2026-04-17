use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use vello::peniko::{Blob, Extend, Format, Image as PenikoImage};

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
        #[cfg(feature = "image")]
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
