use crate::engine::animation::{Node, Signal};
use crate::engine::font::FontManager;
use glam::Vec2;
use lazy_static::lazy_static;
use skrifa::instance::{LocationRef, Size};
use skrifa::MetadataProvider;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use vello::kurbo::{Affine, BezPath, Shape};
use vello::peniko::{Brush, Color, Fill};
use vello::Scene;

lazy_static! {
    static ref GLOBAL_TEXT_CACHE: Mutex<HashMap<TextCacheKey, Arc<Vec<(Affine, BezPath)>>>> =
        Mutex::new(HashMap::new());
}

const DEFAULT_FONT_SIZE: f32 = 32.0;
const DEFAULT_COLOR: Color = Color::WHITE;
const DEFAULT_OPACITY: f32 = 1.0;
const DEFAULT_FONT_FAMILY: &str = "JetBrains Mono";
const FONT_FALLBACKS: &[&str] = &["Inter", "Arial", "sans-serif"];
const ADVANCE_FALLBACK_FACTOR: f32 = 0.6;

#[derive(Hash, Eq, PartialEq)]
struct TextCacheKey {
    text: String,
    font_size_bits: u32,
    font_family: String,
}

pub struct TextNode {
    pub position: Signal<Vec2>,
    pub rotation: Signal<f32>,
    pub scale: Signal<Vec2>,
    pub text: Signal<String>,
    pub font_size: Signal<f32>,
    pub fill_color: Signal<Color>,
    pub opacity: Signal<f32>,
    pub anchor: Signal<Vec2>,
    pub font_family: String,
    cache: Arc<Mutex<Option<Arc<Vec<(Affine, BezPath)>>>>>,
}

impl Default for TextNode {
    fn default() -> Self {
        Self {
            position: Signal::new(Vec2::ZERO),
            rotation: Signal::new(0.0),
            scale: Signal::new(Vec2::ONE),
            text: Signal::new("".to_string()),
            font_size: Signal::new(DEFAULT_FONT_SIZE),
            fill_color: Signal::new(DEFAULT_COLOR),
            opacity: Signal::new(DEFAULT_OPACITY),
            anchor: Signal::new(Vec2::ZERO),
            font_family: DEFAULT_FONT_FAMILY.to_string(),
            cache: Arc::new(Mutex::new(None)),
        }
    }
}

impl TextNode {
    pub fn new(position: Vec2, text: &str, size: f32, color: Color) -> Self {
        Self::default()
            .with_position(position)
            .with_text(text)
            .with_font_size(size)
            .with_fill(color)
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

    pub fn with_font(mut self, family: &str) -> Self {
        self.font_family = family.to_string();
        self
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.text = Signal::new(text.to_string());
        self
    }

    pub fn with_font_size(mut self, size: f32) -> Self {
        self.font_size = Signal::new(size);
        self
    }

    pub fn with_fill(mut self, color: Color) -> Self {
        self.fill_color = Signal::new(color);
        self
    }

    pub fn with_anchor(mut self, anchor: Vec2) -> Self {
        self.anchor = Signal::new(anchor);
        self
    }

    #[deprecated(note = "use with_fill instead")]
    pub fn with_color(self, color: Color) -> Self {
        self.with_fill(color)
    }
}

impl Clone for TextNode {
    fn clone(&self) -> Self {
        Self {
            position: self.position.clone(),
            rotation: self.rotation.clone(),
            scale: self.scale.clone(),
            text: self.text.clone(),
            font_size: self.font_size.clone(),
            fill_color: self.fill_color.clone(),
            opacity: self.opacity.clone(),
            anchor: self.anchor.clone(),
            font_family: self.font_family.clone(),
            cache: self.cache.clone(),
        }
    }
}

struct PathSink<'a>(&'a mut BezPath);

impl<'a> skrifa::outline::OutlinePen for PathSink<'a> {
    fn move_to(&mut self, x: f32, y: f32) {
        self.0.move_to((x as f64, y as f64));
    }
    fn line_to(&mut self, x: f32, y: f32) {
        self.0.line_to((x as f64, y as f64));
    }
    fn quad_to(&mut self, cx0: f32, cy0: f32, x: f32, y: f32) {
        self.0
            .quad_to((cx0 as f64, cy0 as f64), (x as f64, y as f64));
    }
    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
        self.0.curve_to(
            (cx0 as f64, cy0 as f64),
            (cx1 as f64, cy1 as f64),
            (x as f64, y as f64),
        );
    }
    fn close(&mut self) {
        self.0.close_path();
    }
}

