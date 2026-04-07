use crate::engine::animation::{Signal, Node};
use crate::engine::font::FontManager;
use vello::peniko::{Brush, Color, Fill};
use vello::Scene;
use glam::Vec2;
use vello::kurbo::{Affine, Circle as KurboCircle, BezPath, RoundedRect as KurboRoundedRect, Stroke, Line as KurboLine};
use std::sync::Arc;
use std::time::Duration;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet};
use syntect::easy::HighlightLines;
use skrifa::MetadataProvider;
use skrifa::instance::{Size, LocationRef};

pub struct Circle {
    pub position: Signal<Vec2>,
    pub radius: Signal<f32>,
    pub fill: Color,
}

impl Circle {
    pub fn new(position: Vec2, radius: f32, fill: Color) -> Self {
        Self {
            position: Signal::new(position),
            radius: Signal::new(radius),
            fill,
        }
    }
}

impl Node for Circle {
    fn render(&self, scene: &mut Scene) {
        let brush = Brush::Solid(self.fill);
        let pos = self.position.data.lock().unwrap().value.clone();
        let radius = self.radius.data.lock().unwrap().value;
        
        scene.fill(
            Fill::NonZero,
            Affine::translate((pos.x as f64, pos.y as f64)),
            &brush,
            None,
            &KurboCircle::new((0.0, 0.0), radius as f64),
        );
    }
    fn update(&mut self, _dt: Duration) {}
    fn state_hash(&self) -> u64 {
        let pos = self.position.data.lock().unwrap().value;
        let radius = self.radius.data.lock().unwrap().value;
        let mut hash = 0u64;
        hash ^= pos.x.to_bits() as u64;
        hash ^= pos.y.to_bits() as u64;
        hash ^= radius.to_bits() as u64;
        hash
    }
}

pub struct Rect {
    pub position: Signal<Vec2>,
    pub size: Signal<Vec2>,
    pub fill: Color,
    pub radius: f32,
}

impl Rect {
    pub fn new(position: Vec2, size: Vec2, fill: Color) -> Self {
        Self {
            position: Signal::new(position),
            size: Signal::new(size),
            fill,
            radius: 0.0,
        }
    }
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }
}

impl Node for Rect {
    fn render(&self, scene: &mut Scene) {
        let brush = Brush::Solid(self.fill);
        let pos = self.position.data.lock().unwrap().value.clone();
        let size = self.size.data.lock().unwrap().value.clone();
        
        scene.fill(
            Fill::NonZero,
            Affine::translate((pos.x as f64, pos.y as f64)),
            &brush,
            None,
            &KurboRoundedRect::new(0.0, 0.0, size.x as f64, size.y as f64, self.radius as f64),
        );
    }
    fn update(&mut self, _dt: Duration) {}
    fn state_hash(&self) -> u64 {
        let pos = self.position.data.lock().unwrap().value;
        let size = self.size.data.lock().unwrap().value;
        let mut hash = 0u64;
        hash ^= pos.x.to_bits() as u64;
        hash ^= pos.y.to_bits() as u64;
        hash ^= size.x.to_bits() as u64;
        hash ^= size.y.to_bits() as u64;
        hash
    }
}

pub struct Line {
    pub start: Signal<Vec2>,
    pub end: Signal<Vec2>,
    pub stroke: Color,
    pub thickness: f32,
}

impl Line {
    pub fn new(start: Vec2, end: Vec2, stroke: Color, thickness: f32) -> Self {
        Self {
            start: Signal::new(start),
            end: Signal::new(end),
            stroke,
            thickness,
        }
    }
}

impl Node for Line {
    fn render(&self, scene: &mut Scene) {
        let brush = Brush::Solid(self.stroke);
        let start = self.start.data.lock().unwrap().value.clone();
        let end = self.end.data.lock().unwrap().value.clone();
        
        scene.stroke(
            &Stroke::new(self.thickness as f64),
            Affine::IDENTITY,
            &brush,
            None,
            &KurboLine::new((start.x as f64, start.y as f64), (end.x as f64, end.y as f64)),
        );
    }
    fn update(&mut self, _dt: Duration) {}
    fn state_hash(&self) -> u64 {
        let start = self.start.data.lock().unwrap().value;
        let end = self.end.data.lock().unwrap().value;
        let mut hash = 0u64;
        hash ^= start.x.to_bits() as u64;
        hash ^= end.x.to_bits() as u64;
        hash
    }
}

pub struct PathData {
    pub path: BezPath,
    pub segments: Vec<(Vec2, f32)>,
    pub total_length: f32,
}

