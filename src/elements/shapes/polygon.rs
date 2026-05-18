#![allow(deprecated)]

use crate::core::animation::{Node, Paint, Signal};
use glam::Vec2;
use kurbo::{Affine, BezPath, Stroke};
use peniko::{Brush, Color, Fill};
use std::time::Duration;
#[cfg(feature = "runtime")]
use vello::Scene;

const DEFAULT_FILL_COLOR: Color = Color::rgb8(9, 9, 11);
const DEFAULT_STROKE_COLOR: Color = Color::rgba8(250, 250, 250, 25);
const DEFAULT_STROKE_WIDTH: f32 = 1.0;
const DEFAULT_OPACITY: f32 = 1.0;

/// A shape defined by a sequence of points that form a closed loop.
///
/// `Polygon` can be used for custom shapes, stars, or regular polygons (via the `regular` constructor).
///
/// ### Example
/// ```rust
/// # use motion_canvas_rs::prelude::*;
/// let triangle = Polygon::default()
///     .with_position(Vec2::new(640.0, 360.0))
///     .with_points(vec![
///         Vec2::new(0.0, -50.0),
///         Vec2::new(50.0, 50.0),
///         Vec2::new(-50.0, 50.0),
///     ])
///     .with_fill(Color::RED);
/// ```
#[derive(Clone)]
pub struct Polygon {
    /// The absolute position of the polygon's center (before anchor adjustment).
    pub position: Signal<Vec2>,
    /// Rotation in radians.
    pub rotation: Signal<f32>,
    /// Scaling factor for the polygon.
    pub scale: Signal<Vec2>,
    /// The list of vertices that define the polygon.
    pub points: Signal<Vec<Vec2>>,
    /// The solid color used to fill the polygon.
    /// **Deprecated**: prefer `fill_paint` which supports both solid colors and gradients.
    #[deprecated(since = "0.2.3", note = "use fill_paint instead")]
    pub fill_color: Signal<Color>,
    /// The paint (color or gradient) used to fill the polygon.
    pub fill_paint: Signal<Option<Paint>>,
    /// The color of the border stroke.
    /// **Deprecated**: prefer `stroke_paint` which supports both solid colors and gradients.
    #[deprecated(since = "0.2.3", note = "use stroke_paint instead")]
    pub stroke_color: Signal<Color>,
    /// The paint (color or gradient) used for the border stroke.
    pub stroke_paint: Signal<Option<Paint>>,
    /// The width of the border stroke.
    pub stroke_width: Signal<f32>,
    /// Opacity from 0.0 (transparent) to 1.0 (opaque).
    pub opacity: Signal<f32>,
    /// The relative transformation origin. (-1,-1) is top-left, (0,0) is center, (1,1) is bottom-right.
    pub anchor: Signal<Vec2>,
}

impl Default for Polygon {
    fn default() -> Self {
        Self {
            position: Signal::new(Vec2::ZERO),
            rotation: Signal::new(0.0),
            scale: Signal::new(Vec2::ONE),
            points: Signal::new(Vec::new()),
            fill_color: Signal::new(DEFAULT_FILL_COLOR),
            fill_paint: Signal::new(None),
            stroke_color: Signal::new(DEFAULT_STROKE_COLOR),
            stroke_paint: Signal::new(None),
            stroke_width: Signal::new(DEFAULT_STROKE_WIDTH),
            opacity: Signal::new(DEFAULT_OPACITY),
            anchor: Signal::new(Vec2::ZERO),
        }
    }
}

impl Polygon {
    /// Creates a new polygon at the given position with the specified vertices and fill color.
    pub fn new(position: Vec2, points: Vec<Vec2>, fill_color: Color) -> Self {
        Self::default()
            .with_position(position)
            .with_points(points)
            .with_fill(fill_color)
    }

    /// Sets the absolute position of the polygon.
    pub fn with_position(mut self, position: Vec2) -> Self {
        self.position = Signal::new(position);
        self
    }

    /// Sets the rotation of the polygon in radians.
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

