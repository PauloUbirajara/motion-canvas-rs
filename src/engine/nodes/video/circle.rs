use crate::engine::animation::{Node, Signal};
use glam::Vec2;
use std::time::Duration;
use vello::kurbo::{Affine, Circle as KurboCircle};
use vello::peniko::{Brush, Color, Fill};
use vello::Scene;

const DEFAULT_RADIUS: f32 = 50.0;
const DEFAULT_COLOR: Color = Color::rgb8(9, 9, 11); // Zinc 950
const DEFAULT_STROKE_COLOR: Color = Color::rgba8(250, 250, 250, 25); // 10% Zinc 50
const DEFAULT_STROKE_WIDTH: f32 = 1.0;
const DEFAULT_OPACITY: f32 = 1.0;

#[derive(Clone)]
pub struct Circle {
    pub position: Signal<Vec2>,
    pub rotation: Signal<f32>,
    pub scale: Signal<Vec2>,
    pub radius: Signal<f32>,
    pub color: Signal<Color>,
    pub stroke_color: Signal<Color>,
    pub stroke_width: Signal<f32>,
    pub opacity: Signal<f32>,
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            position: Signal::new(Vec2::ZERO),
            rotation: Signal::new(0.0),
            scale: Signal::new(Vec2::ONE),
            radius: Signal::new(DEFAULT_RADIUS),
            color: Signal::new(DEFAULT_COLOR),
            stroke_color: Signal::new(DEFAULT_STROKE_COLOR),
            stroke_width: Signal::new(DEFAULT_STROKE_WIDTH),
            opacity: Signal::new(DEFAULT_OPACITY),
        }
    }
}

impl Circle {
    pub fn new(position: Vec2, radius: f32, color: Color) -> Self {
        Self::default()
            .with_position(position)
            .with_radius(radius)
            .with_color(color)
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

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = Signal::new(radius);
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Signal::new(color);
        self
    }

    pub fn with_fill(mut self, color: Color) -> Self {
        self.color = Signal::new(color);
        self
    }

    pub fn with_stroke(mut self, color: Color, width: f32) -> Self {
        self.stroke_color = Signal::new(color);
        self.stroke_width = Signal::new(width);
        self
    }
}

impl Node for Circle {
    fn render(&self, scene: &mut Scene, parent_transform: Affine, parent_opacity: f32) {
        let radius = self.radius.get();
        let fill_color = self.color.get();
        let stroke_color = self.stroke_color.get();
        let stroke_width = self.stroke_width.get();
        let opacity = self.opacity.get();

        let pos = self.position.get();
        let rot = self.rotation.get();
        let sc = self.scale.get();

        let local_transform = Affine::translate((pos.x as f64, pos.y as f64))
            * Affine::rotate(rot as f64)
            * Affine::scale_non_uniform(sc.x as f64, sc.y as f64);

        let combined_transform = parent_transform * local_transform;
        let combined_opacity = parent_opacity * opacity;

        let circle = KurboCircle::new((0.0, 0.0), radius as f64);

        // Fill
        let mut final_color = fill_color;
        final_color.a = (fill_color.a as f32 * combined_opacity).clamp(0.0, 255.0) as u8;
        scene.fill(
            Fill::NonZero,
            combined_transform,
            &Brush::Solid(final_color),
            None,
            &circle,
        );

        // Stroke
        if stroke_width > 0.001 {
            let mut final_stroke = stroke_color;
            final_stroke.a = (stroke_color.a as f32 * combined_opacity).clamp(0.0, 255.0) as u8;
            scene.stroke(
                &vello::kurbo::Stroke::new(stroke_width as f64),
                combined_transform,
                &Brush::Solid(final_stroke),
                None,
                &circle,
            );
        }
    }
    fn update(&mut self, _dt: Duration) {}
    fn state_hash(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut s = DefaultHasher::new();

        let pos = self.position.get();
        pos.x.to_bits().hash(&mut s);
        pos.y.to_bits().hash(&mut s);

        self.rotation.get().to_bits().hash(&mut s);

        let sc = self.scale.get();
        sc.x.to_bits().hash(&mut s);
        sc.y.to_bits().hash(&mut s);

        self.radius.get().to_bits().hash(&mut s);
        let color = self.color.get();
        color.r.hash(&mut s);
        color.g.hash(&mut s);
        color.b.hash(&mut s);
        color.a.hash(&mut s);

        let s_color = self.stroke_color.get();
        s_color.r.hash(&mut s);
        s_color.g.hash(&mut s);
        s_color.b.hash(&mut s);
        s_color.a.hash(&mut s);

        self.stroke_width.get().to_bits().hash(&mut s);
        self.opacity.get().to_bits().hash(&mut s);
        s.finish()
    }

    fn clone_node(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }
}
