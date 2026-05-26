#![allow(deprecated)]

use crate::core::animation::{Node, Paint, Signal};
use glam::Vec2;
use kurbo::{Affine, RoundedRect as KurboRoundedRect};
use peniko::{Brush, Color, Fill};
use std::time::Duration;
#[cfg(feature = "runtime")]
use vello::Scene;

const DEFAULT_SIZE: Vec2 = Vec2::new(100.0, 100.0);
const DEFAULT_COLOR: Color = Color::rgb8(9, 9, 11);
const DEFAULT_RADIUS: f32 = 12.0;
const DEFAULT_STROKE_COLOR: Color = Color::rgba8(250, 250, 250, 25);
const DEFAULT_STROKE_WIDTH: f32 = 1.0;
const DEFAULT_OPACITY: f32 = 1.0;

/// A rectangular shape with support for rounded corners, strokes, and fills.
///
/// `Rect` is one of the most versatile basic shapes, commonly used for backgrounds,
/// buttons, or as a base for complex UI elements.
///
/// ### Example
/// ```rust
/// # use motion_canvas_rs::prelude::*;
/// let rect = Rect::default()
///     .with_position(Vec2::new(640.0, 360.0))
///     .with_size(Vec2::new(200.0, 100.0))
///     .with_fill(Color::RED)
///     .with_radius(20.0)
///     .with_stroke(Color::WHITE, 2.0);
/// ```
#[derive(Clone)]
pub struct Rect {
    /// The absolute position of the rectangle's center (before anchor adjustment).
    pub position: Signal<Vec2>,
    /// Rotation in radians.
    pub rotation: Signal<f32>,
    /// Scaling factor for the rectangle.
    pub scale: Signal<Vec2>,
    /// The width and height of the rectangle.
    pub size: Signal<Vec2>,
    /// The solid color used to fill the rectangle.
    /// **Deprecated**: prefer `fill_paint` which supports both solid colors and gradients.
    #[deprecated(since = "0.2.3", note = "use fill_paint instead")]
    pub fill_color: Signal<Color>,
    /// The paint (color or gradient) used to fill the rectangle.
    pub fill_paint: Signal<Paint>,
    /// The color of the border stroke.
    /// **Deprecated**: prefer `stroke_paint` which supports both solid colors and gradients.
    #[deprecated(since = "0.2.3", note = "use stroke_paint instead")]
    pub stroke_color: Signal<Color>,
    /// The paint (color or gradient) used for the border stroke.
    pub stroke_paint: Signal<Paint>,
    /// The width of the border stroke.
    pub stroke_width: Signal<f32>,
    /// The corner radius for rounded rectangles.
    pub radius: Signal<f32>,
    /// Opacity from 0.0 (transparent) to 1.0 (opaque).
    pub opacity: Signal<f32>,
    /// The relative transformation origin. (-1,-1) is top-left, (0,0) is center, (1,1) is bottom-right.
    pub anchor: Signal<Vec2>,
    /// Blur radius signal.
    pub blur: Signal<f32>,
}

impl Default for Rect {
    fn default() -> Self {
        Self {
            position: Signal::new(Vec2::ZERO),
            rotation: Signal::new(0.0),
            scale: Signal::new(Vec2::ONE),
            size: Signal::new(DEFAULT_SIZE),
            fill_color: Signal::new(DEFAULT_COLOR),
            fill_paint: Signal::new(Paint::None),
            stroke_color: Signal::new(DEFAULT_STROKE_COLOR),
            stroke_paint: Signal::new(Paint::None),
            stroke_width: Signal::new(DEFAULT_STROKE_WIDTH),
            radius: Signal::new(DEFAULT_RADIUS),
            opacity: Signal::new(DEFAULT_OPACITY),
            anchor: Signal::new(Vec2::ZERO),
            blur: Signal::new(crate::core::filters::DEFAULT_BLUR),
        }
    }
}

impl Rect {
    /// Creates a new rectangle with the given position, size, and fill color.
    pub fn new(position: Vec2, size: Vec2, color: Color) -> Self {
        Self::default()
            .with_position(position)
            .with_size(size)
            .with_fill(color)
    }

