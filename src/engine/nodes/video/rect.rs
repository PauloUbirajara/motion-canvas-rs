use crate::engine::animation::{Node, Signal};
use glam::Vec2;
use std::time::Duration;
use vello::kurbo::{Affine, RoundedRect as KurboRoundedRect};
use vello::peniko::{Brush, Color, Fill};
use vello::Scene;

const DEFAULT_SIZE: Vec2 = Vec2::new(100.0, 100.0);
const DEFAULT_COLOR: Color = Color::rgb8(9, 9, 11);
const DEFAULT_RADIUS: f32 = 12.0;
const DEFAULT_STROKE_COLOR: Color = Color::rgba8(250, 250, 250, 25);
const DEFAULT_STROKE_WIDTH: f32 = 1.0;
const DEFAULT_OPACITY: f32 = 1.0;

#[derive(Clone)]
pub struct Rect {
    pub position: Signal<Vec2>,
    pub rotation: Signal<f32>,
    pub scale: Signal<Vec2>,
    pub size: Signal<Vec2>,
    pub fill_color: Signal<Color>,
    pub stroke_color: Signal<Color>,
    pub stroke_width: Signal<f32>,
    pub radius: Signal<f32>,
    pub opacity: Signal<f32>,
    pub anchor: Signal<Vec2>,
}

impl Default for Rect {
    fn default() -> Self {
        Self {
            position: Signal::new(Vec2::ZERO),
            rotation: Signal::new(0.0),
            scale: Signal::new(Vec2::ONE),
            size: Signal::new(DEFAULT_SIZE),
            fill_color: Signal::new(DEFAULT_COLOR),
            stroke_color: Signal::new(DEFAULT_STROKE_COLOR),
            stroke_width: Signal::new(DEFAULT_STROKE_WIDTH),
            radius: Signal::new(DEFAULT_RADIUS),
            opacity: Signal::new(DEFAULT_OPACITY),
            anchor: Signal::new(Vec2::ZERO),
        }
    }
}

impl Rect {
    pub fn new(position: Vec2, size: Vec2, color: Color) -> Self {
        Self::default()
            .with_position(position)
            .with_size(size)
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

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = Signal::new(radius);
        self
    }

    pub fn with_size(mut self, size: Vec2) -> Self {
        self.size = Signal::new(size);
        self
    }

    pub fn with_fill(mut self, color: Color) -> Self {
        self.fill_color = Signal::new(color);
        self
    }

    pub fn with_stroke(mut self, color: Color, width: f32) -> Self {
        self.stroke_color = Signal::new(color);
        self.stroke_width = Signal::new(width);
        self
    }

    /// Sets the relative transformation origin (anchor).
    /// (-1, -1) is top-left, (0, 0) is center, (1, 1) is bottom-right.
    pub fn with_anchor(mut self, anchor: Vec2) -> Self {
        self.anchor = Signal::new(anchor);
        self
    }
}

impl Node for Rect {
    fn render(&self, scene: &mut Scene, parent_transform: Affine, parent_opacity: f32) {
        let size = self.size.get();
        let fill_color_val = self.fill_color.get();
        let stroke_color = self.stroke_color.get();
        let stroke_width = self.stroke_width.get();
        let radius = self.radius.get();
        let pos = self.position.get();
        let rot = self.rotation.get();
        let sc = self.scale.get();
        let anchor = self.anchor.get();

        let opacity = self.opacity.get();

        let anchor_offset = anchor * size * 0.5;

        let local_transform = Affine::translate((pos.x as f64, pos.y as f64))
            * Affine::rotate(rot as f64)
            * Affine::scale_non_uniform(sc.x as f64, sc.y as f64)
            * Affine::translate((-anchor_offset.x as f64, -anchor_offset.y as f64));

        let combined_transform = parent_transform * local_transform;
        let combined_opacity = parent_opacity * opacity;

        let rect = KurboRoundedRect::new(
            -size.x as f64 / 2.0,
            -size.y as f64 / 2.0,
            size.x as f64 / 2.0,
            size.y as f64 / 2.0,
            radius as f64,
        );

        // Fill
        let mut final_color = fill_color_val;
        final_color.a = (fill_color_val.a as f32 * combined_opacity).clamp(0.0, 255.0) as u8;
        scene.fill(
            Fill::NonZero,
            combined_transform,
            &Brush::Solid(final_color),
            None,
            &rect,
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
                &rect,
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
        h.update_u64(self.size.state_hash());
        h.update_u64(self.radius.state_hash());
        h.update_u64(self.fill_color.state_hash());
        h.update_u64(self.stroke_color.state_hash());
        h.update_u64(self.stroke_width.state_hash());
        h.update_u64(self.opacity.state_hash());
        h.update_u64(self.anchor.state_hash());
        h.finish()
    }

    fn clone_node(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn reset(&mut self) {
        self.position.reset();
        self.rotation.reset();
        self.scale.reset();
        self.size.reset();
        self.radius.reset();
        self.fill_color.reset();
        self.stroke_color.reset();
        self.stroke_width.reset();
        self.opacity.reset();
        self.anchor.reset();
    }
}
