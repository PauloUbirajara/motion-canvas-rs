use crate::engine::animation::{Signal, Node};
use crate::engine::font::FontManager;
use vello::peniko::{Brush, Color, Fill};
use vello::Scene;
use glam::Vec2;
use vello::kurbo::{Affine, BezPath};
use std::time::Duration;
use skrifa::MetadataProvider;
use skrifa::instance::{Size, LocationRef};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref GLOBAL_TEXT_CACHE: Mutex<HashMap<TextCacheKey, Arc<Vec<(Affine, BezPath)>>>> = Mutex::new(HashMap::new());
}

const DEFAULT_FONT_FAMILY: &str = "Inter";
const ADVANCE_FALLBACK_FACTOR: f32 = 0.6;

#[derive(Hash, Eq, PartialEq)]
struct TextCacheKey {
    text: String,
    font_size_bits: u32,
    font_family: String,
}

pub struct TextNode {
    pub position: Signal<Vec2>,
    pub text: Signal<String>,
    pub font_size: Signal<f32>,
    pub color: Signal<Color>,
    pub opacity: Signal<f32>,
    pub font_family: String,
    cache: Arc<Mutex<Option<Arc<Vec<(Affine, BezPath)>>>>>,
}

impl TextNode {
    pub fn new(position: Vec2, text: &str, size: f32, color: Color) -> Self {
        Self {
            position: Signal::new(position),
            text: Signal::new(text.to_string()),
            font_size: Signal::new(size),
            color: Signal::new(color),
            opacity: Signal::new(1.0),
            font_family: DEFAULT_FONT_FAMILY.to_string(),
            cache: Arc::new(Mutex::new(None)),
        }
    }

    pub fn with_font(mut self, family: &str) -> Self {
        self.font_family = family.to_string();
        self
    }
}

impl Clone for TextNode {
    fn clone(&self) -> Self {
        Self {
            position: self.position.clone(),
            text: self.text.clone(),
            font_size: self.font_size.clone(),
            color: self.color.clone(),
            opacity: self.opacity.clone(),
            font_family: self.font_family.clone(),
            cache: self.cache.clone(),
        }
    }
}

struct PathSink<'a>(&'a mut BezPath);

impl<'a> skrifa::outline::OutlinePen for PathSink<'a> {
    fn move_to(&mut self, x: f32, y: f32) { self.0.move_to((x as f64, y as f64)); }
    fn line_to(&mut self, x: f32, y: f32) { self.0.line_to((x as f64, y as f64)); }
    fn quad_to(&mut self, cx0: f32, cy0: f32, x: f32, y: f32) { self.0.quad_to((cx0 as f64, cy0 as f64), (x as f64, y as f64)); }
    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) { self.0.curve_to((cx0 as f64, cy0 as f64), (cx1 as f64, cy1 as f64), (x as f64, y as f64)); }
    fn close(&mut self) { self.0.close_path(); }
}

impl Node for TextNode {
    fn render(&self, scene: &mut Scene, parent_transform: Affine, parent_opacity: f32) {
        let text = self.text.get();
        let size = self.font_size.get();
        let color = self.color.get();
        let pos = self.position.get();
        let opacity = self.opacity.get();

        let key = TextCacheKey {
            text: text.clone(),
            font_size_bits: size.to_bits(),
            font_family: self.font_family.clone(),
        };

        // 1. Check global cache
        let mut global = GLOBAL_TEXT_CACHE.lock().unwrap();
        if let Some(paths) = global.get(&key) {
            let mut local = self.cache.lock().unwrap();
            *local = Some(paths.clone());
        } else {
            // 3. Rebuild
            let mut paths = Vec::new();
            if let Some(font_data) = FontManager::get_font_with_fallback(&[&self.font_family, DEFAULT_FONT_FAMILY, "Arial", "sans-serif"]) {
                let font_ref = FontManager::get_font_ref(&font_data);
                let charmap = font_ref.charmap();
                let outlines = font_ref.outline_glyphs();
                let mut x_offset = 0.0;
                
                for c in text.chars() {
                    let glyph_id = charmap.map(c).unwrap_or_default();
                    let mut pb = BezPath::new();
                    let mut advance = (size * ADVANCE_FALLBACK_FACTOR) as f64;
                    
                    if let Some(glyph) = outlines.get(glyph_id) {
                        let mut sink = PathSink(&mut pb);
                        let font_size = Size::new(size);
                        let _ = glyph.draw(font_size, &mut sink);
                        
                        if let Some(metrics) = font_ref.glyph_metrics(font_size, LocationRef::default()).advance_width(glyph_id) {
                            advance = metrics as f64;
                        }
                    }
                    
                    let base_transform = Affine::translate((x_offset, size as f64)) * Affine::scale_non_uniform(1.0, -1.0);
                    paths.push((base_transform, pb));
                    x_offset += advance;
                }
            }
            let arc_paths = Arc::new(paths);
            global.insert(key, arc_paths.clone());
            let mut local = self.cache.lock().unwrap();
            *local = Some(arc_paths);
        }

        if let Some(c) = self.cache.lock().unwrap().as_ref() {
            let root_transform = parent_transform * Affine::translate((pos.x as f64, pos.y as f64));
            let mut render_color = color;
            render_color.a = (color.a as f32 * opacity * parent_opacity).clamp(0.0, 255.0) as u8;
            let brush = Brush::Solid(render_color);
            for (local_transform, pb) in c.as_ref() {
                scene.fill(Fill::NonZero, root_transform * *local_transform, &brush, None, pb);
            }
        }
    }
    fn update(&mut self, _dt: Duration) {}
    fn state_hash(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;
        let mut s = DefaultHasher::new();
        self.position.get().x.to_bits().hash(&mut s);
        self.position.get().y.to_bits().hash(&mut s);
        self.text.get().hash(&mut s);
        self.font_size.get().to_bits().hash(&mut s);
        let color = self.color.get();
        color.r.hash(&mut s);
        color.g.hash(&mut s);
        color.b.hash(&mut s);
        color.a.hash(&mut s);
        self.opacity.get().to_bits().hash(&mut s);
        s.finish()
    }

    fn clone_node(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