    /// Sets the opacity of the polygon (0.0 to 1.0).
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = Signal::new(opacity);
        self
    }

    /// Sets the stroke paint and width for the border.
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

    /// Sets the list of vertices.
    pub fn with_points(mut self, points: Vec<Vec2>) -> Self {
        self.points = Signal::new(points);
        self
    }

    /// Sets the fill paint (color or gradient).
    pub fn with_fill(mut self, paint: impl Into<Paint>) -> Self {
        let p = paint.into();
        if let Paint::Solid(color) = p {
            self.fill_color = Signal::new(color);
        }
        self.fill_paint = Signal::new(Some(p));
        self
    }

    /// Creates a regular polygon with a given number of sides and radius.
    pub fn regular(sides: u32, radius: f32) -> Self {
        let mut points = Vec::new();
        for i in 0..sides {
            let angle =
                (i as f32 / sides as f32) * 2.0 * std::f32::consts::PI - std::f32::consts::PI / 2.0;
            points.push(Vec2::new(angle.cos() * radius, angle.sin() * radius));
        }
        Self::default().with_points(points)
    }
}

impl Node for Polygon {
    #[cfg(feature = "runtime")]
    fn render(&self, scene: &mut Scene, parent_transform: Affine, parent_opacity: f32) {
        let points = self.points.get();
        if points.len() < 2 {
            return;
        }

        let fill_color = self.fill_color.get();
        let stroke_color = self.stroke_color.get();
        let stroke_width = self.stroke_width.get();
        let opacity = self.opacity.get();

        let pos = self.position.get();
        let rot = self.rotation.get();
        let sc = self.scale.get();
        let anchor = self.anchor.get();

        // Calculate bounding box for centering and anchor
        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        for p in &points {
            min_x = min_x.min(p.x);
            min_y = min_y.min(p.y);
            max_x = max_x.max(p.x);
            max_y = max_y.max(p.y);
        }

        let size_vec = if min_x == f32::MAX {
            Vec2::ZERO
        } else {
            Vec2::new(max_x - min_x, max_y - min_y)
        };

        let center_offset = Vec2::new((min_x + max_x) * 0.5, (min_y + max_y) * 0.5);
        let anchor_offset = anchor * size_vec * 0.5;

        let local_transform = Affine::translate((pos.x as f64, pos.y as f64))
            * Affine::rotate(rot as f64)
            * Affine::scale_non_uniform(sc.x as f64, sc.y as f64)
            * Affine::translate((-anchor_offset.x as f64, -anchor_offset.y as f64))
            * Affine::translate((-center_offset.x as f64, -center_offset.y as f64));

        let combined_transform = parent_transform * local_transform;
        let combined_opacity = parent_opacity * opacity;

        // Construct path
        let mut path = BezPath::new();
        path.move_to((points[0].x as f64, points[0].y as f64));
        for p in points.iter().skip(1) {
            path.line_to((p.x as f64, p.y as f64));
        }
        path.close_path();

        // Fill
        let fill_brush = match self.fill_paint.get() {
            Some(paint) => paint.to_brush_with_opacity(combined_opacity),
            None => {
                let mut final_fill = fill_color;
                final_fill.a = (fill_color.a as f32 * combined_opacity).clamp(0.0, 255.0) as u8;
                Brush::Solid(final_fill)
            }
        };
        scene.fill(Fill::NonZero, combined_transform, &fill_brush, None, &path);

        // Stroke
        if stroke_width > 0.001 {
            let stroke_brush = match self.stroke_paint.get() {
                Some(paint) => paint.to_brush_with_opacity(combined_opacity),
                None => {
                    let mut final_stroke = stroke_color;
                    final_stroke.a =
                        (stroke_color.a as f32 * combined_opacity).clamp(0.0, 255.0) as u8;
                    Brush::Solid(final_stroke)
                }
            };
            scene.stroke(
                &Stroke::new(stroke_width as f64),
                combined_transform,
                &stroke_brush,
                None,
                &path,
            );
        }
    }

    fn update(&mut self, _dt: Duration) {}

    fn state_hash(&self) -> u64 {
        use crate::assets::hash::Hasher;
        let mut h = Hasher::new();
        h.update_u64(self.position.state_hash());
        h.update_u64(self.rotation.state_hash());
        h.update_u64(self.scale.state_hash());
        h.update_u64(self.points.state_hash());
        h.update_u64(self.fill_color.state_hash());
        h.update_u64(self.fill_paint.state_hash());
        h.update_u64(self.stroke_color.state_hash());
        h.update_u64(self.stroke_paint.state_hash());
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
        self.points.reset();
        self.fill_color.reset();
        self.fill_paint.reset();
        self.stroke_color.reset();
        self.stroke_paint.reset();
        self.stroke_width.reset();
        self.opacity.reset();
        self.anchor.reset();
    }
}
