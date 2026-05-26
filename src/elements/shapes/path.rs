#![allow(deprecated)]

use crate::core::animation::{Node, Paint, Signal};
use glam::Vec2;
use kurbo::{Affine, BezPath, Stroke};
use peniko::{Brush, Color};
use std::sync::Arc;
use std::time::Duration;
#[cfg(feature = "runtime")]
use vello::Scene;

const FLATTEN_TOLERANCE: f64 = 0.1;

/// Pre-processed path data for efficient rendering and sampling.
///
/// `PathData` flattens a `BezPath` into linear segments to calculate its total length
/// and support sampling points along the path for animations (e.g., following a path).
pub struct PathData {
    /// The original Vello bezier path.
    pub path: BezPath,
    /// Flattened segments as (position, distance_from_start).
    pub segments: Vec<(Vec2, f32)>,
    /// The total arc length of the flattened path.
    pub total_length: f32,
}

impl Default for PathData {
    fn default() -> Self {
        Self::new(BezPath::new())
    }
}

impl PathData {
    /// Creates a new `PathData` by flattening the provided `BezPath`.
    pub fn new(path: BezPath) -> Self {
        let mut segments = Vec::new();
        let mut total_length = 0.0;
        let mut last_point: Option<Vec2> = None;
        kurbo::flatten(&path, FLATTEN_TOLERANCE, |el| match el {
            kurbo::PathEl::MoveTo(p) => {
                let pt = Vec2::new(p.x as f32, p.y as f32);
                segments.push((pt, 0.0));
                last_point = Some(pt);
            }
            kurbo::PathEl::LineTo(p) => {
                if let Some(last) = last_point {
                    let pt = Vec2::new(p.x as f32, p.y as f32);
                    total_length += last.distance(pt);
                    segments.push((pt, total_length));
                    last_point = Some(pt);
                }
            }
            _ => {}
        });
        Self {
            path,
            segments,
            total_length,
        }
    }

    /// Samples a point along the path at a given normalized time `t` (0.0 to 1.0).
    pub fn sample(&self, t: f32) -> Vec2 {
        if self.segments.is_empty() {
            return Vec2::ZERO;
        }
        let target_len = t.clamp(0.0, 1.0) * self.total_length;
        let idx = match self
            .segments
            .binary_search_by(|&(_, len)| len.partial_cmp(&target_len).unwrap())
        {
            Ok(i) => i,
            Err(i) => i,
        };
        if idx == 0 {
            return self.segments[0].0;
        }
        if idx >= self.segments.len() {
            return self.segments.last().unwrap().0;
        }
        let (p1, l1) = self.segments[idx - 1];
        let (p2, l2) = self.segments[idx];
        let segment_len = l2 - l1;
        if segment_len < 0.0001 {
            return p2;
        }
        let t_segment = (target_len - l1) / segment_len;
        p1.lerp(p2, t_segment)
    }
}

/// A node that renders a complex vector path.
///
/// `PathNode` uses Vello's `BezPath` to represent arbitrary shapes or lines.
/// It is ideal for SVG-like paths or hand-drawn trajectories.
///
/// ### Example
/// ```rust
/// # use motion_canvas_rs::prelude::*;
/// let mut bez = BezPath::new();
/// bez.move_to((0.0, 0.0));
/// bez.quad_to((50.0, -100.0), (100.0, 0.0));
///
/// let path_node = PathNode::default()
///     .with_position(Vec2::new(640.0, 360.0))
///     .with_path(bez)
///     .with_stroke(Color::YELLOW, 2.0);
/// ```
#[derive(Clone)]
pub struct PathNode {
    /// The absolute position of the path's origin.
    pub position: Signal<Vec2>,
    /// Rotation in radians.
    pub rotation: Signal<f32>,
    /// Scaling factor for the path.
    pub scale: Signal<Vec2>,
    /// The processed path data.
    pub data: Arc<PathData>,
    /// The color of the path's stroke.
    /// **Deprecated**: prefer `stroke_paint` which supports both solid colors and gradients.
    #[deprecated(since = "0.2.3", note = "use stroke_paint instead")]
    pub stroke_color: Signal<Color>,
    /// The paint (color or gradient) used for the path's stroke.
    pub stroke_paint: Signal<Paint>,
    /// The width of the path's stroke.
    pub stroke_width: Signal<f32>,
    /// Opacity from 0.0 (transparent) to 1.0 (opaque).
    pub opacity: Signal<f32>,
    /// Blur radius signal.
    pub blur: Signal<f32>,
}

