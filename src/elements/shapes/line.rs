#![allow(deprecated)]

use crate::core::animation::{Node, Paint, Signal};
use glam::Vec2;
use kurbo::{Affine, Line as KurboLine, Stroke};
use peniko::{Brush, Color};
use std::time::Duration;
#[cfg(feature = "runtime")]
use vello::Scene;

const DEFAULT_START: Vec2 = Vec2::ZERO;
const DEFAULT_END: Vec2 = Vec2::new(100.0, 0.0);
const DEFAULT_COLOR: Color = Color::rgba8(250, 250, 250, 25);
const DEFAULT_WIDTH: f32 = 1.0;
const DEFAULT_OPACITY: f32 = 1.0;

/// A straight line between two points.
///
/// `Line` is used to draw paths, connections, or as a simple linear highlight.
///
/// ### Example
/// ```rust
/// # use motion_canvas_rs::prelude::*;
/// let line = Line::default()
///     .with_position(Vec2::new(640.0, 360.0))
///     .with_start(Vec2::ZERO)
///     .with_end(Vec2::new(100.0, 100.0))
///     .with_stroke(Color::WHITE, 2.0);
/// ```
#[derive(Clone)]
pub struct Line {
    /// The absolute position of the line (before anchor adjustment).
    pub position: Signal<Vec2>,
    /// Rotation in radians.
    pub rotation: Signal<f32>,
    /// Scaling factor for the line.
    pub scale: Signal<Vec2>,
    /// The starting point of the line relative to its position.
    pub start: Signal<Vec2>,
    /// The ending point of the line relative to its position.
    pub end: Signal<Vec2>,
    /// The color of the line stroke.
    /// **Deprecated**: prefer `stroke_paint` which supports both solid colors and gradients.
    #[deprecated(since = "0.2.3", note = "use stroke_paint instead")]
    pub stroke_color: Signal<Color>,
    /// The paint (color or gradient) used for the line stroke.
    pub stroke_paint: Signal<Option<Paint>>,
    /// The width of the line stroke.
    pub stroke_width: Signal<f32>,
    /// Opacity from 0.0 (transparent) to 1.0 (opaque).
    pub opacity: Signal<f32>,
    /// The relative transformation origin. (-1,-1) is top-left, (0,0) is center, (1,1) is bottom-right.
    pub anchor: Signal<Vec2>,
}

impl Default for Line {
    fn default() -> Self {
        Self {
            position: Signal::new(Vec2::ZERO),
            rotation: Signal::new(0.0),
            scale: Signal::new(Vec2::ONE),
            start: Signal::new(DEFAULT_START),
            end: Signal::new(DEFAULT_END),
            stroke_color: Signal::new(DEFAULT_COLOR),
            stroke_paint: Signal::new(None),
            stroke_width: Signal::new(DEFAULT_WIDTH),
            opacity: Signal::new(DEFAULT_OPACITY),
            anchor: Signal::new(Vec2::ZERO),
        }
    }
}

impl Line {
    /// Creates a new line with given start/end points, color, and width.
    pub fn new(start: Vec2, end: Vec2, color: Color, width: f32) -> Self {
        Self::default()
            .with_start(start)
            .with_end(end)
            .with_stroke(color, width)
    }

    /// Sets the absolute position of the line.
    pub fn with_position(mut self, position: Vec2) -> Self {
        self.position = Signal::new(position);
        self
    }

    /// Sets the rotation of the line in radians.
    pub fn with_rotation(mut self, angle: f32) -> Self {
        self.rotation = Signal::new(angle);
        self
    }

