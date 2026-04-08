use crate::engine::animation::{Signal, Node};
use vello::peniko::{Brush, Color, Fill};
use vello::Scene;
use glam::Vec2;
use vello::kurbo::{Affine, BezPath};
use std::time::Duration;
use skrifa::MetadataProvider;
use skrifa::instance::{Size};
use std::sync::Mutex;

#[derive(Clone, Default)]
struct MathCache {
    equation: String,
    font_size: f32,
    color: Color,
    paths: Vec<(Affine, Color, BezPath)>,
}

pub struct MathNode {
    pub position: Signal<Vec2>,
    pub equation: Signal<String>,
    pub font_size: Signal<f32>,
    pub color: Signal<Color>,
    pub opacity: Signal<f32>,
    pub transition_progress: Signal<f32>,
    cache: Mutex<Option<MathCache>>,
    prev_cache: Mutex<Option<MathCache>>,
}

impl Clone for MathNode {
    fn clone(&self) -> Self {
        Self {
            position: self.position.clone(),
            equation: self.equation.clone(),
            font_size: self.font_size.clone(),
            color: self.color.clone(),
            opacity: self.opacity.clone(),
            transition_progress: self.transition_progress.clone(),
            cache: Mutex::new(None),
            prev_cache: Mutex::new(None),
        }
    }
}

impl MathNode {
    pub fn new(pos: Vec2, equation: &str, size: f32, color: Color) -> Self {
        Self {
            position: Signal::new(pos),
            equation: Signal::new(equation.to_string()),
            font_size: Signal::new(size),
            color: Signal::new(color),
            opacity: Signal::new(1.0),
            transition_progress: Signal::new(1.0),
            cache: Mutex::new(None),
            prev_cache: Mutex::new(None),
        }
    }

    pub fn tex(&self, equation: &str, duration: Duration) -> Box<dyn crate::engine::animation::Animation> {
        use crate::engine::animation::{wait, Animation};
        let prev_eq = self.equation.get();
        let target_eq = equation.to_string();
        
        // If the equation is the same, just return an empty animation
        if prev_eq == target_eq {
            return crate::all![wait(Duration::ZERO)];
        }

        // 1. Move current cache to prev_cache
        {
            let cache = self.cache.lock().unwrap();
            let mut prev = self.prev_cache.lock().unwrap();
            *prev = cache.clone();
        }

        // 2. Start transition
        self.transition_progress.set(0.0);
        self.equation.set(target_eq);

        // 3. Return an all! animation that tweens transition_progress
        crate::all![
            self.transition_progress.to(1.0, duration)
        ]
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

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

impl Node for MathNode {
    fn render(&self, scene: &mut Scene) {
        let pos = self.position.get();
        let eq = self.equation.get();
        let size = self.font_size.get();
        let color = self.color.get();

        let mut cache = self.cache.lock().unwrap();
        let needs_rebuild = cache.as_ref().map_or(true, |c| {
            c.equation != eq || c.font_size != size || c.color != color
        });

        if needs_rebuild {
            let mut paths = Vec::new();
            let typst_code = format!("#set text(size: {}pt)\n#show math.equation: set text(font: \"DejaVu Math TeX Gyre\")\n$ {} $", size, eq);
            let world = crate::engine::typst_support::TypstWorld::new(&typst_code);
            let output = typst::compile::<typst::layout::PagedDocument>(&world).output;
            match output {
                Ok(document) => {
                    if let Some(page) = document.pages.first() {
                        let frame = &page.frame;
                        for (p, item) in frame.items() {
                            match item {
                                typst::layout::FrameItem::Text(text) => {
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
                                            p.x.to_pt() + glyph.x_offset.at(text.size).to_pt(), 
                                            p.y.to_pt() + glyph.y_offset.at(text.size).to_pt() + text.size.to_pt()
                                        )) * Affine::scale_non_uniform(1.0, -1.0);
                                        paths.push((transform, color, pb));
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                Err(errors) => {
                    eprintln!("Typst compilation failed for equation '{}':", eq);
                    for err in errors {
                        eprintln!("  - {}", err.message);
                    }
                }
            }
            *cache = Some(MathCache {
                equation: eq.clone(),
                font_size: size,
                color,
                paths,
            });
        }

        if let Some(c) = cache.as_ref() {
            let progress = self.transition_progress.get();
            let base_opacity = self.opacity.get();
            let root_transform = Affine::translate((pos.x as f64, pos.y as f64));
            
            // Draw previous equation if transitioning
            if progress < 1.0 {
                if let Some(prev) = self.prev_cache.lock().unwrap().as_ref() {
                    let mut prev_color = color;
                    prev_color.a = (color.a as f32 * base_opacity * (1.0 - progress)) as u8;
                    for (local_transform, _c, pb) in &prev.paths {
                        scene.fill(Fill::NonZero, root_transform * *local_transform, &Brush::Solid(prev_color), None, pb);
                    }
                }
            }

            // Draw current equation
            let mut current_color = color;
            let current_alpha = if progress < 1.0 {
                base_opacity * progress
            } else {
                base_opacity
            };
            current_color.a = (color.a as f32 * current_alpha) as u8;

            for (local_transform, _cached_color, pb) in &c.paths {
                scene.fill(Fill::NonZero, root_transform * *local_transform, &Brush::Solid(current_color), None, pb);
            }
        }
    }
    fn update(&mut self, _dt: Duration) {}
    fn state_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        let pos = self.position.get();
        let eq = self.equation.get();
        let size = self.font_size.get();
        let color = self.color.get();
        
        pos.x.to_bits().hash(&mut s);
        pos.y.to_bits().hash(&mut s);
        eq.hash(&mut s);
        size.to_bits().hash(&mut s);
        color.r.hash(&mut s);
        color.g.hash(&mut s);
        color.b.hash(&mut s);
        color.a.hash(&mut s);
        self.opacity.get().to_bits().hash(&mut s);
        self.transition_progress.get().to_bits().hash(&mut s);
        
        s.finish()
    }
}
