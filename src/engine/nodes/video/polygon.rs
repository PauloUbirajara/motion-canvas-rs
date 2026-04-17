use crate::engine::animation::{Node, Signal};
use glam::Vec2;
use std::time::Duration;
use vello::kurbo::{Affine, BezPath, Stroke};
use vello::peniko::{Brush, Color, Fill};
use vello::Scene;

const DEFAULT_FILL_COLOR: Color = Color::rgb8(9, 9, 11); // Zinc 950
const DEFAULT_STROKE_COLOR: Color = Color::rgba8(250, 250, 250, 25); // 10% Zinc 50
const DEFAULT_STROKE_WIDTH: f32 = 1.0;
const DEFAULT_OPACITY: f32 = 1.0;

#[derive(Clone)]
pub struct Polygon {
    pub position: Signal<Vec2>,
    pub rotation: Signal<f32>,
    pub scale: Signal<Vec2>,
    pub points: Signal<Vec<Vec2>>,
    pub fill_color: Signal<Color>,
    pub stroke_color: Signal<Color>,
    pub stroke_width: Signal<f32>,
    pub opacity: Signal<f32>,
}

impl Default for Polygon {
    fn default() -> Self {
        Self {
            position: Signal::new(Vec2::ZERO),
            rotation: Signal::new(0.0),
            scale: Signal::new(Vec2::ONE),
            points: Signal::new(Vec::new()),
            fill_color: Signal::new(DEFAULT_FILL_COLOR),
            stroke_color: Signal::new(DEFAULT_STROKE_COLOR),
            stroke_width: Signal::new(DEFAULT_STROKE_WIDTH),
            opacity: Signal::new(DEFAULT_OPACITY),
        }
    }
}

impl Polygon {
    pub fn new(position: Vec2, points: Vec<Vec2>, fill_color: Color) -> Self {
        Self::default()
            .with_position(position)
            .with_points(points)
            .with_fill(fill_color)
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

    pub fn with_stroke(mut self, color: Color, width: f32) -> Self {
        self.stroke_color = Signal::new(color);
        self.stroke_width = Signal::new(width);
        self
    }

    pub fn with_points(mut self, points: Vec<Vec2>) -> Self {
        self.points = Signal::new(points);
        self
    }

    pub fn with_fill(mut self, color: Color) -> Self {
        self.fill_color = Signal::new(color);
        self
    }

    /// Convenience method to create a regular polygon.
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

        let local_transform = Affine::translate((pos.x as f64, pos.y as f64))
            * Affine::rotate(rot as f64)
            * Affine::scale_non_uniform(sc.x as f64, sc.y as f64);

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
        let mut final_fill = fill_color;
        final_fill.a = (fill_color.a as f32 * combined_opacity).clamp(0.0, 255.0) as u8;
        scene.fill(
            Fill::NonZero,
            combined_transform,
            &Brush::Solid(final_fill),
            None,
            &path,
        );

        // Stroke
        if stroke_width > 0.001 {
            let mut final_stroke = stroke_color;
            final_stroke.a = (stroke_color.a as f32 * combined_opacity).clamp(0.0, 255.0) as u8;
            scene.stroke(
                &Stroke::new(stroke_width as f64),
                combined_transform,
                &Brush::Solid(final_stroke),
                None,
                &path,
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

        for p in self.points.get() {
            p.x.to_bits().hash(&mut s);
            p.y.to_bits().hash(&mut s);
        }

        let color = self.fill_color.get();
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
