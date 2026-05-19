#![cfg(feature = "math")]
#![allow(deprecated)]
use crate::core::animation::{Node, Paint, Signal};
use glam::Vec2;
use kurbo::{Affine, BezPath, Shape};
use peniko::{Brush, Color, Fill};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
#[cfg(feature = "runtime")]
use vello::Scene;

use std::sync::LazyLock;

static GLOBAL_MATH_CACHE: LazyLock<Mutex<HashMap<MathCacheKey, Arc<Vec<(Affine, BezPath)>>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

const DEFAULT_FONT_SIZE: f32 = 32.0;
const DEFAULT_COLOR: Color = Color::WHITE;
const DEFAULT_OPACITY: f32 = 1.0;

const TYPST_MATH_TEMPLATE: &str = r#"
#set text(size: {size}pt)
#show math.equation: set text(font: "{font}")
$ {equation} $
"#;

#[derive(Hash, Eq, PartialEq)]
struct MathCacheKey {
    equation: String,
    font_size_bits: u32,
}

/// A mathematical formula node powered by Typst.
///
/// `MathNode` allows you to render complex mathematical equations using Typst's
/// math syntax. It supports "magic move" style transitions between equations
/// via the `tex` animation.
///
/// ### Example
/// ```rust
/// # use motion_canvas_rs::prelude::*;
/// let math = MathNode::default()
///     .with_position(Vec2::new(640.0, 360.0))
///     .with_equation("E = m c^2")
///     .with_font_size(48.0)
///     .with_fill(Color::WHITE);
/// ```
pub struct MathNode {
    /// The absolute position of the formula's transformation origin.
    pub position: Signal<Vec2>,
    /// The rotation in radians.
    pub rotation: Signal<f32>,
    /// The scale factor.
    pub scale: Signal<Vec2>,
    /// The Typst-syntax mathematical equation string.
    pub equation: Signal<String>,
    /// The font size in points.
    pub font_size: Signal<f32>,
    /// The solid color used to fill the glyphs.
    /// **Deprecated**: prefer `fill_paint` which supports both solid colors and gradients.
    #[deprecated(since = "0.2.3", note = "use fill_paint instead")]
    pub fill_color: Signal<Color>,
    /// The paint (color or gradient) used to fill the math glyphs.
    pub fill_paint: Signal<Option<Paint>>,
    /// The overall opacity (0.0 to 1.0).
    pub opacity: Signal<f32>,
    /// Internal transition progress signal (0.0 to 1.0).
    pub transition_progress: Signal<f32>,
    /// The relative transformation origin (anchor).
    /// (-1, -1) is top-left, (0, 0) is center, (1, 1) is bottom-right.
    ///
    /// NOTE: During transitions, MathNode uses a union of the previous and current
    /// bounding boxes to ensure the anchor point remains stable.
    pub anchor: Signal<Vec2>,
    cache: Arc<Mutex<Option<Arc<Vec<(Affine, BezPath)>>>>>,
    prev_cache: Arc<Mutex<Option<Arc<Vec<(Affine, BezPath)>>>>>,
}

impl Default for MathNode {
    fn default() -> Self {
        Self {
            position: Signal::new(Vec2::ZERO),
            rotation: Signal::new(0.0),
            scale: Signal::new(Vec2::ONE),
            equation: Signal::new("".to_string()),
            font_size: Signal::new(DEFAULT_FONT_SIZE),
            fill_color: Signal::new(DEFAULT_COLOR),
            fill_paint: Signal::new(None),
            opacity: Signal::new(DEFAULT_OPACITY),
            transition_progress: Signal::new(1.0),
            anchor: Signal::new(Vec2::ZERO),
            cache: Arc::new(Mutex::new(None)),
            prev_cache: Arc::new(Mutex::new(None)),
        }
    }
}

impl Clone for MathNode {
    fn clone(&self) -> Self {
        Self {
            position: self.position.clone(),
            rotation: self.rotation.clone(),
            scale: self.scale.clone(),
            equation: self.equation.clone(),
            font_size: self.font_size.clone(),
            fill_color: self.fill_color.clone(),
            fill_paint: self.fill_paint.clone(),
            opacity: self.opacity.clone(),
            transition_progress: self.transition_progress.clone(),
            anchor: self.anchor.clone(),
            cache: self.cache.clone(),
            prev_cache: self.prev_cache.clone(),
        }
    }
}

impl MathNode {
    /// Creates a new math node at the given position with an initial equation, size, and color.
    pub fn new(pos: Vec2, equation: &str, size: f32, color: Color) -> Self {
        Self::default()
            .with_position(pos)
            .with_equation(equation)
            .with_font_size(size)
            .with_fill(color)
    }

    /// Sets the absolute position of the math node.
    pub fn with_position(mut self, position: Vec2) -> Self {
        self.position = Signal::new(position);
        self
    }

    /// Sets the rotation in radians.
    pub fn with_rotation(mut self, angle: f32) -> Self {
        self.rotation = Signal::new(angle);
        self
    }