impl Default for PathNode {
    fn default() -> Self {
        Self {
            position: Signal::new(Vec2::ZERO),
            rotation: Signal::new(0.0),
            scale: Signal::new(Vec2::ONE),
            data: Arc::new(PathData::default()),
            stroke_color: Signal::new(Color::WHITE),
            stroke_paint: Signal::new(Paint::None),
            stroke_width: Signal::new(1.0),
            opacity: Signal::new(1.0),
            blur: Signal::new(crate::core::filters::DEFAULT_BLUR),
        }
    }
}

impl PathNode {
    /// Creates a new path node at the given position with the specified path, color, and width.
    pub fn new(position: Vec2, path: BezPath, color: Color, width: f32) -> Self {
        Self {
            position: Signal::new(position),
            rotation: Signal::new(0.0),
            scale: Signal::new(Vec2::ONE),
            data: Arc::new(PathData::new(path)),
            stroke_color: Signal::new(color),
            stroke_paint: Signal::new(Paint::None),
            stroke_width: Signal::new(width),
            opacity: Signal::new(1.0),
            blur: Signal::new(crate::core::filters::DEFAULT_BLUR),
        }
    }

    /// Sets the absolute position of the path.
    pub fn with_position(mut self, position: Vec2) -> Self {
        self.position = Signal::new(position);
        self
    }

    /// Sets the rotation of the path in radians.
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

    /// Sets the opacity of the path (0.0 to 1.0).
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = Signal::new(opacity);
        self
    }

    /// Updates the path geometry.
    pub fn with_path(mut self, path: BezPath) -> Self {
        self.data = Arc::new(PathData::new(path));
        self
    }

    /// Sets the stroke paint and width.
    pub fn with_stroke(mut self, paint: impl Into<Paint>, width: f32) -> Self {
        let p = paint.into();
        if let Paint::Solid(color) = p {
            self.stroke_color = Signal::new(color);
        }
        self.stroke_paint = Signal::new(p);
        self.stroke_width = Signal::new(width);
        self
    }
}

impl crate::core::filters::Blur for PathNode {
    fn with_blur(mut self, radius: f32) -> Self {
        self.blur = Signal::new(radius);
        self
    }
}

impl Node for PathNode {
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
                let stroke_color = self.stroke_color.get();
                let stroke_width = self.stroke_width.get();

                let pos = self.position.get();
                let rot = self.rotation.get();
                let sc = self.scale.get();

                let local_transform = Affine::translate((pos.x as f64, pos.y as f64))
                    * Affine::rotate(rot as f64)
                    * Affine::scale_non_uniform(sc.x as f64, sc.y as f64);

                let combined_transform = parent_transform * local_transform;

                let brush = match self.stroke_paint.get() {
                    Paint::None => {
                        let mut final_color = stroke_color;
                        final_color.a =
                            (stroke_color.a as f32 * target_opacity).clamp(0.0, 255.0) as u8;
                        Brush::Solid(final_color)
                    }
                    paint => paint.to_brush_with_opacity(target_opacity),
                };
                scene.stroke(
                    &Stroke::new(stroke_width as f64),
                    combined_transform,
                    &brush,
                    None,
                    &self.data.path,
                );
            },
        );
    }
    fn update(&mut self, _dt: Duration) {}
    fn state_hash(&self) -> u64 {
        use crate::assets::hash::Hasher;
        let mut h = Hasher::new();
        h.update_u64(self.position.state_hash());
        h.update_u64(self.rotation.state_hash());
        h.update_u64(self.scale.state_hash());
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
        self.rotation.reset();
        self.scale.reset();
        self.stroke_color.reset();
        self.stroke_paint.reset();
        self.stroke_width.reset();
        self.opacity.reset();
        self.blur.reset();
    }
}
