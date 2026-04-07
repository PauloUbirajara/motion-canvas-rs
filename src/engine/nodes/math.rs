use crate::engine::animation::{Signal, Node};
use vello::peniko::{Brush, Color, Fill};
use vello::Scene;
use glam::Vec2;
use vello::kurbo::{Affine, BezPath};
use std::time::Duration;
use skrifa::MetadataProvider;
use skrifa::instance::{Size};

#[derive(Clone)]
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

struct PathSink<'a>(&'a mut BezPath);

impl<'a> skrifa::outline::OutlinePen for PathSink<'a> {
    fn move_to(&mut self, x: f32, y: f32) { self.0.move_to((x as f64, y as f64)); }
    fn line_to(&mut self, x: f32, y: f32) { self.0.line_to((x as f64, y as f64)); }
    fn quad_to(&mut self, cx0: f32, cy0: f32, x: f32, y: f32) { self.0.quad_to((cx0 as f64, cy0 as f64), (x as f64, y as f64)); }
    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) { self.0.curve_to((cx0 as f64, cy0 as f64), (cx1 as f64, cy1 as f64), (x as f64, y as f64)); }
    fn close(&mut self) { self.0.close_path(); }
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