impl PathData {
    pub fn new(path: BezPath) -> Self {
        let mut segments = Vec::new();
        let mut total_length = 0.0;
        let mut last_point: Option<Vec2> = None;
        vello::kurbo::flatten(&path, 0.1, |el| {
            match el {
                vello::kurbo::PathEl::MoveTo(p) => {
                    let pt = Vec2::new(p.x as f32, p.y as f32);
                    segments.push((pt, 0.0));
                    last_point = Some(pt);
                }
                vello::kurbo::PathEl::LineTo(p) => {
                    if let Some(last) = last_point {
                        let pt = Vec2::new(p.x as f32, p.y as f32);
                        total_length += last.distance(pt);
                        segments.push((pt, total_length));
                        last_point = Some(pt);
                    }
                }
                _ => {} 
            }
        });
        Self { path, segments, total_length }
    }
    pub fn sample(&self, t: f32) -> Vec2 {
        if self.segments.is_empty() { return Vec2::ZERO; }
        let target_len = t.clamp(0.0, 1.0) * self.total_length;
        let idx = match self.segments.binary_search_by(|&(_, len)| len.partial_cmp(&target_len).unwrap()) {
            Ok(i) => i,
            Err(i) => i,
        };
        if idx == 0 { return self.segments[0].0; }
        if idx >= self.segments.len() { return self.segments.last().unwrap().0; }
        let (p1, l1) = self.segments[idx - 1];
        let (p2, l2) = self.segments[idx];
        let segment_len = l2 - l1;
        if segment_len < 0.0001 { return p2; }
        let t_segment = (target_len - l1) / segment_len;
        p1.lerp(p2, t_segment)
    }
}

pub struct PathNode {
    pub data: Arc<PathData>,
    pub stroke: Color,
    pub thickness: f32,
}

impl PathNode {
    pub fn new(path: BezPath, stroke: Color, thickness: f32) -> Self {
        Self {
            data: Arc::new(PathData::new(path)),
            stroke,
            thickness,
        }
    }
}

impl Node for PathNode {
    fn render(&self, scene: &mut Scene) {
        let brush = Brush::Solid(self.stroke);
        scene.stroke(&Stroke::new(self.thickness as f64), Affine::IDENTITY, &brush, None, &self.data.path);
    }
    fn update(&mut self, _dt: Duration) {}
    fn state_hash(&self) -> u64 {
        let mut hash = 0u64;
        hash ^= self.thickness.to_bits() as u64;
        hash
    }
}

// --- Text rendering helper ---
struct PathSink<'a>(&'a mut BezPath);
impl<'a> skrifa::outline::OutlinePen for PathSink<'a> {
    fn move_to(&mut self, x: f32, y: f32) { self.0.move_to((x as f64, y as f64)); }
    fn line_to(&mut self, x: f32, y: f32) { self.0.line_to((x as f64, y as f64)); }
    fn quad_to(&mut self, cx: f32, cy: f32, x: f32, y: f32) { self.0.quad_to((cx as f64, cy as f64), (x as f64, y as f64)); }
    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) { self.0.curve_to((cx0 as f64, cy0 as f64), (cx1 as f64, cy1 as f64), (x as f64, y as f64)); }
    fn close(&mut self) { self.0.close_path(); }
}

pub struct TextNode {
    pub position: Signal<Vec2>,
    pub text: Signal<String>,
    pub font_size: Signal<f32>,
    pub color: Signal<Color>,
    pub font_family: String,
}

impl TextNode {
    pub fn new(position: Vec2, text: &str, font_size: f32, color: Color) -> Self {
        Self {
            position: Signal::new(position),
            text: Signal::new(text.to_string()),
            font_size: Signal::new(font_size),
            color: Signal::new(color),
            font_family: "Inter".to_string(),
        }
    }
    pub fn with_font(mut self, family: &str) -> Self {
        self.font_family = family.to_string();
        self
    }
}

