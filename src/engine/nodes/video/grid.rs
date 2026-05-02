use crate::engine::animation::{Node, Signal};
use glam::Vec2;
use std::time::Duration;
use vello::kurbo::{Affine, Line as KurboLine};
use vello::peniko::{Brush, Color};
use vello::Scene;

const DEFAULT_ROWS: f32 = 10.0;
const DEFAULT_COLUMNS: f32 = 10.0;
const DEFAULT_SPACING: Vec2 = Vec2::new(100.0, 100.0);
const DEFAULT_STROKE_COLOR: Color = Color::rgba8(250, 250, 250, 50);
const DEFAULT_STROKE_WIDTH: f32 = 1.0;
const DEFAULT_OPACITY: f32 = 1.0;

#[derive(Clone)]
pub struct GridNode {
    pub position: Signal<Vec2>,
    pub rotation: Signal<f32>,
    pub scale: Signal<Vec2>,
    pub rows: Signal<f32>,
    pub columns: Signal<f32>,
    pub spacing: Signal<Vec2>,
    pub stroke_color: Signal<Color>,
    pub stroke_width: Signal<f32>,
    pub opacity: Signal<f32>,
    pub anchor: Signal<Vec2>,
}

impl Default for GridNode {
    fn default() -> Self {
        Self {
            position: Signal::new(Vec2::ZERO),
            rotation: Signal::new(0.0),
            scale: Signal::new(Vec2::ONE),
            rows: Signal::new(DEFAULT_ROWS),
            columns: Signal::new(DEFAULT_COLUMNS),
            spacing: Signal::new(DEFAULT_SPACING),
            stroke_color: Signal::new(DEFAULT_STROKE_COLOR),
            stroke_width: Signal::new(DEFAULT_STROKE_WIDTH),
            opacity: Signal::new(DEFAULT_OPACITY),
            anchor: Signal::new(Vec2::ZERO),
        }
    }
}

impl GridNode {
    pub fn new(position: Vec2, rows: f32, columns: f32, spacing: Vec2) -> Self {
        Self::default()
            .with_position(position)
            .with_rows(rows)
            .with_columns(columns)
            .with_spacing(spacing)
    }

    pub fn square(position: Vec2, size: f32, spacing: f32) -> Self {
        Self::new(position, size, size, Vec2::splat(spacing))
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

    pub fn with_rows(mut self, rows: f32) -> Self {
        self.rows = Signal::new(rows);
        self
    }

    pub fn with_columns(mut self, columns: f32) -> Self {
        self.columns = Signal::new(columns);
        self
    }

    pub fn with_spacing(mut self, spacing: Vec2) -> Self {
        self.spacing = Signal::new(spacing);
        self
    }

    pub fn with_stroke(mut self, color: Color, width: f32) -> Self {
        self.stroke_color = Signal::new(color);
        self.stroke_width = Signal::new(width);
        self
    }

    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = Signal::new(opacity);
        self
    }

    pub fn with_anchor(mut self, anchor: Vec2) -> Self {
        self.anchor = Signal::new(anchor);
        self
    }
}

impl Node for GridNode {
    fn render(&self, scene: &mut Scene, parent_transform: Affine, parent_opacity: f32) {
        let rows = self.rows.get();
        let columns = self.columns.get();
        let spacing = self.spacing.get();
        let stroke_color = self.stroke_color.get();
        let stroke_width = self.stroke_width.get();
        let pos = self.position.get();
        let rot = self.rotation.get();
        let sc = self.scale.get();
        let anchor = self.anchor.get();
        let opacity = self.opacity.get();

        let size = Vec2::new(columns * spacing.x, rows * spacing.y);
        let anchor_offset = anchor * size * 0.5;

        let local_transform = Affine::translate((pos.x as f64, pos.y as f64))
            * Affine::rotate(rot as f64)
            * Affine::scale_non_uniform(sc.x as f64, sc.y as f64)
            * Affine::translate((-anchor_offset.x as f64, -anchor_offset.y as f64));

        let combined_transform = parent_transform * local_transform;
        let combined_opacity = parent_opacity * opacity;

        if stroke_width < 0.001 || combined_opacity < 0.001 || spacing.x <= 0.0 || spacing.y <= 0.0
        {
            return;
        }

        let mut final_stroke = stroke_color;
        final_stroke.a = (stroke_color.a as f32 * combined_opacity).clamp(0.0, 255.0) as u8;
        let brush = Brush::Solid(final_stroke);
        let stroke = vello::kurbo::Stroke::new(stroke_width as f64);

        let half_w = size.x as f64 / 2.0;
        let half_h = size.y as f64 / 2.0;

        let start_x = -half_w;
        let start_y = -half_h;
        let end_x = half_w;
        let end_y = half_h;

        // Vertical lines
        let mut x = start_x;
        while x <= end_x {
            let line = KurboLine::new((x, start_y), (x, end_y));
            scene.stroke(&stroke, combined_transform, &brush, None, &line);
            x += spacing.x as f64;
        }

        // Horizontal lines
        let mut y = start_y;
        while y <= end_y {
            let line = KurboLine::new((start_x, y), (end_x, y));
            scene.stroke(&stroke, combined_transform, &brush, None, &line);
            y += spacing.y as f64;
        }
    }

    fn update(&mut self, _dt: Duration) {}

    fn state_hash(&self) -> u64 {
        use crate::engine::util::hash::Hasher;
        let mut h = Hasher::new();
        h.update_u64(self.position.state_hash());
        h.update_u64(self.rotation.state_hash());
        h.update_u64(self.scale.state_hash());
        h.update_u64(self.rows.state_hash());
        h.update_u64(self.columns.state_hash());
        h.update_u64(self.spacing.state_hash());
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
        self.rows.reset();
        self.columns.reset();
        self.spacing.reset();
        self.stroke_color.reset();
        self.stroke_width.reset();
        self.opacity.reset();
        self.anchor.reset();
    }
}
