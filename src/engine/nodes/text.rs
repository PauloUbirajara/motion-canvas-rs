use crate::engine::animation::{Signal, Node};
use crate::engine::font::FontManager;
use vello::peniko::{Brush, Color, Fill};
use vello::Scene;
use glam::Vec2;
use vello::kurbo::{Affine, BezPath};
use std::time::Duration;
use skrifa::MetadataProvider;
use skrifa::instance::{Size, LocationRef};
use std::sync::Mutex;

#[derive(Clone, Default)]
struct TextCache {
    text: String,
    font_size: f32,
    font_family: String,
    color: Color,
    paths: Vec<(Affine, Color, BezPath)>,
}

pub struct TextNode {
    pub position: Signal<Vec2>,
    pub text: Signal<String>,
    pub font_size: Signal<f32>,
    pub color: Signal<Color>,
    pub font_family: String,
    cache: Mutex<Option<TextCache>>,
}

impl TextNode {
    pub fn new(position: Vec2, text: &str, size: f32, color: Color) -> Self {
        Self {
            position: Signal::new(position),
            text: Signal::new(text.to_string()),
            font_size: Signal::new(size),
            color: Signal::new(color),
            font_family: "Inter".to_string(),
            cache: Mutex::new(None),
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
            font_family: self.font_family.clone(),
            cache: Mutex::new(None),
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
    fn render(&self, scene: &mut Scene) {
        let text = self.text.data.lock().unwrap().value.clone();
        let size = self.font_size.data.lock().unwrap().value;
        let color = self.color.data.lock().unwrap().value;
        let pos = self.position.data.lock().unwrap().value;

        let mut cache = self.cache.lock().unwrap();
        let needs_rebuild = cache.as_ref().map_or(true, |c| {
            c.text != text || c.font_size != size || c.font_family != self.font_family || c.color != color
        });

        if needs_rebuild {
            let mut paths = Vec::new();
            if let Some(font_data) = FontManager::get_font_with_fallback(&[&self.font_family, "Inter", "Arial", "sans-serif"]) {
                let font_ref = FontManager::get_font_ref(&font_data);
                let charmap = font_ref.charmap();
                let outlines = font_ref.outline_glyphs();
                let mut x_offset = 0.0;
                
                for c in text.chars() {
                    let glyph_id = charmap.map(c).unwrap_or_default();
                    let mut pb = BezPath::new();
                    let mut advance = (size * 0.6) as f64;
                    
                    if let Some(glyph) = outlines.get(glyph_id) {
                        let mut sink = PathSink(&mut pb);
                        let font_size = Size::new(size);
                        let _ = glyph.draw(font_size, &mut sink);
                        
                        if let Some(metrics) = font_ref.glyph_metrics(font_size, LocationRef::default()).advance_width(glyph_id) {
                            advance = metrics as f64;
                        }
                    }
                    
                    let base_transform = Affine::translate((x_offset, size as f64)) * Affine::scale_non_uniform(1.0, -1.0);
                    paths.push((base_transform, color, pb));
                    x_offset += advance;
                }
            }
            *cache = Some(TextCache {
                text: text.clone(),
                font_size: size,
                font_family: self.font_family.clone(),
                color,
                paths,
            });
        }

        if let Some(c) = cache.as_ref() {
            let root_transform = Affine::translate((pos.x as f64, pos.y as f64));
            for (local_transform, path_color, pb) in &c.paths {
                scene.fill(Fill::NonZero, root_transform * *local_transform, &Brush::Solid(*path_color), None, pb);
            }
        }
    }
    fn update(&mut self, _dt: Duration) {}
    fn state_hash(&self) -> u64 {
        let pos = self.position.get();
        let text = self.text.get();
        let size = self.font_size.get();
        let color = self.color.get();
        let mut hash = 0u64;
        hash ^= pos.x.to_bits() as u64;
        hash ^= pos.y.to_bits() as u64;
        hash ^= size.to_bits() as u64;
        hash ^= (color.r as u64) << 24 | (color.g as u64) << 16 | (color.b as u64) << 8 | (color.a as u64);
        for b in text.as_bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(*b as u64);
        }
        hash
    }
}