    /// Sets a uniform scale factor.
    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = Signal::new(Vec2::splat(scale));
        self
    }

    /// Sets non-uniform scaling for X and Y axes.
    pub fn with_scale_xy(mut self, scale: Vec2) -> Self {
        self.scale = Signal::new(scale);
        self
    }

    /// Sets the overall opacity (0.0 to 1.0).
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = Signal::new(opacity);
        self
    }

    /// Returns an animation that transitions the current equation to a new one.
    pub fn tex(
        &self,
        equation: &str,
        duration: Duration,
    ) -> Box<dyn crate::core::animation::Animation> {
        Box::new(MathTransition {
            node: self.clone(),
            target_eq: equation.to_string(),
            duration,
            tween: None,
        })
    }

    /// Sets the Typst mathematical equation string.
    pub fn with_equation(mut self, equation: &str) -> Self {
        self.equation = Signal::new(equation.to_string());
        self
    }

    /// Sets the font size in points.
    pub fn with_font_size(mut self, size: f32) -> Self {
        self.font_size = Signal::new(size);
        self
    }

    /// Sets the fill paint (color or gradient) for the glyphs.
    pub fn with_fill(mut self, paint: impl Into<Paint>) -> Self {
        let p = paint.into();
        if let Paint::Solid(color) = p {
            self.fill_color = Signal::new(color);
        }
        self.fill_paint = Signal::new(Some(p));
        self
    }

    /// Sets the relative transformation origin (anchor).
    /// (-1, -1) is top-left, (0, 0) is center, (1, 1) is bottom-right.
    pub fn with_anchor(mut self, anchor: Vec2) -> Self {
        self.anchor = Signal::new(anchor);
        self
    }

    /// Triggers a cross-fade transition to a new equation.
    pub fn start_transition(&self, new_eq: &str) {
        let prev_eq = self.equation.get();
        if prev_eq == new_eq {
            return;
        }

        self.rebuild_if_needed();

        // 1. Move current cache to prev_cache
        {
            let cache = self.cache.lock().unwrap();
            let mut prev = self.prev_cache.lock().unwrap();
            *prev = cache.clone();
        }

        // 2. Start transition
        self.transition_progress.set(0.0);
        self.equation.set(new_eq.to_string());

        self.rebuild_if_needed();
    }

    fn rebuild_if_needed(&self) {
        let eq = self.equation.get();
        let size = self.font_size.get();

        let key = MathCacheKey {
            equation: eq.clone(),
            font_size_bits: size.to_bits(),
        };

        // 1. Check global cache
        let mut global = GLOBAL_MATH_CACHE.lock().unwrap();
        if let Some(paths) = global.get(&key) {
            let mut local = self.cache.lock().unwrap();
            *local = Some(paths.clone());
            return;
        }

        // 3. Compile
        let mut paths_with_color = Vec::new();
        let (font_name, _) = crate::assets::font_manager::FontManager::get_math_font();
        let typst_code = TYPST_MATH_TEMPLATE
            .replace("{size}", &size.to_string())
            .replace("{font}", &font_name)
            .replace("{equation}", &eq);
        let world = crate::assets::typst_support::TypstWorld::new(&typst_code);
        let output = typst::compile::<typst::layout::PagedDocument>(&world).output;

        match output {
            Ok(document) => {
                for page in document.pages {
                    crate::assets::typst_support::collect_paths(
                        &page.frame,
                        Affine::IDENTITY,
                        &mut paths_with_color,
                    );
                }
            }
            Err(e) => println!("Typst compilation failed: {:?}", e),
        }

        let paths: Arc<Vec<(Affine, BezPath)>> = Arc::new(
            paths_with_color
                .into_iter()
                .map(|(a, _, p)| (a, p))
                .collect(),
        );
        global.insert(key, paths.clone());

        let mut local = self.cache.lock().unwrap();
        *local = Some(paths);
    }
}

struct MathTransition {
    node: MathNode,
    target_eq: String,
    duration: Duration,
    tween: Option<crate::core::animation::SignalTween<f32>>,
}

impl crate::core::animation::Animation for MathTransition {
    fn update(&mut self, dt: Duration) -> (bool, Duration) {
        if self.tween.is_none() {
            self.node.start_transition(&self.target_eq);
            self.tween = Some(self.node.transition_progress.to(1.0, self.duration));
        }
        self.tween.as_mut().unwrap().update(dt)
    }
    fn duration(&self) -> Duration {
        self.duration
    }

    fn reset(&mut self) {
        self.tween = None;
    }
}

