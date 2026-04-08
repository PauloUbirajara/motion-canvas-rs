use crate::engine::animation::{Signal, Node, Tweenable};
use crate::engine::font::FontManager;
use vello::peniko::{Brush, Color, Fill};
use vello::Scene;
use glam::Vec2;
use vello::kurbo::Affine;
use std::time::Duration;
use skrifa::MetadataProvider;
use skrifa::instance::{Size, LocationRef};

#[derive(Clone)]
pub struct TextNode {
    pub transform: Signal<Affine>,
    pub opacity: Signal<f32>,
    pub text: Signal<String>,
    pub font_size: Signal<f32>,
    pub color: Color,
    pub font_family: String,
}

impl TextNode {
    pub fn new(pos: Vec2, text: &str, font_size: f32, color: Color) -> Self {
        Self {
            transform: Signal::new(Affine::translate((pos.x as f64, pos.y as f64))),
            opacity: Signal::new(1.0),
            text: Signal::new(text.to_string()),
            font_size: Signal::new(font_size),
            color,
            font_family: "Inter".to_string(), // Default font
        }
    }

    pub fn with_font(mut self, font: &str) -> Self {
        self.font_family = font.to_string();
        self
    }

    pub fn with_transform(mut self, transform: Affine) -> Self {
        self.transform = Signal::new(transform);
        self
    }

    pub fn with_position(mut self, pos: Vec2) -> Self {
        self.transform = Signal::new(Affine::translate((pos.x as f64, pos.y as f64)));
        self
    }

    pub fn with_rotation(mut self, rad: f32) -> Self {
        self.transform = Signal::new(self.transform.get() * Affine::rotate(rad as f64));
        self
    }

    pub fn with_scale(mut self, s: f32) -> Self {
        self.transform = Signal::new(self.transform.get() * Affine::scale(s as f64));
        self
    }

    pub fn with_opacity(mut self, a: f32) -> Self {
        self.opacity = Signal::new(a);
        self
    }
}

impl Node for TextNode {
    fn render(&self, scene: &mut Scene, parent_transform: Affine, parent_opacity: f32) {
        let local_transform = self.transform.get();
        let opacity = self.opacity.get() * parent_opacity;
        
        let combined_transform = parent_transform * local_transform;
        
        if opacity <= 0.0 {
            return;
        }

        let font_size = self.font_size.get();
        let text = self.text.get();

        let font_manager = FontManager::instance();
        if let Some(font) = font_manager.get_font(&self.font_family) {
            let font_ref = font.to_ref();
            let axes = font_ref.axes();
            let charmap = font_ref.charmap();
            let glyph_metrics = font_ref.glyph_metrics(Size::new(font_size), LocationRef::new(&axes, &[]));

            let mut x_offset = 0.0;
            for c in text.chars() {
                let glyph_id = charmap.map(c).unwrap_or_default();
                let advance = glyph_metrics.advance_width(glyph_id).unwrap_or(0.0);

                let glyph_transform = combined_transform
                    * Affine::translate((x_offset as f64, 0.0))
                    * Affine::scale_non_uniform(1.0, -1.0); // Flip Y for vello

                scene
                    .draw_glyph(glyph_id, glyph_transform)
                    .fill(Fill::NonZero, Brush::Solid(self.color.with_alpha_factor(opacity)))
                    .draw();

                x_offset += advance;
            }
        }
    }

    fn update(&mut self, _dt: Duration) {}

    fn state_hash(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;
        let mut s = DefaultHasher::new();
        
        let coeffs = self.transform.get().as_coeffs();
        for c in coeffs {
            c.to_bits().hash(&mut s);
        }
        self.text.get().hash(&mut s);
        self.font_size.get().to_bits().hash(&mut s);
        let color = self.color;
        color.r.hash(&mut s);
        color.g.hash(&mut s);
        color.b.hash(&mut s);
        color.a.hash(&mut s);
        self.opacity.get().to_bits().hash(&mut s);
        self.font_family.hash(&mut s);
        s.finish()
    }

    fn clone_node(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
