use crate::engine::animation::{Signal, Node};
use crate::engine::font::FontManager;
use vello::peniko::{Brush, Color, Fill};
use vello::Scene;
use glam::Vec2;
use vello::kurbo::{Affine, BezPath};
use std::time::Duration;
use skrifa::MetadataProvider;
use skrifa::instance::{Size, LocationRef};
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet};
use syntect::easy::HighlightLines;

pub struct CodeNode {
    pub position: Signal<Vec2>,
    pub code: Signal<String>,
    pub font_size: Signal<f32>,
    pub language: String,
    pub theme: String,
    pub font_family: String,
}

impl CodeNode {
    pub fn new(pos: Vec2, code: &str, lang: &str) -> Self {
        Self {
            position: Signal::new(pos),
            code: Signal::new(code.to_string()),
            font_size: Signal::new(20.0),
            language: lang.to_string(),
            theme: "base16-ocean.dark".to_string(),
            font_family: "Fira Code".to_string(),
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

impl Node for CodeNode {
    fn render(&self, scene: &mut Scene) {
        let code = self.code.data.lock().unwrap().value.clone();
        let size = self.font_size.data.lock().unwrap().value;
        let pos = self.position.data.lock().unwrap().value;

        let ss = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        let syntax = ss.find_syntax_by_extension(&self.language).unwrap_or_else(|| ss.find_syntax_plain_text());
        let mut h = HighlightLines::new(syntax, &ts.themes[&self.theme]);
        let mut y_offset = 0.0;
        if let Some(font_data) = FontManager::get_font_with_fallback(&[&self.font_family, "Fira Code", "Courier New", "monospace"]) {
            let font_ref = FontManager::get_font_ref(&font_data);
            let charmap = font_ref.charmap();
            let outlines = font_ref.outline_glyphs();

            for line in code.lines() {
                let ranges: Vec<(syntect::highlighting::Style, &str)> = h.highlight_line(line, &ss).unwrap();
                let mut x_offset = 0.0;
                for (style, text) in ranges {
                    let fg = style.foreground;
                    let brush = Brush::Solid(Color::rgba8(fg.r, fg.g, fg.b, fg.a));
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
                        
                        let transform = Affine::translate((pos.x as f64 + x_offset, pos.y as f64 + y_offset + size as f64))
                            * Affine::scale_non_uniform(1.0, -1.0);
                        scene.fill(Fill::NonZero, transform, &brush, None, &pb);
                        x_offset += advance;
                    }
                }
                y_offset += (size * 1.2) as f64;
            }
        }
    }
    fn update(&mut self, _dt: Duration) {}
    fn state_hash(&self) -> u64 { self.position.data.lock().unwrap().value.x.to_bits() as u64 }
}