impl Node for MathNode {
    #[cfg(feature = "runtime")]
    fn render(&self, scene: &mut Scene, parent_transform: Affine, parent_opacity: f32) {
        let color = self.fill_color.get();

        self.rebuild_if_needed();

        let cache_guard = self.cache.lock().unwrap();
        let progress = self.transition_progress.get();
        let base_opacity = self.opacity.get();

        let pos = self.position.get();
        let rot = self.rotation.get();
        let sc = self.scale.get();
        let anchor = self.anchor.get();

        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;

        if let Some(c) = cache_guard.as_ref() {
            for (glyph_transform, pb) in c.as_ref() {
                let bounds = pb.bounding_box();
                let p0 = *glyph_transform * vello::kurbo::Point::new(bounds.x0, bounds.y0);
                let p1 = *glyph_transform * vello::kurbo::Point::new(bounds.x1, bounds.y1);
                min_x = min_x.min(p0.x).min(p1.x);
                min_y = min_y.min(p0.y).min(p1.y);
                max_x = max_x.max(p0.x).max(p1.x);
                max_y = max_y.max(p0.y).max(p1.y);
            }
        }

        // Use the union of current and previous bounding boxes during transition for absolute stability
        if progress < 1.0 {
            let prev_cache = self.prev_cache.lock().unwrap();
            if let Some(prev) = prev_cache.as_ref() {
                for (glyph_transform, pb) in prev.as_ref() {
                    let bounds = pb.bounding_box();
                    let p0 = *glyph_transform * vello::kurbo::Point::new(bounds.x0, bounds.y0);
                    let p1 = *glyph_transform * vello::kurbo::Point::new(bounds.x1, bounds.y1);
                    min_x = min_x.min(p0.x).min(p1.x);
                    min_y = min_y.min(p0.y).min(p1.y);
                    max_x = max_x.max(p0.x).max(p1.x);
                    max_y = max_y.max(p0.y).max(p1.y);
                }
            }
        }

        let size_vec = if min_x == f64::MAX {
            Vec2::ZERO
        } else {
            Vec2::new((max_x - min_x) as f32, (max_y - min_y) as f32)
        };
        let center_offset = if min_x == f64::MAX {
            Vec2::ZERO
        } else {
            Vec2::new((min_x + max_x) as f32 * 0.5, (min_y + max_y) as f32 * 0.5)
        };

        let anchor_offset = anchor * size_vec * 0.5;

        let local_transform = Affine::translate((pos.x as f64, pos.y as f64))
            * Affine::rotate(rot as f64)
            * Affine::scale_non_uniform(sc.x as f64, sc.y as f64)
            * Affine::translate((-anchor_offset.x as f64, -anchor_offset.y as f64))
            * Affine::translate((-center_offset.x as f64, -center_offset.y as f64));

        let root_transform = parent_transform * local_transform;

        // 1. Draw previous equation if transitioning
        if progress < 1.0 {
            let prev_cache = self.prev_cache.lock().unwrap();
            if let Some(prev) = prev_cache.as_ref() {
                let prev_opacity = base_opacity * (1.0 - progress) * parent_opacity;
                let brush = match self.fill_paint.get() {
                    Some(paint) => paint.to_brush_with_opacity(prev_opacity),
                    None => {
                        let mut prev_color = color;
                        prev_color.a = (color.a as f32 * prev_opacity).clamp(0.0, 255.0) as u8;
                        Brush::Solid(prev_color)
                    }
                };
                for (local_transform, pb) in prev.as_ref() {
                    scene.fill(
                        Fill::NonZero,
                        root_transform * *local_transform,
                        &brush,
                        None,
                        pb,
                    );
                }
            }
        }

        if let Some(c) = cache_guard.as_ref() {
            let current_alpha = if progress < 1.0 {
                base_opacity * progress
            } else {
                base_opacity
            };
            let current_opacity = current_alpha * parent_opacity;
            let brush = match self.fill_paint.get() {
                Some(paint) => paint.to_brush_with_opacity(current_opacity),
                None => {
                    let mut current_color = color;
                    current_color.a = (color.a as f32 * current_opacity).clamp(0.0, 255.0) as u8;
                    Brush::Solid(current_color)
                }
            };

            for (local_transform, pb) in c.as_ref() {
                scene.fill(
                    Fill::NonZero,
                    root_transform * *local_transform,
                    &brush,
                    None,
                    pb,
                );
            }
        }
    }
    fn update(&mut self, _dt: Duration) {}
    fn state_hash(&self) -> u64 {
        use crate::assets::hash::Hasher;
        let mut h = Hasher::new();
        h.update_u64(self.position.state_hash());
        h.update_u64(self.rotation.state_hash());
        h.update_u64(self.scale.state_hash());
        h.update_u64(self.equation.state_hash());
        h.update_u64(self.font_size.state_hash());
        h.update_u64(self.fill_color.state_hash());
        h.update_u64(self.fill_paint.state_hash());
        h.update_u64(self.opacity.state_hash());
        h.update_u64(self.transition_progress.state_hash());
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
        self.equation.reset();
        self.font_size.reset();
        self.fill_color.reset();
        self.fill_paint.reset();
        self.opacity.reset();
        self.transition_progress.reset();
        self.anchor.reset();
        *self.cache.lock().unwrap() = None;
        *self.prev_cache.lock().unwrap() = None;
    }
}