    /// Sets the absolute position of the rectangle.
    pub fn with_position(mut self, position: Vec2) -> Self {
        self.position = Signal::new(position);
        self
    }

    /// Sets the rotation of the rectangle in radians.
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

    /// Sets the opacity of the rectangle (0.0 to 1.0).
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = Signal::new(opacity);
        self
    }

    /// Sets the corner radius for rounded corners.
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = Signal::new(radius);
        self
    }

    /// Sets the dimensions (width, height) of the rectangle.
    pub fn with_size(mut self, size: Vec2) -> Self {
        self.size = Signal::new(size);
        self
    }

    /// Sets the fill paint (color or gradient).
    pub fn with_fill(mut self, paint: impl Into<Paint>) -> Self {
        let p = paint.into();
        if let Paint::Solid(color) = p {
            self.fill_color = Signal::new(color);
        }
        self.fill_paint = Signal::new(p);
        self
    }

    /// Sets the stroke paint and width for the border.
    pub fn with_stroke(mut self, paint: impl Into<Paint>, width: f32) -> Self {
        let p = paint.into();
        if let Paint::Solid(color) = p {
            self.stroke_color = Signal::new(color);
        }
        self.stroke_paint = Signal::new(p);
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

impl crate::core::filters::Blur for Rect {
    fn with_blur(mut self, radius: f32) -> Self {
        self.blur = Signal::new(radius);
        self
    }
}

impl Node for Rect {
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
                let size = self.size.get();
                let fill_color_val = self.fill_color.get();
                let stroke_color = self.stroke_color.get();
                let stroke_width = self.stroke_width.get();
                let radius = self.radius.get();
                let pos = self.position.get();
                let rot = self.rotation.get();
                let sc = self.scale.get();
                let anchor = self.anchor.get();

                let anchor_offset = anchor * size * 0.5;

                let local_transform = Affine::translate((pos.x as f64, pos.y as f64))
                    * Affine::rotate(rot as f64)
                    * Affine::scale_non_uniform(sc.x as f64, sc.y as f64)
                    * Affine::translate((-anchor_offset.x as f64, -anchor_offset.y as f64));

                let combined_transform = parent_transform * local_transform;

                let rect = KurboRoundedRect::new(
                    -size.x as f64 / 2.0,
                    -size.y as f64 / 2.0,
                    size.x as f64 / 2.0,
                    size.y as f64 / 2.0,
                    radius as f64,
                );

                // Fill
                let fill_brush = match self.fill_paint.get() {
                    Paint::None => {
                        let mut final_color = fill_color_val;
                        final_color.a =
                            (fill_color_val.a as f32 * target_opacity).clamp(0.0, 255.0) as u8;
                        Brush::Solid(final_color)
                    }
                    paint => paint.to_brush_with_opacity(target_opacity),
                };
                scene.fill(Fill::NonZero, combined_transform, &fill_brush, None, &rect);

                // Stroke
                if stroke_width > 0.001 {
                    let stroke_brush = match self.stroke_paint.get() {
                        Paint::None => {
                            let mut final_stroke = stroke_color;
                            final_stroke.a =
                                (stroke_color.a as f32 * target_opacity).clamp(0.0, 255.0) as u8;
                            Brush::Solid(final_stroke)
                        }
                        paint => paint.to_brush_with_opacity(target_opacity),
                    };
                    scene.stroke(
                        &kurbo::Stroke::new(stroke_width as f64),
                        combined_transform,
                        &stroke_brush,
                        None,
                        &rect,
                    );
                }
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
        h.update_u64(self.size.state_hash());
        h.update_u64(self.radius.state_hash());
        h.update_u64(self.fill_color.state_hash());
        h.update_u64(self.fill_paint.state_hash());
        h.update_u64(self.stroke_color.state_hash());
        h.update_u64(self.stroke_paint.state_hash());
        h.update_u64(self.stroke_width.state_hash());
        h.update_u64(self.opacity.state_hash());
        h.update_u64(self.anchor.state_hash());
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
        self.size.reset();
        self.radius.reset();
        self.fill_color.reset();
        self.fill_paint.reset();
        self.stroke_color.reset();
        self.stroke_paint.reset();
        self.stroke_width.reset();
        self.opacity.reset();
        self.anchor.reset();
        self.blur.reset();
    }
}