    /// Sets a uniform scale factor for both axes.
    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = Signal::new(Vec2::splat(scale));
        self
    }

    /// Sets non-uniform scaling factors for X and Y axes.
    pub fn with_scale_xy(mut self, scale: Vec2) -> Self {
        self.scale = Signal::new(scale);
        self
    }

    /// Sets the opacity of the line (0.0 to 1.0).
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = Signal::new(opacity);
        self
    }

    /// Sets the relative starting point of the line.
    pub fn with_start(mut self, start: Vec2) -> Self {
        self.start = Signal::new(start);
        self
    }

    /// Sets the relative ending point of the line.
    pub fn with_end(mut self, end: Vec2) -> Self {
        self.end = Signal::new(end);
        self
    }

    /// Sets the stroke paint and width for the line.
    pub fn with_stroke(mut self, paint: impl Into<Paint>, width: f32) -> Self {
        let p = paint.into();
        if let Paint::Solid(color) = p {
            self.stroke_color = Signal::new(color);
        }
        self.stroke_paint = Signal::new(Some(p));
        self.stroke_width = Signal::new(width);
        self
    }

    /// Sets the relative transformation origin (anchor).
    /// (-1, -1) is top-left, (0, 0) is center, (1, 1) is bottom-right.
    pub fn with_anchor(mut self, anchor: Vec2) -> Self {
        self.anchor = Signal::new(anchor);
        self
    }

    /// Sets the width of the line stroke.
    pub fn with_stroke_width(mut self, width: f32) -> Self {
        self.stroke_width = Signal::new(width);
        self
    }
}

impl Node for Line {
    #[cfg(feature = "runtime")]
    fn render(&self, scene: &mut Scene, parent_transform: Affine, parent_opacity: f32) {
        let stroke_color = self.stroke_color.get();
        let stroke_width = self.stroke_width.get();
        let opacity = self.opacity.get();

        let pos = self.position.get();
        let rot = self.rotation.get();
        let sc = self.scale.get();
        let anchor = self.anchor.get();

        let start = self.start.get();
        let end = self.end.get();

        // Calculate bounding box for centering and anchor
        let min_x = start.x.min(end.x);
        let min_y = start.y.min(end.y);
        let max_x = start.x.max(end.x);
        let max_y = start.y.max(end.y);

        let size_vec = Vec2::new(max_x - min_x, max_y - min_y);

        let anchor_offset = anchor * size_vec * 0.5;

        let local_transform = Affine::translate((pos.x as f64, pos.y as f64))
            * Affine::rotate(rot as f64)
            * Affine::scale_non_uniform(sc.x as f64, sc.y as f64)
            * Affine::translate((-anchor_offset.x as f64, -anchor_offset.y as f64));

        let combined_transform = parent_transform * local_transform;
        let combined_opacity = parent_opacity * opacity;

        let brush = match self.stroke_paint.get() {
            Some(paint) => paint.to_brush_with_opacity(combined_opacity),
            None => {
                let mut final_color = stroke_color;
                final_color.a = (stroke_color.a as f32 * combined_opacity).clamp(0.0, 255.0) as u8;
                Brush::Solid(final_color)
            }
        };

        scene.stroke(
            &Stroke::new(stroke_width as f64),
            combined_transform,
            &brush,
            None,
            &KurboLine::new(
                (start.x as f64, start.y as f64),
                (end.x as f64, end.y as f64),
            ),
        );
    }
    fn update(&mut self, _dt: Duration) {}
    fn state_hash(&self) -> u64 {
        use crate::assets::hash::Hasher;
        let mut h = Hasher::new();
        h.update_u64(self.position.state_hash());
        h.update_u64(self.rotation.state_hash());
        h.update_u64(self.scale.state_hash());
        h.update_u64(self.start.state_hash());
        h.update_u64(self.end.state_hash());
        h.update_u64(self.stroke_width.state_hash());
        h.update_u64(self.stroke_color.state_hash());
        h.update_u64(self.stroke_paint.state_hash());
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
        self.start.reset();
        self.end.reset();
        self.stroke_width.reset();
        self.stroke_color.reset();
        self.stroke_paint.reset();
        self.opacity.reset();
        self.anchor.reset();
    }
}