impl Node for TextNode {
    fn render(&self, scene: &mut Scene) {
        let pos = self.position.data.lock().unwrap().value.clone();
        let text = self.text.data.lock().unwrap().value.clone();
        let size = self.font_size.data.lock().unwrap().value;
        let color = self.color.data.lock().unwrap().value;
        if let Some(font_data) = FontManager::get_font_with_fallback(&[&self.font_family, "Inter", "Arial", "sans-serif"]) {
            let font_ref = FontManager::get_font_ref(&font_data);
            let charmap = font_ref.charmap();
            let outlines = font_ref.outline_glyphs();
            let brush = Brush::Solid(color);
            let mut x_offset = 0.0;
            for c in text.chars() {
                let glyph_id = charmap.map(c).unwrap_or_default();
                let mut pb = BezPath::new();
                let mut advance = (size * 0.6) as f64; // Fallback
                
                if let Some(glyph) = outlines.get(glyph_id) {
                    let mut sink = PathSink(&mut pb);
                    let font_size = Size::new(size);
                    let _ = glyph.draw(font_size, &mut sink);
                    
                    if let Some(metrics) = font_ref.glyph_metrics(font_size, LocationRef::default()).advance_width(glyph_id) {
                        advance = metrics as f64;
                    }
                }
                
                let transform = Affine::translate((pos.x as f64 + x_offset, pos.y as f64 + size as f64))
                    * Affine::scale_non_uniform(1.0, -1.0);
                scene.fill(Fill::NonZero, transform, &brush, None, &pb);
                x_offset += advance;
            }
        }
    }
    fn update(&mut self, _dt: Duration) {}
    fn state_hash(&self) -> u64 {
        let pos = self.position.data.lock().unwrap().value;
        pos.x.to_bits() as u64
    }
}

pub struct CodeNode {
    pub position: Signal<Vec2>,
    pub code: Signal<String>,
    pub font_size: Signal<f32>,
    pub font_family: String,
    pub language: String,
    pub theme: String,
}

impl CodeNode {
    pub fn new(pos: Vec2, code: &str, lang: &str) -> Self {
        Self {
            position: Signal::new(pos),
            code: Signal::new(code.to_string()),
            font_size: Signal::new(14.0),
            font_family: "Fira Code".to_string(),
            language: lang.to_string(),
            theme: "base16-ocean.dark".to_string(),
        }
    }
}

impl Node for CodeNode {
    fn render(&self, scene: &mut Scene) {
        let pos = self.position.data.lock().unwrap().value.clone();
        let code = self.code.data.lock().unwrap().value.clone();
        let size = self.font_size.data.lock().unwrap().value;
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
                let ranges = h.highlight_line(line, &ss).unwrap();
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

pub struct MathNode {
    pub position: Signal<Vec2>,
    pub equation: Signal<String>,
    pub font_size: Signal<f32>,
    pub color: Color,
}

impl MathNode {
    pub fn new(pos: Vec2, equation: &str, size: f32, color: Color) -> Self {
        Self {
            position: Signal::new(pos),
            equation: Signal::new(equation.to_string()),
            font_size: Signal::new(size),
            color,
        }
    }
}

impl Node for MathNode {
    fn render(&self, scene: &mut Scene) {
        let pos = self.position.data.lock().unwrap().value.clone();
        let eq = self.equation.data.lock().unwrap().value.clone();
        let size = self.font_size.data.lock().unwrap().value;
        let typst_code = format!("#set text(size: {}pt)\n$ {} $", size, eq);
        let world = crate::engine::typst_support::TypstWorld::new(&typst_code);
        let output = typst::compile::<typst::layout::PagedDocument>(&world).output;
        if let Ok(document) = output {
            if let Some(page) = document.pages.first() {
                let frame = &page.frame;
                for (p, item) in frame.items() {
                    match item {
                        typst::layout::FrameItem::Text(text) => {
                            let brush = Brush::Solid(self.color);
                            let font_data = text.font.data();
                            let font_ref = skrifa::FontRef::new(font_data).unwrap();
                            let outlines = font_ref.outline_glyphs();
                            for glyph in &text.glyphs {
                                let mut pb = BezPath::new();
                                if let Some(g_out) = outlines.get(skrifa::GlyphId::from(glyph.id)) {
                                    let mut sink = PathSink(&mut pb);
                                    let _ = g_out.draw(Size::new(text.size.to_pt() as f32), &mut sink);
                                }
                                let transform = Affine::translate((
                                    pos.x as f64 + p.x.to_pt() + glyph.x_offset.at(text.size).to_pt(), 
                                    pos.y as f64 + p.y.to_pt() + glyph.y_offset.at(text.size).to_pt() + text.size.to_pt() // Adjust for flip
                                )) * Affine::scale_non_uniform(1.0, -1.0);
                                scene.fill(Fill::NonZero, transform, &brush, None, &pb);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    fn update(&mut self, _dt: Duration) {}
    fn state_hash(&self) -> u64 { self.position.data.lock().unwrap().value.x.to_bits() as u64 }
}