impl Node for TextNode {
    fn render(&self, scene: &mut Scene, parent_transform: Affine, parent_opacity: f32) {
        let text = self.text.get();
        let size = self.font_size.get();
        let color = self.fill_color.get();
        let opacity = self.opacity.get();

        let pos = self.position.get();
        let rot = self.rotation.get();
        let sc = self.scale.get();
        let anchor = self.anchor.get();

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
            let mut fallback_list = vec![self.font_family.as_str(), DEFAULT_FONT_FAMILY];
            fallback_list.extend_from_slice(FONT_FALLBACKS);

            if let Some(font_data) = FontManager::get_font_with_fallback(&fallback_list) {
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

                        if let Some(metrics) = font_ref
                            .glyph_metrics(font_size, LocationRef::default())
                            .advance_width(glyph_id)
                        {
                            advance = metrics as f64;
                        }
                    }

                    let base_transform = Affine::translate((x_offset, size as f64))
                        * Affine::scale_non_uniform(1.0, -1.0);
                    paths.push((base_transform, pb));
                    x_offset += advance;
                }
            }
            let arc_paths = Arc::new(paths);
            global.insert(key, arc_paths.clone());
            let mut local = self.cache.lock().unwrap();
            *local = Some(arc_paths);
        }

        let cache_guard = self.cache.lock().unwrap();
        let Some(c) = cache_guard.as_ref() else {
            return;
        };

        // Calculate bounding box for centering and anchor
        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;

        for (glyph_transform, pb) in c.as_ref() {
            let bounds = pb.bounding_box();
            let p0 = *glyph_transform * vello::kurbo::Point::new(bounds.x0, bounds.y0);
            let p1 = *glyph_transform * vello::kurbo::Point::new(bounds.x1, bounds.y1);
            min_x = min_x.min(p0.x).min(p1.x);
            min_y = min_y.min(p0.y).min(p1.y);
            max_x = max_x.max(p0.x).max(p1.x);
            max_y = max_y.max(p0.y).max(p1.y);
        }

        let size_vec = if min_x == f64::MAX {
            Vec2::ZERO
        } else {
            Vec2::new((max_x - min_x) as f32, (max_y - min_y) as f32)
        };

        let center_offset = if min_x == f64::MAX {
            Vec2::ZERO
        } else {
            Vec2::new((min_x + max_x) as f32 * 0.5, (min_y + max_y) as f32 * 0.5)
        };

        let anchor_offset = anchor * size_vec * 0.5;

        let local_transform = Affine::translate((pos.x as f64, pos.y as f64))
            * Affine::rotate(rot as f64)
            * Affine::scale_non_uniform(sc.x as f64, sc.y as f64)
            * Affine::translate((-anchor_offset.x as f64, -anchor_offset.y as f64))
            * Affine::translate((-center_offset.x as f64, -center_offset.y as f64));

        let root_transform = parent_transform * local_transform;
        let mut render_color = color;
        render_color.a = (color.a as f32 * opacity * parent_opacity).clamp(0.0, 255.0) as u8;
        let brush = Brush::Solid(render_color);
        for (glyph_transform, pb) in c.as_ref() {
            scene.fill(
                Fill::NonZero,
                root_transform * *glyph_transform,
                &brush,
                None,
                pb,
            );
        }
    }
    fn update(&mut self, _dt: Duration) {}
    fn state_hash(&self) -> u64 {
        use crate::engine::util::hash::Hasher;
        let mut h = Hasher::new();
        h.update_u64(self.position.state_hash());
        h.update_u64(self.rotation.state_hash());
        h.update_u64(self.scale.state_hash());
        h.update_u64(self.text.state_hash());
        h.update_u64(self.font_size.state_hash());
        h.update_u64(self.fill_color.state_hash());
        h.update_u64(self.opacity.state_hash());
        h.update_u64(self.anchor.state_hash());
        h.update_bytes(self.font_family.as_bytes());
        h.finish()
    }

    fn clone_node(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn reset(&mut self) {
        self.position.reset();
        self.rotation.reset();
        self.scale.reset();
        self.text.reset();
        self.font_size.reset();
        self.fill_color.reset();
        self.opacity.reset();
        self.anchor.reset();
    }
}
