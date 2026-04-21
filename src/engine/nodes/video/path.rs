use crate::engine::animation::{Node, Signal};
use glam::Vec2;
use std::sync::Arc;
use std::time::Duration;
use vello::kurbo::{Affine, BezPath, Stroke};
use vello::peniko::{Brush, Color};
use vello::Scene;

const FLATTEN_TOLERANCE: f64 = 0.1;

pub struct PathData {
    pub path: BezPath,
    pub segments: Vec<(Vec2, f32)>,
    pub total_length: f32,
}

impl Default for PathData {
    fn default() -> Self {
        Self::new(BezPath::new())
    }
}

impl PathData {
    pub fn new(path: BezPath) -> Self {
        let mut segments = Vec::new();
        let mut total_length = 0.0;
        let mut last_point: Option<Vec2> = None;
        vello::kurbo::flatten(&path, FLATTEN_TOLERANCE, |el| match el {
            vello::kurbo::PathEl::MoveTo(p) => {
                let pt = Vec2::new(p.x as f32, p.y as f32);
                segments.push((pt, 0.0));
                last_point = Some(pt);
            }
            vello::kurbo::PathEl::LineTo(p) => {
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

impl Default for PathNode {
    fn default() -> Self {
        Self {
            position: Signal::new(Vec2::ZERO),
            rotation: Signal::new(0.0),
            scale: Signal::new(Vec2::ONE),
            data: Arc::new(PathData::default()),
            stroke_color: Signal::new(Color::WHITE),
            stroke_width: Signal::new(1.0),
            opacity: Signal::new(1.0),
        }
    }
}

#[derive(Clone)]
pub struct PathNode {
    pub position: Signal<Vec2>,
    pub rotation: Signal<f32>,
    pub scale: Signal<Vec2>,
    pub data: Arc<PathData>,
    pub stroke_color: Signal<Color>,
    pub stroke_width: Signal<f32>,
    pub opacity: Signal<f32>,
}

impl PathNode {
    pub fn new(position: Vec2, path: BezPath, color: Color, width: f32) -> Self {
        Self {
            position: Signal::new(position),
            rotation: Signal::new(0.0),
            scale: Signal::new(Vec2::ONE),
            data: Arc::new(PathData::new(path)),
            stroke_color: Signal::new(color),
            stroke_width: Signal::new(width),
            opacity: Signal::new(1.0),
        }
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

    pub fn with_path(mut self, path: BezPath) -> Self {
        self.data = Arc::new(PathData::new(path));
        self
    }

    pub fn with_stroke(mut self, color: Color, width: f32) -> Self {
        self.stroke_color = Signal::new(color);
        self.stroke_width = Signal::new(width);
        self
    }

    #[deprecated(note = "use with_stroke instead")]
    pub fn with_color(self, color: Color) -> Self {
        let width = self.stroke_width.get();
        self.with_stroke(color, width)
    }
}

impl Node for PathNode {
    fn render(&self, scene: &mut Scene, parent_transform: Affine, parent_opacity: f32) {
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

        let mut final_color = stroke_color;
        final_color.a = (stroke_color.a as f32 * combined_opacity).clamp(0.0, 255.0) as u8;

        let brush = Brush::Solid(final_color);
        scene.stroke(
            &Stroke::new(stroke_width as f64),
            combined_transform,
            &brush,
            None,
            &self.data.path,
        );
    }
    fn update(&mut self, _dt: Duration) {}
    fn state_hash(&self) -> u64 {
        use crate::engine::util::hash::Hasher;
        let mut h = Hasher::new();
        h.update_u64(self.position.state_hash());
        h.update_u64(self.rotation.state_hash());
        h.update_u64(self.scale.state_hash());
        h.update_u64(self.stroke_color.state_hash());
        h.update_u64(self.stroke_width.state_hash());
        h.update_u64(self.opacity.state_hash());
        // For PathData, we could hash segments, but currently it's static Arc.
        // If data changes, we should include it.
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
        self.stroke_width.reset();
        self.opacity.reset();
    }
}
