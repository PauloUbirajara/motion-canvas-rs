use crate::engine::animation::{Signal, Node};
use vello::peniko::Color;
use vello::Scene;
use glam::Vec2;
use vello::kurbo::Affine;
use std::time::Duration;

#[derive(Clone)]
pub struct MathNode {
    pub transform: Signal<Affine>,
    pub opacity: Signal<f32>,
    pub tex: Signal<String>,
    pub font_size: Signal<f32>,
    pub color: Color,
    pub transition_progress: Signal<f32>,
}

impl MathNode {
    pub fn new(pos: Vec2, tex: &str, font_size: f32, color: Color) -> Self {
        Self {
            transform: Signal::new(Affine::translate((pos.x as f64, pos.y as f64))),
            opacity: Signal::new(1.0),
            tex: Signal::new(tex.to_string()),
            font_size: Signal::new(font_size),
            color,
            transition_progress: Signal::new(1.0),
        }
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

impl Node for MathNode {
    fn render(&self, scene: &mut Scene, parent_transform: Affine, parent_opacity: f32) {
        let local_transform = self.transform.get();
        let opacity = self.opacity.get() * parent_opacity;
        
        let combined_transform = parent_transform * local_transform;
        
        if opacity <= 0.0 {
            return;
        }

        let tex = self.tex.get();
        let font_size = self.font_size.get();

        // Render math via typst
        if let Ok(paths) = crate::engine::font::render_math(&tex, font_size) {
            for (path, color) in paths {
                let color = color.unwrap_or(self.color);
                scene.fill(
                    vello::peniko::Fill::NonZero,
                    combined_transform,
                    color.with_alpha_factor(opacity),
                    None,
                    &path,
                );
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
        self.tex.get().hash(&mut s);
        self.font_size.get().to_bits().hash(&mut s);
        let color = self.color;
        color.r.hash(&mut s);
        color.g.hash(&mut s);
        color.b.hash(&mut s);
        color.a.hash(&mut s);
        self.opacity.get().to_bits().hash(&mut s);
        self.transition_progress.get().to_bits().hash(&mut s);
        s.finish()
    }

    fn clone_node(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
