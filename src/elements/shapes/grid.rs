#![allow(deprecated)]

use crate::assets::hash::Hasher;
use crate::core::animation::base::Node;
use crate::core::animation::paint::Paint;
use crate::core::animation::tween::Signal;
use glam::Vec2;
use kurbo::{Affine, BezPath, Stroke};
use peniko::{Brush, Color};
use std::time::Duration;
#[cfg(feature = "runtime")]
use vello::Scene;

/// A procedural grid shape.
///
/// `GridNode` draws a series of vertical and horizontal lines to form a grid,
/// centered around its position.
///
/// ### Example
/// ```rust
/// # use motion_canvas_rs::prelude::*;
/// let grid = GridNode::default()
///     .with_position(Vec2::new(640.0, 360.0))
///     .with_columns(20.0)
///     .with_rows(20.0)
///     .with_spacing_all(40.0)
///     .with_stroke(Color::rgb8(40, 40, 40), 1.0);
/// ```
#[derive(Clone)]
pub struct GridNode {
    /// The absolute position of the grid's center.
    pub position: Signal<Vec2>,
    /// The number of vertical columns.
    pub columns: Signal<f32>,
    /// The number of horizontal rows.
    pub rows: Signal<f32>,
    /// The distance between adjacent grid lines.
    pub spacing: Signal<Vec2>,
    /// The color of the grid lines.
    /// **Deprecated**: prefer `stroke_paint` which supports both solid colors and gradients.
    #[deprecated(since = "0.2.3", note = "use stroke_paint instead")]
    pub stroke_color: Signal<Color>,
    /// The paint (color or gradient) used for the grid lines.
    pub stroke_paint: Signal<Paint>,
    /// The width of the grid lines.
    pub stroke_width: Signal<f32>,
    /// Opacity from 0.0 (transparent) to 1.0 (opaque).
    pub opacity: Signal<f32>,
    /// Blur radius signal.
    pub blur: Signal<f32>,
}

impl Default for GridNode {
    fn default() -> Self {
        Self {
            position: Signal::new(Vec2::ZERO),
            columns: Signal::new(10.0),
            rows: Signal::new(10.0),
            spacing: Signal::new(Vec2::new(50.0, 50.0)),
            stroke_color: Signal::new(Color::rgb8(100, 100, 100)),
            stroke_paint: Signal::new(Paint::None),
            stroke_width: Signal::new(1.0),
            opacity: Signal::new(1.0),
            blur: Signal::new(crate::core::filters::DEFAULT_BLUR),
        }
    }
}

impl GridNode {
    /// Creates a new grid at the given position with default settings.
    pub fn new(position: Vec2) -> Self {
        Self::default().with_position(position)
    }

    /// Creates a square grid with equal columns/rows and uniform spacing.
    pub fn square(position: Vec2, count: f32, spacing: f32) -> Self {
        Self::default()
            .with_position(position)
            .with_columns(count)
            .with_rows(count)
            .with_spacing(Vec2::splat(spacing))
    }

    /// Sets the absolute position of the grid.
    pub fn with_position(mut self, pos: Vec2) -> Self {
        self.position = Signal::new(pos);
        self
    }

    /// Sets the number of columns.
    pub fn with_columns(mut self, cols: f32) -> Self {
        self.columns = Signal::new(cols);
        self
    }

    /// Sets the number of rows.
    pub fn with_rows(mut self, rows: f32) -> Self {
        self.rows = Signal::new(rows);
        self
    }

    /// Sets the spacing between grid lines.
    pub fn with_spacing(mut self, spacing: Vec2) -> Self {
        self.spacing = Signal::new(spacing);
        self
    }

    /// Sets uniform spacing between grid lines on both axes.
    pub fn with_spacing_all(mut self, spacing: f32) -> Self {
        self.spacing = Signal::new(Vec2::splat(spacing));
        self
    }

    /// Sets the stroke paint and width for the grid lines.
    pub fn with_stroke(mut self, paint: impl Into<Paint>, width: f32) -> Self {
        let p = paint.into();
        if let Paint::Solid(color) = p {
            self.stroke_color = Signal::new(color);
        }
        self.stroke_paint = Signal::new(p);
        self.stroke_width = Signal::new(width);
        self
    }

    /// Sets the opacity of the grid (0.0 to 1.0).
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = Signal::new(opacity);
        self
    }
}

impl crate::core::filters::Blur for GridNode {
    fn with_blur(mut self, radius: f32) -> Self {
        self.blur = Signal::new(radius);
        self
    }
}

impl Node for GridNode {
    #[cfg(feature = "runtime")]
    fn render(&self, scene: &mut Scene, parent_transform: Affine, parent_opacity: f32) {
        let blur_radius = self.blur.get().max(0.0);
        let opacity = self.opacity.get();
        let combined_opacity = parent_opacity * opacity;

        crate::core::filters::apply_blur_filter(
            scene,
            blur_radius,
            combined_opacity,
            |scene, target_opacity| {
                let pos = self.position.get();
                let cols = self.columns.get().max(0.0);
                let rows = self.rows.get().max(0.0);
                let spacing = self.spacing.get();
                let stroke_width = self.stroke_width.get();

                let transform = parent_transform * Affine::translate((pos.x as f64, pos.y as f64));
                let brush = match self.stroke_paint.get() {
                    Paint::None => {
                        let mut stroke_color = self.stroke_color.get();
                        stroke_color.a =
                            (stroke_color.a as f32 * target_opacity).clamp(0.0, 255.0) as u8;
                        Brush::Solid(stroke_color)
                    }
                    paint => paint.to_brush_with_opacity(target_opacity),
                };
                let stroke = Stroke::new(stroke_width as f64);

                let width = cols * spacing.x;
                let height = rows * spacing.y;
                let start_x = -width / 2.0;
                let start_y = -height / 2.0;

                // Vertical lines
                for i in 0..=(cols.ceil() as i32) {
                    let x = start_x + i as f32 * spacing.x;
                    if x > width / 2.0 {
                        break;
                    }
                    let mut path = BezPath::new();
                    path.move_to((x as f64, start_y as f64));
                    path.line_to((x as f64, (start_y + height) as f64));
                    scene.stroke(&stroke, transform, &brush, None, &path);
                }

                // Horizontal lines
                for i in 0..=(rows.ceil() as i32) {
                    let y = start_y + i as f32 * spacing.y;
                    if y > height / 2.0 {
                        break;
                    }
                    let mut path = BezPath::new();
                    path.move_to((start_x as f64, y as f64));
                    path.line_to(((start_x + width) as f64, y as f64));
                    scene.stroke(&stroke, transform, &brush, None, &path);
                }
            },
        );
    }

    fn update(&mut self, _dt: Duration) {}

    fn state_hash(&self) -> u64 {
        let mut h = Hasher::new();
        h.update_u64(self.position.state_hash());
        h.update_u64(self.columns.state_hash());
        h.update_u64(self.rows.state_hash());
        h.update_u64(self.spacing.state_hash());
        h.update_u64(self.stroke_color.state_hash());
        h.update_u64(self.stroke_paint.state_hash());
        h.update_u64(self.stroke_width.state_hash());
        h.update_u64(self.opacity.state_hash());
        h.update_u64(self.blur.state_hash());
        h.finish()
    }

    fn clone_node(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn reset(&mut self) {
        self.position.reset();
        self.columns.reset();
        self.rows.reset();
        self.spacing.reset();
        self.stroke_color.reset();
        self.stroke_paint.reset();
        self.stroke_width.reset();
        self.opacity.reset();
        self.blur.reset();
    }
}
