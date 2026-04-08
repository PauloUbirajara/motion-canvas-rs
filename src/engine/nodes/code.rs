//! Code highlighting and animation module.
//! 
//! Credits: The token-based animation logic and diffing approach is inspired by
//! [shiki-magic-move](https://github.com/shikijs/shiki-magic-move).

use crate::engine::animation::{Signal, Node, Tweenable};
use crate::engine::font::FontManager;
use vello::peniko::{Brush, Color, Fill};
use vello::Scene;
use glam::Vec2;
use vello::kurbo::{Affine, BezPath};
use std::time::Duration;
use skrifa::MetadataProvider;
use skrifa::instance::{Size, LocationRef};
use syntect::parsing::SyntaxSet;
use syntect::highlighting::ThemeSet;

#[derive(Clone)]
pub struct CodeNode {
    pub transform: Signal<Affine>,
    pub opacity: Signal<f32>,
    pub code: Signal<String>,
    pub font_size: Signal<f32>,
    pub font_family: String,
    pub theme: String,
    pub language: String,
}

impl CodeNode {
    pub fn new(pos: Vec2, code: &str, font_size: f32) -> Self {
        Self {
            transform: Signal::new(Affine::translate((pos.x as f64, pos.y as f64))),
            opacity: Signal::new(1.0),
            code: Signal::new(code.to_string()),
            font_size: Signal::new(font_size),
            font_family: "Fira Code".to_string(), // Common for code
            theme: "base16-ocean.dark".to_string(),
            language: "rust".to_string(),
        }
    }

    pub fn with_theme(mut self, theme: &str) -> Self {
        self.theme = theme.to_string();
        self
    }

    pub fn with_language(mut self, lang: &str) -> Self {
        self.language = lang.to_string();
        self
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

impl Node for CodeNode {
    fn render(&self, scene: &mut Scene, parent_transform: Affine, parent_opacity: f32) {
        let local_transform = self.transform.get();
        let opacity = self.opacity.get() * parent_opacity;
        
        let combined_transform = parent_transform * local_transform;
        
        if opacity <= 0.0 {
            return;
        }

        let code = self.code.get();
        let font_size = self.font_size.get();

        // Highlighter logic
        let ps = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        let syntax = ps.find_syntax_by_extension(&self.language).unwrap_or_else(|| ps.find_syntax_plain_text());
        let theme = ts.themes.get(&self.theme).unwrap_or(&ts.themes["base16-ocean.dark"]);

        let mut h = syntect::easy::HighlightLines::new(syntax, theme);
        let font_manager = FontManager::instance();
        
        if let Some(font) = font_manager.get_font(&self.font_family) {
            let font_ref = font.to_ref();
            let axes = font_ref.axes();
            let charmap = font_ref.charmap();
            let glyph_metrics = font_ref.glyph_metrics(Size::new(font_size), LocationRef::new(&axes, &[]));

            let mut y_offset = 0.0;
            for line in code.lines() {
                let mut x_offset = 0.0;
                let ranges: Vec<(syntect::highlighting::Style, &str)> = h.highlight_line(line, &ps).unwrap();
                
                for (style, text) in ranges {
                    let color = style.foreground;
                    let vello_color = Color::rgba8(color.r, color.g, color.b, color.a);
                    
                    for c in text.chars() {
                        let glyph_id = charmap.map(c).unwrap_or_default();
                        let advance = glyph_metrics.advance_width(glyph_id).unwrap_or(0.0);

                        let glyph_transform = combined_transform
                            * Affine::translate((x_offset as f64, y_offset as f64))
                            * Affine::scale_non_uniform(1.0, -1.0);

                        scene
                            .draw_glyph(glyph_id, glyph_transform)
                            .fill(Fill::NonZero, Brush::Solid(vello_color.with_alpha_factor(opacity)))
                            .draw();

                        x_offset += advance;
                    }
                }
                y_offset += font_size as f64 * 1.2;
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
        self.code.get().hash(&mut s);
        self.font_size.get().to_bits().hash(&mut s);
        self.opacity.get().to_bits().hash(&mut s);
        self.language.hash(&mut s);
        self.theme.hash(&mut s);
        self.font_family.hash(&mut s);
        s.finish()
    }

    fn clone_node(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
